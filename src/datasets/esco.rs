// ESCO — European Skills, Competences, Qualifications and Occupations
//
// License: The ESCO classification is available under the European Commission's
// reuse policy (Commission Decision 2011/833/EU). If used in production:
//
//   For services/tools/applications integrating ESCO:
//     "This service uses the ESCO classification of the European Commission."
//
//   For documents such as studies, analysis, or reports:
//     "This publication uses the ESCO classification of the European Commission."

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Public output types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct EscoSkill {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub skill_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_uri: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alt_labels: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub broader: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub narrower: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EscoSkillGroup {
    pub uri: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub level: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uri: Option<String>,
}

// ---------------------------------------------------------------------------
// Download
// ---------------------------------------------------------------------------

/// The ESCO bulk download API (ec.europa.eu/esco/api/bulk) was decommissioned.
/// The new portal requires interactive download via email. This function prints
/// instructions for manual download and checks if data is already in place.
pub async fn download(data_dir: &Path) -> Result<()> {
    let raw_dir = data_dir.join("datasets/raw/esco");
    fs::create_dir_all(&raw_dir).context("creating raw esco directory")?;

    // Check if data already exists
    let skills_csv = raw_dir.join("skills_en.csv");
    if skills_csv.exists() {
        eprintln!("ESCO data already present at {}", raw_dir.display());
        return Ok(());
    }

    eprintln!("ESCO requires manual download (the bulk API was decommissioned).");
    eprintln!();
    eprintln!("Steps:");
    eprintln!("  1. Go to https://esco.ec.europa.eu/en/use-esco/download");
    eprintln!("  2. Version:  ESCO dataset - v1.2.1");
    eprintln!("     Content:  Classification");
    eprintln!("     File type: csv");
    eprintln!("     Language:  en");
    eprintln!("  3. Click 'Add to your package', then download");
    eprintln!("  4. Extract CSVs to: {}", raw_dir.display());
    eprintln!();
    eprintln!("Required files:");
    eprintln!("  - skills_en.csv");
    eprintln!("  - broaderRelationsSkillPillar.csv");
    eprintln!();
    eprintln!("Then run: cargo run -- process esco");

    Ok(())
}

// ---------------------------------------------------------------------------
// Process
// ---------------------------------------------------------------------------

/// Process the extracted ESCO CSV files into structured JSON.
pub fn process(data_dir: &Path) -> Result<()> {
    let raw_dir = data_dir.join("datasets/raw/esco");
    let out_dir = data_dir.join("datasets/processed");
    fs::create_dir_all(&out_dir).context("creating processed directory")?;

    // 1. Load skills from skills_en.csv
    let mut skills_map = load_skills(&raw_dir)?;
    eprintln!("  loaded {} skills", skills_map.len());

    // 2. Load broader/narrower relations
    let (broader, narrower) = load_broader_relations(&raw_dir)?;
    eprintln!(
        "  loaded {} broader relations",
        broader.values().map(|v| v.len()).sum::<usize>()
    );

    // 3. Attach relations to skills
    for (uri, skill) in skills_map.iter_mut() {
        if let Some(b) = broader.get(uri.as_str()) {
            skill.broader = b.clone();
        }
        if let Some(n) = narrower.get(uri.as_str()) {
            skill.narrower = n.clone();
        }
    }

    // 4. Process skill groups hierarchy
    let skill_groups = load_skill_groups(&raw_dir)?;
    eprintln!("  loaded {} skill groups", skill_groups.len());

    // 5. Assign group_uri to skills from broaderRelationsSkillPillar
    let skill_to_group = load_skill_group_assignments(&raw_dir)?;
    let mut assigned = 0usize;
    for (uri, skill) in skills_map.iter_mut() {
        if let Some(group_uri) = skill_to_group.get(uri.as_str()) {
            skill.group_uri = Some(group_uri.clone());
            assigned += 1;
        }
    }
    eprintln!("  assigned {} of {} skills to groups", assigned, skills_map.len());

    // 6. Collect and sort
    let mut skills: Vec<EscoSkill> = skills_map.into_values().collect();
    skills.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // 7. Write skills output
    let out_path = out_dir.join("esco-skills.json");
    let json = serde_json::to_string_pretty(&skills).context("serializing ESCO skills JSON")?;
    fs::write(&out_path, json).with_context(|| format!("writing {}", out_path.display()))?;
    eprintln!("  wrote {} skills to {}", skills.len(), out_path.display());

    // 8. Write skill groups output
    let groups_path = out_dir.join("esco-skill-groups.json");
    let groups_json =
        serde_json::to_string_pretty(&skill_groups).context("serializing skill groups JSON")?;
    fs::write(&groups_path, groups_json)
        .with_context(|| format!("writing {}", groups_path.display()))?;
    eprintln!(
        "  wrote {} skill groups to {}",
        skill_groups.len(),
        groups_path.display()
    );

    eprintln!("ESCO processing complete → {}", out_dir.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn col_index(headers: &csv::StringRecord, name: &str) -> Result<usize> {
    headers
        .iter()
        .position(|h| h.trim() == name)
        .with_context(|| format!("column '{}' not found in headers: {:?}", name, headers))
}

/// Load skills from skills_en.csv.
/// Columns vary by ESCO version; we look for: conceptUri, preferredLabel, description,
/// skillType, altLabels.
fn load_skills(raw_dir: &Path) -> Result<HashMap<String, EscoSkill>> {
    let path = find_skills_csv(raw_dir)?;
    let mut rdr = csv::ReaderBuilder::new()
        .from_path(&path)
        .with_context(|| format!("opening {}", path.display()))?;

    let headers = rdr.headers().context("reading skills CSV headers")?.clone();
    let uri_idx = col_index(&headers, "conceptUri")?;
    let label_idx = col_index(&headers, "preferredLabel")?;
    let desc_idx = col_index(&headers, "description")?;
    let type_idx = col_index(&headers, "skillType")?;
    // altLabels may not exist in all versions — optional
    let alt_idx = headers.iter().position(|h| h.trim() == "altLabels");

    let mut map = HashMap::new();
    for result in rdr.records() {
        let record = result.context("reading skills CSV row")?;
        let uri = record.get(uri_idx).unwrap_or("").trim().to_string();
        if uri.is_empty() {
            continue;
        }

        let name = record.get(label_idx).unwrap_or("").trim().to_string();
        let description = record.get(desc_idx).unwrap_or("").trim().to_string();

        let raw_type = record.get(type_idx).unwrap_or("").trim().to_lowercase();
        let skill_type = if raw_type.contains("knowledge") {
            "knowledge".to_string()
        } else {
            "skill".to_string()
        };

        let alt_labels: Vec<String> = alt_idx
            .and_then(|i| record.get(i))
            .map(|s| {
                s.split('\n')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        map.insert(
            uri.clone(),
            EscoSkill {
                uri,
                name,
                description,
                skill_type,
                group_uri: None,
                alt_labels,
                broader: Vec::new(),
                narrower: Vec::new(),
            },
        );
    }

    Ok(map)
}

/// Find the skills CSV file. ESCO exports may name it differently across versions.
fn find_skills_csv(raw_dir: &Path) -> Result<std::path::PathBuf> {
    // Try common file names
    for name in &["skills_en.csv", "skills.csv"] {
        let p = raw_dir.join(name);
        if p.exists() {
            return Ok(p);
        }
    }

    // Fall back: find any CSV containing "skill" in the name
    if let Ok(entries) = fs::read_dir(raw_dir) {
        for entry in entries.flatten() {
            let fname = entry.file_name().to_string_lossy().to_lowercase();
            if fname.contains("skill") && fname.ends_with(".csv") && !fname.contains("relation") {
                return Ok(entry.path());
            }
        }
    }

    anyhow::bail!(
        "Could not find ESCO skills CSV in {}. Run download first.",
        raw_dir.display()
    );
}

/// Load broader/narrower relations from broaderRelationsSkillPillar.csv.
/// Returns (broader_map, narrower_map) where each maps a concept URI to its
/// broader/narrower URIs.
fn load_broader_relations(
    raw_dir: &Path,
) -> Result<(HashMap<String, Vec<String>>, HashMap<String, Vec<String>>)> {
    let path = find_relations_csv(raw_dir)?;
    let mut rdr = csv::ReaderBuilder::new()
        .from_path(&path)
        .with_context(|| format!("opening {}", path.display()))?;

    let headers = rdr.headers().context("reading relations CSV headers")?.clone();
    let concept_idx = col_index(&headers, "conceptUri")?;
    let broader_idx = col_index(&headers, "broaderUri")?;

    let mut broader_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut narrower_map: HashMap<String, Vec<String>> = HashMap::new();

    for result in rdr.records() {
        let record = result.context("reading relations CSV row")?;
        let concept = record.get(concept_idx).unwrap_or("").trim().to_string();
        let broader = record.get(broader_idx).unwrap_or("").trim().to_string();

        if concept.is_empty() || broader.is_empty() {
            continue;
        }

        broader_map
            .entry(concept.clone())
            .or_default()
            .push(broader.clone());
        narrower_map
            .entry(broader)
            .or_default()
            .push(concept);
    }

    Ok((broader_map, narrower_map))
}

/// Find the broader relations CSV file.
fn find_relations_csv(raw_dir: &Path) -> Result<std::path::PathBuf> {
    for name in &[
        "broaderRelationsSkillPillar.csv",
        "broaderRelationsSkillPillar_en.csv",
    ] {
        let p = raw_dir.join(name);
        if p.exists() {
            return Ok(p);
        }
    }

    // Fall back: find any CSV containing "broader" in the name
    if let Ok(entries) = fs::read_dir(raw_dir) {
        for entry in entries.flatten() {
            let fname = entry.file_name().to_string_lossy().to_lowercase();
            if fname.contains("broader") && fname.contains("skill") && fname.ends_with(".csv") {
                return Ok(entry.path());
            }
        }
    }

    anyhow::bail!(
        "Could not find ESCO broader relations CSV in {}. Run download first.",
        raw_dir.display()
    );
}

/// Load skill groups from skillGroups_en.csv and skillsHierarchy_en.csv.
/// Builds a flat list of EscoSkillGroup with level and parent_uri.
fn load_skill_groups(raw_dir: &Path) -> Result<Vec<EscoSkillGroup>> {
    // Step 1: Parse skillGroups_en.csv for name, description, code
    let groups_path = raw_dir.join("skillGroups_en.csv");
    anyhow::ensure!(
        groups_path.exists(),
        "Missing skillGroups_en.csv in {}",
        raw_dir.display()
    );

    let mut group_info: HashMap<String, (String, String, String)> = HashMap::new(); // uri -> (name, description, code)
    {
        let mut rdr = csv::ReaderBuilder::new()
            .from_path(&groups_path)
            .with_context(|| format!("opening {}", groups_path.display()))?;
        let headers = rdr.headers().context("reading skillGroups headers")?.clone();
        let uri_idx = col_index(&headers, "conceptUri")?;
        let label_idx = col_index(&headers, "preferredLabel")?;
        let desc_idx = col_index(&headers, "description")?;
        let code_idx = col_index(&headers, "code")?;

        for result in rdr.records() {
            let record = result.context("reading skillGroups row")?;
            let uri = record.get(uri_idx).unwrap_or("").trim().to_string();
            if uri.is_empty() {
                continue;
            }
            let name = record.get(label_idx).unwrap_or("").trim().to_string();
            let description = record.get(desc_idx).unwrap_or("").trim().to_string();
            let code = record.get(code_idx).unwrap_or("").trim().to_string();
            group_info.insert(uri, (name, description, code));
        }
    }

    // Step 2: Parse skillsHierarchy_en.csv to determine level for each group URI
    let hierarchy_path = raw_dir.join("skillsHierarchy_en.csv");
    anyhow::ensure!(
        hierarchy_path.exists(),
        "Missing skillsHierarchy_en.csv in {}",
        raw_dir.display()
    );

    // Maps: group_uri -> (level, code, parent_uri)
    let mut group_levels: HashMap<String, (u8, String, Option<String>)> = HashMap::new();
    {
        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .from_path(&hierarchy_path)
            .with_context(|| format!("opening {}", hierarchy_path.display()))?;
        let headers = rdr.headers().context("reading hierarchy headers")?.clone();
        let l0_uri_idx = col_index(&headers, "Level 0 URI")?;
        let l1_uri_idx = col_index(&headers, "Level 1 URI")?;
        let l2_uri_idx = col_index(&headers, "Level 2 URI")?;
        let l3_uri_idx = col_index(&headers, "Level 3 URI")?;
        let l0_code_idx = col_index(&headers, "Level 0 code")?;
        let l1_code_idx = col_index(&headers, "Level 1 code")?;
        let l2_code_idx = col_index(&headers, "Level 2 code")?;
        let l3_code_idx = col_index(&headers, "Level 3 code")?;

        for result in rdr.records() {
            let record = result.context("reading hierarchy row")?;
            let l0_uri = record.get(l0_uri_idx).unwrap_or("").trim().to_string();
            let l1_uri = record.get(l1_uri_idx).unwrap_or("").trim().to_string();
            let l2_uri = record.get(l2_uri_idx).unwrap_or("").trim().to_string();
            let l3_uri = record.get(l3_uri_idx).unwrap_or("").trim().to_string();
            let l0_code = record.get(l0_code_idx).unwrap_or("").trim().to_string();
            let l1_code = record.get(l1_code_idx).unwrap_or("").trim().to_string();
            let l2_code = record.get(l2_code_idx).unwrap_or("").trim().to_string();
            let l3_code = record.get(l3_code_idx).unwrap_or("").trim().to_string();

            // Each row defines the deepest non-empty level
            if !l3_uri.is_empty() {
                group_levels
                    .entry(l3_uri)
                    .or_insert((3, l3_code, Some(l2_uri.clone())));
            }
            if !l2_uri.is_empty() {
                group_levels
                    .entry(l2_uri)
                    .or_insert((2, l2_code, Some(l1_uri.clone())));
            }
            if !l1_uri.is_empty() {
                group_levels
                    .entry(l1_uri)
                    .or_insert((1, l1_code, Some(l0_uri.clone())));
            }
            if !l0_uri.is_empty() {
                group_levels
                    .entry(l0_uri)
                    .or_insert((0, l0_code, None));
            }
        }
    }

    // Step 3: Build final list — only groups that appear in the hierarchy
    let mut groups: Vec<EscoSkillGroup> = Vec::new();
    for (uri, (level, _code, parent_uri)) in &group_levels {
        let (name, description, code) = match group_info.get(uri) {
            Some(info) => info.clone(),
            None => continue, // skip if no group info (shouldn't happen)
        };

        groups.push(EscoSkillGroup {
            uri: uri.clone(),
            name,
            code,
            description,
            level: *level,
            parent_uri: parent_uri.clone().filter(|u| !u.is_empty()),
        });
    }

    groups.sort_by(|a, b| a.code.cmp(&b.code).then(a.name.cmp(&b.name)));
    Ok(groups)
}

/// Load KnowledgeSkillCompetence → SkillGroup assignments from broaderRelationsSkillPillar.
/// Returns map: skill_uri → group_uri (first SkillGroup parent found).
fn load_skill_group_assignments(raw_dir: &Path) -> Result<HashMap<String, String>> {
    let path = find_relations_csv(raw_dir)?;
    let mut rdr = csv::ReaderBuilder::new()
        .from_path(&path)
        .with_context(|| format!("opening {}", path.display()))?;

    let headers = rdr.headers().context("reading relations CSV headers")?.clone();
    let concept_type_idx = col_index(&headers, "conceptType")?;
    let concept_uri_idx = col_index(&headers, "conceptUri")?;
    let broader_type_idx = col_index(&headers, "broaderType")?;
    let broader_uri_idx = col_index(&headers, "broaderUri")?;

    let mut skill_to_group: HashMap<String, String> = HashMap::new();

    for result in rdr.records() {
        let record = result.context("reading relations row")?;
        let concept_type = record.get(concept_type_idx).unwrap_or("").trim();
        let broader_type = record.get(broader_type_idx).unwrap_or("").trim();

        // We want KnowledgeSkillCompetence → SkillGroup relations
        if concept_type == "KnowledgeSkillCompetence" && broader_type == "SkillGroup" {
            let concept_uri = record.get(concept_uri_idx).unwrap_or("").trim().to_string();
            let broader_uri = record.get(broader_uri_idx).unwrap_or("").trim().to_string();
            if !concept_uri.is_empty() && !broader_uri.is_empty() {
                // Use first assignment (don't overwrite)
                skill_to_group.entry(concept_uri).or_insert(broader_uri);
            }
        }
    }

    Ok(skill_to_group)
}
