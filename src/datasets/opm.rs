use std::path::Path;

use anyhow::{Context, Result};
use calamine::{open_workbook_auto, Data, Reader};
use serde::Serialize;

const DOWNLOAD_URL: &str =
    "https://www.opm.gov/policy-data-oversight/assessment-and-selection/competencies/mosaic-studies-and-competencies.xls";

const RAW_SUBDIR: &str = "datasets/raw/opm";
const RAW_FILENAME: &str = "mosaic-competencies.xlsx";
const PROCESSED_SUBDIR: &str = "datasets/processed";
const PROCESSED_FILENAME: &str = "opm-competencies.json";

#[derive(Debug, Serialize)]
pub struct OpmCompetency {
    pub name: String,
    pub definition: String,
    pub source: String,
    pub category: String,
}

/// Download the OPM MOSAIC competencies Excel file.
pub async fn download(data_dir: &Path) -> Result<()> {
    let raw_dir = data_dir.join(RAW_SUBDIR);
    std::fs::create_dir_all(&raw_dir)
        .with_context(|| format!("Failed to create directory: {}", raw_dir.display()))?;

    let dest = raw_dir.join(RAW_FILENAME);

    let response = reqwest::get(DOWNLOAD_URL)
        .await
        .with_context(|| format!("Failed to download {DOWNLOAD_URL}"))?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("HTTP {status} downloading OPM MOSAIC file from {DOWNLOAD_URL}");
    }

    let bytes = response
        .bytes()
        .await
        .context("Failed to read response body")?;

    std::fs::write(&dest, &bytes)
        .with_context(|| format!("Failed to write {}", dest.display()))?;

    println!("Downloaded OPM MOSAIC file to {}", dest.display());
    Ok(())
}

/// Process the downloaded OPM MOSAIC Excel file into structured JSON.
pub fn process(data_dir: &Path) -> Result<()> {
    let raw_path = data_dir.join(RAW_SUBDIR).join(RAW_FILENAME);
    let processed_dir = data_dir.join(PROCESSED_SUBDIR);
    std::fs::create_dir_all(&processed_dir)
        .with_context(|| format!("Failed to create directory: {}", processed_dir.display()))?;

    let output_path = processed_dir.join(PROCESSED_FILENAME);

    let mut workbook = open_workbook_auto(&raw_path)
        .with_context(|| format!("Failed to open workbook: {}", raw_path.display()))?;

    let sheet_names = workbook.sheet_names();
    let competencies = extract_competencies(&mut workbook, &sheet_names)
        .context("Failed to extract competencies from workbook")?;

    if competencies.is_empty() {
        anyhow::bail!(
            "No competencies extracted from {}. Available sheets: {:?}",
            raw_path.display(),
            workbook.sheet_names()
        );
    }

    let json = serde_json::to_string_pretty(&competencies)
        .context("Failed to serialize competencies to JSON")?;

    std::fs::write(&output_path, &json)
        .with_context(|| format!("Failed to write {}", output_path.display()))?;

    println!(
        "Processed {} OPM competencies to {}",
        competencies.len(),
        output_path.display()
    );
    Ok(())
}

/// Convert a calamine `Data` cell to a trimmed String.
fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::String(s) => s.trim().to_string(),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTimeIso(s) | Data::DurationIso(s) => s.trim().to_string(),
        Data::DateTime(_) | Data::Error(_) | Data::Empty => String::new(),
    }
}

/// Try to find and extract competencies from the workbook.
///
/// The OPM MOSAIC file structure varies, so we try multiple strategies:
/// 1. Look for a sheet whose name contains "competenc" (case-insensitive)
/// 2. Fall back to the first sheet
///
/// Within a sheet, we look for columns containing "competency"/"name" and "definition"/"description".
fn extract_competencies(
    workbook: &mut calamine::Sheets<std::io::BufReader<std::fs::File>>,
    sheet_names: &[String],
) -> Result<Vec<OpmCompetency>> {
    // Pick the best sheet
    let target_sheet = sheet_names
        .iter()
        .find(|name| name.to_lowercase().contains("competenc"))
        .or_else(|| sheet_names.first())
        .context("Workbook has no sheets")?
        .clone();

    let range = workbook
        .worksheet_range(&target_sheet)
        .with_context(|| format!("Failed to read sheet '{target_sheet}'"))?;

    let rows: Vec<Vec<String>> = range
        .rows()
        .map(|row| row.iter().map(cell_to_string).collect())
        .collect();

    if rows.is_empty() {
        anyhow::bail!("Sheet '{target_sheet}' is empty");
    }

    // Find the header row — look for a row that contains something like "competency" and "definition"
    let (header_idx, name_col, def_col) = find_header_and_columns(&rows)
        .context("Could not identify competency name and definition columns")?;

    // Optional: look for a "source" column
    let source_col = find_column_index(&rows[header_idx], &["source", "study"]);

    let mut competencies = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    for row in rows.iter().skip(header_idx + 1) {
        let name = row.get(name_col).map(|s| s.as_str()).unwrap_or("").trim();
        let definition = row.get(def_col).map(|s| s.as_str()).unwrap_or("").trim();

        // Skip empty rows or header-like duplicates
        if name.is_empty() || definition.is_empty() {
            continue;
        }

        // Skip if we've already seen this competency name (dedup)
        let name_lower = name.to_lowercase();
        if seen_names.contains(&name_lower) {
            continue;
        }
        seen_names.insert(name_lower);

        let source = source_col
            .and_then(|col| row.get(col))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "OPM MOSAIC".to_string());

        competencies.push(OpmCompetency {
            name: name.to_string(),
            definition: definition.to_string(),
            source,
            category: "General Competency".to_string(),
        });
    }

    Ok(competencies)
}

/// Search rows for a header row that contains competency name and definition columns.
/// Returns (header_row_index, name_column_index, definition_column_index).
fn find_header_and_columns(rows: &[Vec<String>]) -> Option<(usize, usize, usize)> {
    // Only scan the first 20 rows for a header
    let scan_limit = rows.len().min(20);

    for (row_idx, row) in rows.iter().enumerate().take(scan_limit) {
        let name_col = find_column_index(row, &["competency", "competency name", "name"]);
        let def_col = find_column_index(
            row,
            &["definition", "description", "competency definition"],
        );

        if let (Some(nc), Some(dc)) = (name_col, def_col) {
            return Some((row_idx, nc, dc));
        }
    }

    // Fallback: if we have at least 2 columns, assume col 0 = name, col 1 = definition
    // but only if the first row looks like a header
    if let Some(first_row) = rows.first() {
        if first_row.len() >= 2 {
            let looks_like_header = first_row
                .iter()
                .any(|c| c.to_lowercase().contains("compet") || c.to_lowercase().contains("name"));
            if looks_like_header {
                return Some((0, 0, 1));
            }
        }
    }

    None
}

/// Find a column index by checking if any cell in the row matches one of the candidate labels
/// (case-insensitive, substring match).
fn find_column_index(row: &[String], candidates: &[&str]) -> Option<usize> {
    for (col_idx, cell) in row.iter().enumerate() {
        let lower = cell.to_lowercase();
        for candidate in candidates {
            if lower.contains(candidate) {
                return Some(col_idx);
            }
        }
    }
    None
}
