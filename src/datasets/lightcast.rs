use std::path::Path;

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

// ── Auth types ──────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

// ── Raw API response types ──────────────────────────────────────────────────

#[derive(Deserialize)]
struct SkillsResponse {
    data: Vec<RawSkill>,
}

#[derive(Deserialize)]
struct RawSkill {
    id: String,
    name: String,
    #[serde(rename = "type")]
    skill_type: Option<RawRef>,
    category: Option<RawRef>,
    subcategory: Option<RawRef>,
}

#[derive(Deserialize)]
struct RawRef {
    id: String,
    name: String,
}

// ── Processed output types ──────────────────────────────────────────────────

#[derive(Serialize)]
struct ProcessedSkill {
    id: String,
    name: String,
    #[serde(rename = "type")]
    skill_type: Option<RefOut>,
    category: Option<RefOut>,
    subcategory: Option<RefOut>,
}

#[derive(Serialize)]
struct RefOut {
    id: String,
    name: String,
}

// ── Download ────────────────────────────────────────────────────────────────

/// Fetch all Lightcast skills via their OAuth2 API and cache the raw response.
pub async fn download(data_dir: &Path) -> anyhow::Result<()> {
    dotenvy::dotenv().ok(); // load .env if present

    let client_id = std::env::var("LIGHTCAST_CLIENT_ID")
        .context("Missing LIGHTCAST_CLIENT_ID in environment / .env")?;
    let client_secret = std::env::var("LIGHTCAST_CLIENT_SECRET")
        .context("Missing LIGHTCAST_CLIENT_SECRET in environment / .env")?;

    let client = reqwest::Client::new();

    // 1. OAuth2 client-credentials token
    let token_resp = client
        .post("https://auth.emsicloud.com/connect/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!(
            "client_id={}&client_secret={}&grant_type=client_credentials&scope=emsi_open",
            client_id, client_secret
        ))
        .send()
        .await
        .context("Failed to request Lightcast auth token")?;

    if !token_resp.status().is_success() {
        let status = token_resp.status();
        let body = token_resp.text().await.unwrap_or_default();
        bail!("Lightcast auth failed ({}): {}", status, body);
    }

    let token: TokenResponse = token_resp
        .json()
        .await
        .context("Failed to parse Lightcast token response")?;

    // 2. Fetch all skills
    let skills_resp = client
        .get("https://emsiservices.com/skills/versions/latest/skills")
        .bearer_auth(&token.access_token)
        .send()
        .await
        .context("Failed to fetch Lightcast skills")?;

    if !skills_resp.status().is_success() {
        let status = skills_resp.status();
        let body = skills_resp.text().await.unwrap_or_default();
        bail!("Lightcast skills request failed ({}): {}", status, body);
    }

    let body = skills_resp
        .bytes()
        .await
        .context("Failed to read Lightcast skills response body")?;

    // 3. Cache raw response
    let raw_dir = data_dir.join("datasets/raw/lightcast");
    std::fs::create_dir_all(&raw_dir)
        .context("Failed to create datasets/raw/lightcast directory")?;

    let raw_path = raw_dir.join("skills-response.json");
    std::fs::write(&raw_path, &body)
        .with_context(|| format!("Failed to write {}", raw_path.display()))?;

    println!(
        "Lightcast: cached {} bytes to {}",
        body.len(),
        raw_path.display()
    );

    Ok(())
}

// ── Process ─────────────────────────────────────────────────────────────────

/// Read the cached Lightcast response and produce normalised output.
pub fn process(data_dir: &Path) -> anyhow::Result<()> {
    let raw_path = data_dir.join("datasets/raw/lightcast/skills-response.json");
    let raw_bytes = std::fs::read(&raw_path)
        .with_context(|| format!("Cannot read {}. Run download first.", raw_path.display()))?;

    let response: SkillsResponse = serde_json::from_slice(&raw_bytes)
        .context("Failed to parse cached Lightcast skills JSON")?;

    let processed: Vec<ProcessedSkill> = response
        .data
        .into_iter()
        .map(|s| ProcessedSkill {
            id: s.id,
            name: s.name,
            skill_type: s.skill_type.map(|r| RefOut { id: r.id, name: r.name }),
            category: s.category.map(|r| RefOut { id: r.id, name: r.name }),
            subcategory: s.subcategory.map(|r| RefOut { id: r.id, name: r.name }),
        })
        .collect();

    let out_dir = data_dir.join("datasets/processed");
    std::fs::create_dir_all(&out_dir)
        .context("Failed to create datasets/processed directory")?;

    let out_path = out_dir.join("lightcast-skills.json");
    let file = std::fs::File::create(&out_path)
        .with_context(|| format!("Failed to create {}", out_path.display()))?;
    serde_json::to_writer_pretty(file, &processed)
        .context("Failed to write processed Lightcast skills")?;

    println!(
        "Lightcast: wrote {} skills to {}",
        processed.len(),
        out_path.display()
    );

    Ok(())
}
