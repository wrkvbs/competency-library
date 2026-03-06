use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// A single WORKBank task record, normalized for our processed output.
#[derive(Debug, Serialize)]
pub struct WorkBankTask {
    pub task_id: String,
    pub occupation: String,
    pub o_net_soc: String,
    pub task_statement: String,
    pub worker_desire_mean: Option<f64>,
    pub ai_capability_mean: Option<f64>,
    pub human_agency_level: String,
}

/// Raw CSV row — field names match expected WORKBank column headers.
/// We use `#[serde(alias)]` to tolerate common variations.
#[derive(Debug, Deserialize)]
struct RawRow {
    #[serde(alias = "Task ID", alias = "task_id", alias = "TaskID")]
    task_id: Option<String>,

    #[serde(
        alias = "Occupation",
        alias = "occupation",
        alias = "Title",
        alias = "title"
    )]
    occupation: Option<String>,

    #[serde(
        alias = "O*NET-SOC Code",
        alias = "O*NET-SOC 2019 Code",
        alias = "SOC",
        alias = "soc",
        alias = "o_net_soc",
        alias = "onet_soc_code",
        alias = "ONET_SOC_CODE"
    )]
    o_net_soc: Option<String>,

    #[serde(
        alias = "Task",
        alias = "task",
        alias = "Task Statement",
        alias = "task_statement",
        alias = "Task Description",
        alias = "task_description"
    )]
    task_statement: Option<String>,

    #[serde(
        alias = "Worker Desire Mean",
        alias = "worker_desire_mean",
        alias = "Worker_Desire_Mean",
        alias = "worker_desire"
    )]
    worker_desire_mean: Option<f64>,

    #[serde(
        alias = "AI Capability Mean",
        alias = "ai_capability_mean",
        alias = "AI_Capability_Mean",
        alias = "ai_capability"
    )]
    ai_capability_mean: Option<f64>,

    #[serde(
        alias = "Human Agency Level",
        alias = "human_agency_level",
        alias = "Human_Agency_Level",
        alias = "agency_level"
    )]
    human_agency_level: Option<String>,
}

const HF_BASE: &str = "https://huggingface.co/datasets/SALT-NLP/WORKBank/resolve/main";

/// Files to download: (remote path relative to HF_BASE, local filename).
const FILES: &[(&str, &str)] = &[
    ("task_data/task_statement_with_metadata.csv", "task_metadata.csv"),
    ("worker_data/domain_worker_desires.csv", "worker_desires.csv"),
    ("expert_ratings/expert_rated_technological_capability.csv", "expert_ratings.csv"),
];

/// Download all WORKBank CSVs to `<data_dir>/datasets/raw/workbank/`.
pub async fn download(data_dir: &Path) -> Result<()> {
    let raw_dir = data_dir.join("datasets/raw/workbank");
    std::fs::create_dir_all(&raw_dir)
        .with_context(|| format!("failed to create directory {}", raw_dir.display()))?;

    let client = reqwest::Client::builder()
        .user_agent("competency-library/0.1")
        .build()
        .context("failed to build HTTP client")?;

    for (remote_path, local_name) in FILES {
        let url = format!("{HF_BASE}/{remote_path}");
        println!("  downloading {url}");
        let resp = client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("request to {url} failed"))?;

        if !resp.status().is_success() {
            anyhow::bail!("HTTP {} from {url}", resp.status());
        }

        let bytes = resp
            .bytes()
            .await
            .with_context(|| format!("failed to read response body from {url}"))?;

        let dest = raw_dir.join(local_name);
        std::fs::write(&dest, &bytes)
            .with_context(|| format!("failed to write {}", dest.display()))?;
        println!("  saved {} ({} bytes)", dest.display(), bytes.len());
    }

    Ok(())
}

/// Process the raw WORKBank CSVs into `<data_dir>/datasets/processed/workbank-tasks.json`.
pub fn process(data_dir: &Path) -> Result<()> {
    let raw_path = data_dir.join("datasets/raw/workbank/task_metadata.csv");
    anyhow::ensure!(
        raw_path.exists(),
        "raw CSV not found at {}. Run download first.",
        raw_path.display()
    );

    let processed_dir = data_dir.join("datasets/processed");
    std::fs::create_dir_all(&processed_dir)
        .with_context(|| format!("failed to create directory {}", processed_dir.display()))?;

    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_path(&raw_path)
        .with_context(|| format!("failed to open {}", raw_path.display()))?;

    let mut tasks: Vec<WorkBankTask> = Vec::new();
    let mut row_num: usize = 0;

    for result in reader.deserialize::<RawRow>() {
        row_num += 1;
        let row = result.with_context(|| format!("failed to parse CSV row {row_num}"))?;

        // Skip rows that lack a task statement entirely — likely header duplicates or blanks.
        let task_statement = match row.task_statement {
            Some(ref s) if !s.trim().is_empty() => s.trim().to_string(),
            _ => continue,
        };

        let task_id = row
            .task_id
            .unwrap_or_else(|| format!("T{row_num}"));

        let occupation = row.occupation.unwrap_or_default().trim().to_string();
        let o_net_soc = row.o_net_soc.unwrap_or_default().trim().to_string();
        let human_agency_level = row.human_agency_level.unwrap_or_default().trim().to_string();

        tasks.push(WorkBankTask {
            task_id,
            occupation,
            o_net_soc,
            task_statement,
            worker_desire_mean: row.worker_desire_mean,
            ai_capability_mean: row.ai_capability_mean,
            human_agency_level,
        });
    }

    let out_path = processed_dir.join("workbank-tasks.json");
    let json = serde_json::to_string_pretty(&tasks)
        .context("failed to serialize tasks to JSON")?;
    std::fs::write(&out_path, &json)
        .with_context(|| format!("failed to write {}", out_path.display()))?;

    println!(
        "  wrote {} tasks to {}",
        tasks.len(),
        out_path.display()
    );

    Ok(())
}
