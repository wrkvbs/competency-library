# Research Index

Master reference for all source material feeding the competency-library.

## Internal Research (Vault)

| Source | Path | Summary |
|--------|------|---------|
| Response Scoring & Competency Framework | `notes/01_Projects/Response Scoring Framework/Response Scoring & Competency Framework.md` | Master framework: quality vs signal axes, three-layer assessment model, competency taxonomy, confidence mechanics |
| O\*NET / OPM / Smart & Street Research | `notes/01_Projects/Response Scoring Framework/Research/ONET OPM Smart Street Competency Models Research.md` | Comparative analysis of federal competency models and hiring frameworks |
| Lightcast Research | `notes/01_Projects/Response Scoring Framework/Research/Lightcast Labor Market Data Research.md` | Labor market data, skills taxonomies, open skills API evaluation |
| Stanford SALT Lab Research | `notes/01_Projects/Response Scoring Framework/Research/Stanford SALT Lab - Future of Work with AI Agents.md` | WORKBank dataset, task-level work analysis, AI impact on occupations |
| Competency Assessment Methods & Reliability | `notes/01_Projects/Response Scoring Framework/Research/Competency Assessment Methods & Reliability Research.md` | HOW to measure: structured interviews, BARS rubrics, AI scoring validity, psychometrics, calibration methodology, bias detection. Covers Schmidt & Hunter, Sackett 2022, HireVue validation, STAR/BEI, inter-rater reliability metrics (ICC, Kappa, QWK), three-way signal classification, and regulatory landscape. |

## Internal Research (Notion)

| Source | Link | Summary |
|--------|------|---------|
| Competencies / User Understanding | [Notion page](https://www.notion.so/21dfd29d6cf380719a4ac65c0e304ddc) | Product thinking on what competencies mean for allUP users |
| Job Criteria Brainstorm | [Notion page](https://www.notion.so/2aafd29d6cf3809bac51d1573c13bfd6) | Brainstorm on job-matching criteria and signal sources |
| Video Ranking Exploration | [Notion page](https://www.notion.so/allupworld/303fd29d6cf381e5a85cda899c02013c) | Exploration of video response ranking approaches |

## External Datasets

These are the primary external taxonomies this library ingests and normalizes.

| Dataset | URL | Format | Notes |
|---------|-----|--------|-------|
| O\*NET Database | [onetcenter.org/database.html](https://www.onetcenter.org/database.html) | CSV/Excel zip | ~1,000 occupations, abilities, skills, knowledge, work activities, work context. Updated annually. |
| O\*NET API | [services.onetcenter.org/reference](https://services.onetcenter.org/reference/) | REST JSON | Live queries against O\*NET data. Requires API key. |
| OPM MOSAIC | [opm.gov MOSAIC Excel](https://www.opm.gov/policy-data-oversight/assessment-and-selection/competencies/mosaic-studies-and-competencies.xls) | Excel (.xls) | Federal competencies mapped to GS job series. ~300 competencies across ~200 series. |
| OPM FWCI Handbook | [opm.gov CHCOC transmittals](https://www.opm.gov/chcoc/transmittals/2023/) | PDF | Federal Workforce Competency Index — hierarchical competency model for federal workforce. |
| Lightcast Open Skills | [lightcast.io/open-skills](https://lightcast.io/open-skills) | API (JSON) | ~30k skills with taxonomy and relationships. Real-time labor market signal. |
| WORKBank (GitHub) | [SALT-NLP/workbank](https://github.com/SALT-NLP/workbank) | Code + data | Stanford SALT Lab dataset: tasks, activities, and AI impact annotations for occupations. |
| WORKBank (HuggingFace) | [SALT-NLP/WORKBank](https://huggingface.co/datasets/SALT-NLP/WORKBank) | Dataset | HuggingFace-hosted version for direct download. |

## Related Repos

| Repo | Path | Relationship |
|------|------|-------------|
| execution-graph | `/Users/ww/dev/execution-graph` | Upstream pipeline that consumes competency-library outputs for scoring operations |
| competency-library | (this repo) | Ingests external taxonomies, normalizes into unified competency model |

## How Sources Map to Code

| Source | Module | Purpose |
|--------|--------|---------|
| O\*NET Database/API | `src/datasets/onet.rs` | Download, parse, and normalize O\*NET abilities/skills/knowledge |
| OPM MOSAIC | `src/datasets/opm.rs` | Download and parse MOSAIC competency-to-job-series mappings |
| Lightcast Open Skills | `src/datasets/lightcast.rs` | Fetch skills taxonomy via API |
| WORKBank | `src/datasets/workbank.rs` | Download and parse task-level occupation data |
| All of the above | `src/library/schema.rs` | Unified competency schema that normalizes across sources |
| Scoring Framework | `src/scoring/` | Prompt-based scoring using the competency library |
