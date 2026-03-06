# Datasets

This directory holds raw downloads and processed output for the four external data sources that feed the competency library.

## Directory Structure

```
datasets/
├── README.md              # This file
├── raw/                   # Downloaded originals (git-ignored, re-downloadable)
│   ├── onet/              # O*NET 30.2 zip + extracted text files
│   ├── lightcast/         # Lightcast API JSON responses
│   ├── opm/               # OPM MOSAIC .xls spreadsheet
│   └── workbank/          # WORKBank dataset from HuggingFace
└── processed/             # Cleaned, normalized output (tracked in git)
    ├── onet.json
    ├── lightcast.json
    ├── opm.json
    └── workbank.json
```

`datasets/raw/` is listed in `.gitignore` because the files are large and re-downloadable. Run `cargo run -- fetch` to populate it.

`datasets/processed/` contains the normalized JSON output checked into the repo so downstream consumers do not need API keys or network access.

## Data Sources

### O\*NET 30.2

| Field | Value |
|-------|-------|
| **License** | CC BY 4.0 |
| **Download** | https://www.onetcenter.org/dl_files/database/db_30_2_text.zip |
| **Description** | U.S. Department of Labor occupational taxonomy. We extract six content-model domains: Abilities (52 items), Skills (35), Knowledge (33), Work Styles (16), Work Activities (41), and the Content Model reference structure. |
| **Raw format** | Tab-delimited `.txt` files inside a zip archive |
| **Key files** | `Abilities.txt`, `Skills.txt`, `Knowledge.txt`, `Work Styles.txt`, `Work Activities.txt`, `Content Model Reference.txt` |

#### Processed output example (`onet.json`)

```json
[
  {
    "id": "onet:ability:1.A.1.a.1",
    "source": "onet",
    "domain": "ability",
    "name": "Oral Comprehension",
    "description": "The ability to listen to and understand information and ideas presented through spoken words and sentences.",
    "scale_id": "IM",
    "scale_name": "Importance",
    "onet_element_id": "1.A.1.a.1"
  }
]
```

### Lightcast Open Skills

| Field | Value |
|-------|-------|
| **License** | CC BY-SA 4.0 |
| **Download** | API: `https://api.lightcast.io/skills/versions/latest/skills` |
| **Description** | Lightcast (formerly Emsi/Burning Glass) open skills taxonomy. 33,000+ skills with type classifications (Hard Skill, Soft Skill, Certification, etc.) and category groupings. |
| **Raw format** | JSON from REST API |
| **Auth** | Requires `LIGHTCAST_CLIENT_ID` and `LIGHTCAST_CLIENT_SECRET` in `.env` |

#### Processed output example (`lightcast.json`)

```json
[
  {
    "id": "lightcast:KS120076FGP5SFSIL3FR",
    "source": "lightcast",
    "name": "Python (Programming Language)",
    "type": "Hard Skill",
    "category": "Information Technology",
    "subcategory": "Programming Languages",
    "lightcast_id": "KS120076FGP5SFSIL3FR"
  }
]
```

### OPM MOSAIC Competencies

| Field | Value |
|-------|-------|
| **License** | Public domain (U.S. Government work) |
| **Download** | https://www.opm.gov/policy-data-oversight/assessment-and-selection/competencies/mosaic-studies-and-competencies.xls |
| **Description** | U.S. Office of Personnel Management general competencies derived from MOSAIC (Multipurpose Occupational Systems Analysis Inventory -- Close-ended) studies. 32 competencies with definitions applicable across federal occupations. |
| **Raw format** | Excel `.xls` spreadsheet |

#### Processed output example (`opm.json`)

```json
[
  {
    "id": "opm:accountability",
    "source": "opm",
    "name": "Accountability",
    "description": "Holds self and others accountable for measurable high-quality, timely, and cost-effective results.",
    "opm_slug": "accountability"
  }
]
```

### WORKBank

| Field | Value |
|-------|-------|
| **License** | MIT |
| **Download** | HuggingFace: `SALT-NLP/WORKBank` |
| **Description** | Research dataset linking 844 tasks across 104 occupations with human-annotated worker desire scores and AI capability ratings. Useful for understanding which competencies are automatable vs. uniquely human. |
| **Raw format** | Parquet / CSV from HuggingFace datasets |

#### Processed output example (`workbank.json`)

```json
[
  {
    "id": "workbank:task:1",
    "source": "workbank",
    "task": "Analyze financial data to identify trends and make recommendations",
    "occupation": "Financial Analysts",
    "occupation_code": "13-2051.00",
    "worker_desire": 3.8,
    "ai_capability": 4.2
  }
]
```

## Fetching Data

```bash
# Download and process all sources
cargo run -- fetch --all

# Download a single source
cargo run -- fetch --source onet
cargo run -- fetch --source lightcast
cargo run -- fetch --source opm
cargo run -- fetch --source workbank
```

Lightcast requires API credentials. Copy `.env.example` to `.env` and fill in:

```
LIGHTCAST_CLIENT_ID=your_client_id
LIGHTCAST_CLIENT_SECRET=your_client_secret
```

The other three sources are freely downloadable without authentication.

## Processed Output Format

All processed files follow the same conventions:

- **Top-level array** of objects, one per competency/skill/task
- **`id` field** prefixed with source name (e.g., `onet:ability:1.A.1.a.1`, `lightcast:KS120...`, `opm:accountability`, `workbank:task:1`)
- **`source` field** for filtering when datasets are merged downstream
- **`name` or `task` field** as the human-readable label
- **`description` field** where available

The library module (`src/library/`) reads these processed files and unifies them into the internal competency schema.

## Updating

When a new version of a source is released (e.g., O\*NET 31.0), update the download URL in `src/datasets/<source>.rs`, re-run `cargo run -- fetch --source <name>`, and commit the updated processed JSON.
