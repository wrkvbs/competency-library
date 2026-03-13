use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Public output types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentModelElement {
    pub element_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleAnchor {
    pub level: u8,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub element_id: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scale_anchors: Vec<ScaleAnchor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub element_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub element_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkStyle {
    pub element_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkActivity {
    pub element_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechSkill {
    pub name: String,
    pub commodity_code: String,
    pub commodity_title: String,
    pub class_title: String,
    pub family_title: String,
    pub segment_title: String,
    pub hot_technology: bool,
    pub in_demand: bool,
    pub occupation_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnspscCategory {
    pub commodity_code: String,
    pub commodity_title: String,
    pub class_code: String,
    pub class_title: String,
    pub family_code: String,
    pub family_title: String,
    pub segment_code: String,
    pub segment_title: String,
}

// ---------------------------------------------------------------------------
// Download
// ---------------------------------------------------------------------------

const ONET_ZIP_URL: &str = "https://www.onetcenter.org/dl_files/database/db_30_2_text.zip";

/// Download the O*NET database ZIP and extract it into `data_dir/datasets/raw/onet/`.
pub async fn download(data_dir: &Path) -> Result<()> {
    let raw_dir = data_dir.join("datasets/raw/onet");
    fs::create_dir_all(&raw_dir).context("creating raw onet directory")?;

    eprintln!("Downloading O*NET ZIP from {ONET_ZIP_URL}");
    let response = reqwest::get(ONET_ZIP_URL)
        .await
        .context("requesting O*NET ZIP")?;

    let bytes = response
        .error_for_status()
        .context("O*NET ZIP response status")?
        .bytes()
        .await
        .context("reading O*NET ZIP body")?;

    eprintln!("Downloaded {} bytes, extracting...", bytes.len());

    let cursor = io::Cursor::new(&bytes[..]);
    let mut archive = zip::ZipArchive::new(cursor).context("opening ZIP archive")?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).context("reading ZIP entry")?;
        let name = file.name().to_string();

        // Skip directories
        if name.ends_with('/') {
            continue;
        }

        // Flatten: strip any leading directory component (e.g. "db_30_2_text/")
        let file_name = match name.rfind('/') {
            Some(pos) => &name[pos + 1..],
            None => &name,
        };

        if file_name.is_empty() {
            continue;
        }

        let out_path = raw_dir.join(file_name);
        let mut out_file =
            fs::File::create(&out_path).with_context(|| format!("creating {}", out_path.display()))?;
        io::copy(&mut file, &mut out_file)
            .with_context(|| format!("writing {}", out_path.display()))?;
    }

    eprintln!("O*NET extraction complete → {}", raw_dir.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// Process
// ---------------------------------------------------------------------------

/// Process the extracted O*NET text files into structured JSON outputs under
/// `data_dir/datasets/processed/`.
pub fn process(data_dir: &Path) -> Result<()> {
    let raw_dir = data_dir.join("datasets/raw/onet");
    let out_dir = data_dir.join("datasets/processed");
    fs::create_dir_all(&out_dir).context("creating processed directory")?;

    // Load Content Model Reference — the master element list with descriptions
    let cmr = load_content_model_reference(&raw_dir)?;

    // 1. Content model (full hierarchy)
    write_json(&out_dir.join("onet-content-model.json"), &cmr)?;

    // Build lookup maps
    let cmr_map: HashMap<&str, &ContentModelElement> =
        cmr.iter().map(|e| (e.element_id.as_str(), e)).collect();

    // 2. Abilities — unique elements starting with "1.A."
    let abilities = build_abilities(&raw_dir, &cmr_map)?;
    write_json(&out_dir.join("onet-abilities.json"), &abilities)?;

    // 3. Skills — elements starting with "2.A." (Basic) and "2.B." (Cross-Functional)
    let skills = build_simple_elements(&cmr, &["2.A.", "2.B."]);
    write_json(&out_dir.join("onet-skills.json"), &skills)?;

    // 4. Knowledge — elements starting with "2.C."
    let knowledge = build_simple_elements(&cmr, &["2.C."]);
    write_json(&out_dir.join("onet-knowledge.json"), &knowledge)?;

    // 5. Work Styles — elements starting with "1.D."
    let work_styles = build_simple_elements(&cmr, &["1.D."]);
    write_json(&out_dir.join("onet-work-styles.json"), &work_styles)?;

    // 6. Work Activities — elements starting with "4.A."
    let work_activities = build_simple_elements(&cmr, &["4.A."]);
    write_json(&out_dir.join("onet-work-activities.json"), &work_activities)?;

    // 7. Technology Skills + UNSPSC categories
    let (tech_skills, unspsc) = build_tech_skills(&raw_dir)?;
    write_json(&out_dir.join("onet-tech-skills.json"), &tech_skills)?;
    write_json(&out_dir.join("onet-unspsc.json"), &unspsc)?;

    eprintln!("O*NET processing complete → {}", out_dir.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn load_content_model_reference(raw_dir: &Path) -> Result<Vec<ContentModelElement>> {
    let path = raw_dir.join("Content Model Reference.txt");
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(&path)
        .with_context(|| format!("opening {}", path.display()))?;

    let mut elements = Vec::new();
    for result in rdr.records() {
        let record = result.context("reading Content Model Reference row")?;
        let element_id = record.get(0).unwrap_or("").trim().to_string();
        let name = record.get(1).unwrap_or("").trim().to_string();
        let description = record.get(2).unwrap_or("").trim().to_string();
        if !element_id.is_empty() {
            elements.push(ContentModelElement {
                element_id,
                name,
                description,
            });
        }
    }
    Ok(elements)
}

/// Derive parent element ID by trimming the last dotted segment.
fn parent_element_id(id: &str) -> Option<String> {
    let pos = id.rfind('.')?;
    let parent = &id[..pos];
    // Only return parent if it has at least one dot (i.e., it's a real element)
    if parent.contains('.') {
        Some(parent.to_string())
    } else {
        None
    }
}

/// Build abilities with parent info and scale anchors.
fn build_abilities(
    raw_dir: &Path,
    cmr_map: &HashMap<&str, &ContentModelElement>,
) -> Result<Vec<Ability>> {
    // Collect anchor data from Abilities.txt where Scale ID == "AB"
    let anchors = load_ability_anchors(raw_dir)?;

    // Ability elements are those starting with "1.A." that are leaf-level
    // (have 4 dotted segments, e.g. "1.A.1.a.1")
    let mut abilities: Vec<Ability> = Vec::new();
    for (id, elem) in cmr_map {
        if !id.starts_with("1.A.") {
            continue;
        }
        // Include all ability elements (parents + leaves)
        let parent_id = parent_element_id(id);
        let parent_name = parent_id
            .as_deref()
            .and_then(|pid| cmr_map.get(pid))
            .map(|e| e.name.clone());

        let scale_anchors = anchors.get(*id).cloned().unwrap_or_default();

        abilities.push(Ability {
            element_id: elem.element_id.clone(),
            name: elem.name.clone(),
            description: elem.description.clone(),
            parent_id,
            parent_name,
            scale_anchors,
        });
    }

    abilities.sort_by(|a, b| a.element_id.cmp(&b.element_id));
    Ok(abilities)
}

/// Parse Abilities.txt for scale anchor rows (Scale ID = "AB").
fn load_ability_anchors(raw_dir: &Path) -> Result<HashMap<String, Vec<ScaleAnchor>>> {
    let path = raw_dir.join("Abilities.txt");
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(&path)
        .with_context(|| format!("opening {}", path.display()))?;

    let headers = rdr.headers().context("reading Abilities.txt headers")?.clone();
    let scale_id_idx = col_index(&headers, "Scale ID")?;
    let element_id_idx = col_index(&headers, "Element ID")?;
    let data_value_idx = col_index(&headers, "Data Value")?;
    // The anchor description is in the "Category" or "Element Name" field —
    // O*NET anchor file uses a separate "Level" file. But for Abilities.txt
    // with Scale ID "AB", the Level and Anchor are:
    //   Scale ID = "AB", Data Value = anchor level, Element Name = ability name
    // Actually, anchors live in "Level Scale Anchors.txt" — let's check for that file.
    // If not found, we skip anchors gracefully.

    // In O*NET db, Abilities.txt has occupation-level ratings (Scale ID IM/LV).
    // Anchor descriptions are in "Level Scale Anchors.txt".
    drop(rdr);
    drop(headers);

    load_level_scale_anchors(raw_dir, scale_id_idx, element_id_idx, data_value_idx)
}

/// Load anchors from "Level Scale Anchors.txt" which has:
/// Element ID, Scale ID, Anchor Value, Anchor Description
fn load_level_scale_anchors(
    raw_dir: &Path,
    _scale_id_idx: usize,
    _element_id_idx: usize,
    _data_value_idx: usize,
) -> Result<HashMap<String, Vec<ScaleAnchor>>> {
    let path = raw_dir.join("Level Scale Anchors.txt");
    if !path.exists() {
        eprintln!("Level Scale Anchors.txt not found, skipping anchor data");
        return Ok(HashMap::new());
    }

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(&path)
        .with_context(|| format!("opening {}", path.display()))?;

    let headers = rdr.headers().context("reading Level Scale Anchors headers")?.clone();
    let eid_idx = col_index(&headers, "Element ID")?;
    let anchor_val_idx = col_index(&headers, "Anchor Value")?;
    let anchor_desc_idx = col_index(&headers, "Anchor Description")?;

    let mut map: HashMap<String, Vec<ScaleAnchor>> = HashMap::new();

    for result in rdr.records() {
        let record = result.context("reading Level Scale Anchors row")?;
        let element_id = record.get(eid_idx).unwrap_or("").trim().to_string();

        // Only include ability anchors (1.A.*)
        if !element_id.starts_with("1.A.") {
            continue;
        }

        let level_str = record.get(anchor_val_idx).unwrap_or("").trim();
        let level: u8 = match level_str.parse::<f64>() {
            Ok(v) => v as u8,
            Err(_) => continue,
        };
        let description = record.get(anchor_desc_idx).unwrap_or("").trim().to_string();

        if !description.is_empty() {
            map.entry(element_id)
                .or_default()
                .push(ScaleAnchor { level, description });
        }
    }

    // Sort anchors by level within each element
    for anchors in map.values_mut() {
        anchors.sort_by_key(|a| a.level);
    }

    Ok(map)
}

/// Build simple element lists (skills, knowledge, work styles, work activities)
/// filtered by element ID prefixes.
fn build_simple_elements(cmr: &[ContentModelElement], prefixes: &[&str]) -> Vec<Skill> {
    let mut items: Vec<Skill> = cmr
        .iter()
        .filter(|e| prefixes.iter().any(|p| e.element_id.starts_with(p)))
        .map(|e| Skill {
            element_id: e.element_id.clone(),
            name: e.name.clone(),
            description: e.description.clone(),
        })
        .collect();
    items.sort_by(|a, b| a.element_id.cmp(&b.element_id));
    items
}

/// Build tech skills (deduplicated across occupations) and UNSPSC categories
/// (filtered to only tech-relevant commodity codes).
fn build_tech_skills(raw_dir: &Path) -> Result<(Vec<TechSkill>, Vec<UnspscCategory>)> {
    // Load UNSPSC reference into a lookup by commodity code
    let unspsc_map = load_unspsc_reference(raw_dir)?;

    // Read Technology Skills.txt
    let path = raw_dir.join("Technology Skills.txt");
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(&path)
        .with_context(|| format!("opening {}", path.display()))?;

    let headers = rdr.headers().context("reading Technology Skills headers")?.clone();
    let onet_soc_idx = col_index(&headers, "O*NET-SOC Code")?;
    let example_idx = col_index(&headers, "Example")?;
    let commodity_code_idx = col_index(&headers, "Commodity Code")?;
    let hot_tech_idx = col_index(&headers, "Hot Technology")?;
    let in_demand_idx = col_index(&headers, "In Demand")?;

    // Key: (name, commodity_code) -> aggregation state
    struct Agg {
        hot_technology: bool,
        in_demand: bool,
        occupations: HashSet<String>,
    }

    let mut map: HashMap<(String, String), Agg> = HashMap::new();

    for result in rdr.records() {
        let record = result.context("reading Technology Skills row")?;
        let soc_code = record.get(onet_soc_idx).unwrap_or("").trim().to_string();
        let name = record.get(example_idx).unwrap_or("").trim().to_string();
        let commodity_code = record.get(commodity_code_idx).unwrap_or("").trim().to_string();
        let hot = record.get(hot_tech_idx).unwrap_or("").trim() == "Y";
        let demand = record.get(in_demand_idx).unwrap_or("").trim() == "Y";

        if name.is_empty() || commodity_code.is_empty() {
            continue;
        }

        let key = (name.clone(), commodity_code.clone());
        let entry = map.entry(key).or_insert_with(|| Agg {
            hot_technology: false,
            in_demand: false,
            occupations: HashSet::new(),
        });
        entry.hot_technology = entry.hot_technology || hot;
        entry.in_demand = entry.in_demand || demand;
        entry.occupations.insert(soc_code);
    }

    // Collect commodity codes that appear in tech skills (owned, so we can consume map)
    let tech_commodity_codes: HashSet<String> = map.keys().map(|(_, c)| c.clone()).collect();

    // Build TechSkill vec
    let mut tech_skills: Vec<TechSkill> = map
        .into_iter()
        .map(|((name, commodity_code), agg)| {
            let unspsc = unspsc_map.get(commodity_code.as_str());
            TechSkill {
                name,
                commodity_code: commodity_code.clone(),
                commodity_title: unspsc.map_or_else(String::new, |u| u.commodity_title.clone()),
                class_title: unspsc.map_or_else(String::new, |u| u.class_title.clone()),
                family_title: unspsc.map_or_else(String::new, |u| u.family_title.clone()),
                segment_title: unspsc.map_or_else(String::new, |u| u.segment_title.clone()),
                hot_technology: agg.hot_technology,
                in_demand: agg.in_demand,
                occupation_count: agg.occupations.len() as u32,
            }
        })
        .collect();

    tech_skills.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // Filter UNSPSC to tech-relevant categories
    let mut unspsc: Vec<UnspscCategory> = unspsc_map
        .into_values()
        .filter(|u| tech_commodity_codes.contains(&u.commodity_code))
        .collect();
    unspsc.sort_by(|a, b| a.commodity_code.cmp(&b.commodity_code));

    eprintln!(
        "  tech skills: {} unique, UNSPSC categories: {} (tech-relevant)",
        tech_skills.len(),
        unspsc.len()
    );

    Ok((tech_skills, unspsc))
}

/// Load UNSPSC Reference.txt into a HashMap keyed by commodity code.
fn load_unspsc_reference(raw_dir: &Path) -> Result<HashMap<String, UnspscCategory>> {
    let path = raw_dir.join("UNSPSC Reference.txt");
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(&path)
        .with_context(|| format!("opening {}", path.display()))?;

    let headers = rdr.headers().context("reading UNSPSC Reference headers")?.clone();
    let commodity_code_idx = col_index(&headers, "Commodity Code")?;
    let commodity_title_idx = col_index(&headers, "Commodity Title")?;
    let class_code_idx = col_index(&headers, "Class Code")?;
    let class_title_idx = col_index(&headers, "Class Title")?;
    let family_code_idx = col_index(&headers, "Family Code")?;
    let family_title_idx = col_index(&headers, "Family Title")?;
    let segment_code_idx = col_index(&headers, "Segment Code")?;
    let segment_title_idx = col_index(&headers, "Segment Title")?;

    let mut map = HashMap::new();
    for result in rdr.records() {
        let record = result.context("reading UNSPSC Reference row")?;
        let commodity_code = record.get(commodity_code_idx).unwrap_or("").trim().to_string();
        if commodity_code.is_empty() {
            continue;
        }
        map.insert(
            commodity_code.clone(),
            UnspscCategory {
                commodity_code,
                commodity_title: record.get(commodity_title_idx).unwrap_or("").trim().to_string(),
                class_code: record.get(class_code_idx).unwrap_or("").trim().to_string(),
                class_title: record.get(class_title_idx).unwrap_or("").trim().to_string(),
                family_code: record.get(family_code_idx).unwrap_or("").trim().to_string(),
                family_title: record.get(family_title_idx).unwrap_or("").trim().to_string(),
                segment_code: record.get(segment_code_idx).unwrap_or("").trim().to_string(),
                segment_title: record.get(segment_title_idx).unwrap_or("").trim().to_string(),
            },
        );
    }

    Ok(map)
}

fn col_index(headers: &csv::StringRecord, name: &str) -> Result<usize> {
    headers
        .iter()
        .position(|h| h.trim() == name)
        .with_context(|| format!("column '{}' not found in headers: {:?}", name, headers))
}

fn write_json<T: Serialize>(path: &Path, data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data).context("serializing JSON")?;
    fs::write(path, json).with_context(|| format!("writing {}", path.display()))?;
    eprintln!("  wrote {}", path.display());
    Ok(())
}
