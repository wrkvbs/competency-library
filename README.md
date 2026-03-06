# competency-library

Disposable exploration project for allUP's competency scoring framework. Downloads, processes, and cross-references external competency/skills taxonomies to inform the design of allUP's competency library and scoring pipeline.

This is **separate from the existing tag system**. Tags extract keywords from transcripts; this project builds a structured assessment layer that maps what candidates demonstrate in video responses to validated competency frameworks.

## Why

allUP has ~25k candidates and ~250k video responses. Transcripts power search today, but we need to understand both **response quality** (how good is this content?) and **competency signal** (what does this response tell us about the person?). External competency frameworks (O\*NET, OPM, Lightcast, WORKBank) provide the validated taxonomies to define what we measure and how.

Key insight from research: O\*NET's behavioral competencies (Work Styles, Cross-Functional Skills) map directly to what video transcripts reveal. Lightcast's 33,000+ technical skills fill the granularity gap. WORKBank adds worker preference data on which tasks to automate vs. augment. Together they form the backbone of allUP's competency library.

## Architecture

```
competency-library/
├── Cargo.toml
├── datasets/
│   ├── raw/              # Downloaded source files (gitignored, re-downloadable)
│   │   ├── onet/         # O*NET database ZIP contents
│   │   ├── opm/          # OPM MOSAIC competency documents
│   │   ├── lightcast/    # Lightcast Open Skills API responses
│   │   └── workbank/     # Stanford SALT Lab WORKBank data
│   └── processed/        # Normalized intermediate files
├── library/              # Phase 2: unified competency schema
├── scoring/              # Phase 3: response scoring pipeline
├── runs/                 # Execution outputs (gitignored)
├── docs/                 # Research index, decision log
└── src/
    ├── main.rs           # CLI entry point
    ├── lib.rs            # Module declarations
    ├── datasets/         # Download + processing per source
    │   ├── onet.rs       # O*NET database download + CSV parsing
    │   ├── opm.rs        # OPM MOSAIC PDF/document processing
    │   ├── lightcast.rs  # Lightcast Open Skills API client
    │   └── workbank.rs   # WORKBank GitHub data fetch
    ├── library/
    │   └── schema.rs     # Unified competency types (Phase 2)
    ├── scoring/
    │   ├── fetch.rs      # Response fetching via wrk-mcp (Phase 3)
    │   └── runner.rs     # Scoring pipeline runner (Phase 3)
    └── jd/               # JD parsing + search generation (Phase 4)

```

## Phases

### Phase 1: Data (current)

Download and process the four external datasets into normalized intermediate files:

- **O\*NET** — U.S. Department of Labor occupational database. 900+ occupations, ~136 dimensions each (abilities, skills, knowledge, work styles, work activities, work context). CC BY 4.0. Primary source for behavioral competencies and occupation-linked requirements.
- **OPM MOSAIC** — Federal Workforce Competency Initiative. 32 general competencies + Executive Core Qualifications (ECQs). Person-centered behavioral framework. Fills gaps O\*NET misses: accountability, organizational awareness, political savvy, decisiveness, entrepreneurship, vision, resilience.
- **Lightcast Open Skills** — 33,000+ skills taxonomy from job postings data. Updated biweekly. Provides granular technical skill vocabulary that O\*NET lacks (e.g., "Python" vs. O\*NET's broad "Programming"). Free tier: 50 extractions/month.
- **WORKBank** — Stanford SALT Lab database. 844 tasks across 104 occupations with dual worker-desire + expert-capability ratings. Built on O\*NET task codes. Adds worker preference layer (which competencies are human-essential vs. AI-augmentable).

### Phase 2: Library

Define a unified competency schema that merges insights from all four sources. Map O\*NET's behavioral dimensions, OPM's leadership competencies, Lightcast's technical skills, and WORKBank's automation zones into allUP's three-layer framework (Pre-filter, Person Attributes, Match Quality).

### Phase 3: Scoring

Build a pipeline that fetches allUP candidate responses (via wrk-mcp), scores them against the competency library, and outputs structured competency signals with direction, confidence, and evidence.

### Phase 4: JD-to-Search

Inject a job description, extract the competencies it requires using the library as a controlled vocabulary, and generate a structured search query against allUP's candidate pool. This closes the loop: candidates are scored on competencies (Phase 3), and employers search by competencies (Phase 4) — the library is the shared language between both sides.

The flow:
1. **JD ingestion** — parse a raw job description (text, URL, or structured input)
2. **Competency extraction** — map the JD's requirements to competencies in our library (using LLM + library definitions as grounding)
3. **Search generation** — translate extracted competencies into an allUP search query (weighted by importance, required vs. preferred, seniority signals)
4. **Candidate ranking** — rank candidates by how well their scored competencies match the JD's requirements

## CLI Usage

```bash
# Build the project
cargo build

# Download and process O*NET database
cargo run -- download onet

# Download and process OPM MOSAIC competencies
cargo run -- download opm

# Fetch Lightcast Open Skills taxonomy
cargo run -- download lightcast

# Fetch WORKBank data from GitHub
cargo run -- download workbank

# Process all datasets into normalized format
cargo run -- process

# Run scoring pipeline against allUP responses (Phase 3)
cargo run -- score --sample 100
```

## Data Sources & Research

### External Datasets

| Dataset | Source | License | Size |
|---------|--------|---------|------|
| O\*NET | [onetcenter.org/database.html](https://www.onetcenter.org/database.html) | CC BY 4.0 | 900+ occupations, ~136 dimensions |
| OPM FWCI | [opm.gov/competencies](https://www.opm.gov/policy-data-oversight/assessment-and-selection/competencies/) | Public domain (federal) | 32 general competencies + ECQs |
| Lightcast Open Skills | [lightcast.io/open-skills](https://lightcast.io/open-skills) | CC BY-SA (definitions) | 33,000+ skills |
| WORKBank | [github.com/SALT-NLP/workbank](https://github.com/SALT-NLP/workbank) | Open (GitHub) | 844 tasks, 104 occupations |

### Vault Research Notes

| Research | Vault Path |
|----------|-----------|
| O\*NET / OPM / Smart & Street mapping | `01_Projects/Response Scoring Framework/Research/ONET OPM Smart Street Competency Models Research.md` |
| Lightcast products & pricing analysis | `01_Projects/Response Scoring Framework/Research/Lightcast Labor Market Data Research.md` |
| WORKBank / Stanford SALT Lab study | `01_Projects/Response Scoring Framework/Research/Stanford SALT Lab - Future of Work with AI Agents.md` |
| Master scoring framework | `01_Projects/Response Scoring Framework/Response Scoring & Competency Framework.md` |
| Assessment methods & reliability science | `01_Projects/Response Scoring Framework/Research/Competency Assessment Methods & Reliability Research.md` |
| Seniority matching proposal | `01_Projects/Response Scoring Framework/Research/Proposal for a Multidimensional Seniority Matching System.pdf` |
| Seniority matching visualization | `01_Projects/Response Scoring Framework/Research/Visualizing Multidimensional Seniority Matches in a Chat-Based Candidate Search Interface.pdf` |

### Notion Docs

| Document | URL |
|----------|-----|
| Job Criteria Brainstorm | https://www.notion.so/Job-Criteria-Brainstorm-2aafd29d6cf3809bac51d1573c13bfd6 |
| Competencies - User Understanding & Candidate Fit | https://www.notion.so/Competencies-User-Understanding-Candidate-Fit-21dfd29d6cf380719a4ac65c0e304ddc |
| Video Ranking Exploration | https://www.notion.so/allupworld/Video-Ranking-Exploration-303fd29d6cf381e5a85cda899c02013c |

### Key External References

- [O\*NET Content Model](https://www.onetcenter.org/content.html) — Official taxonomy docs
- [O\*NET OnLine](https://www.onetonline.org/) — Interactive occupational database
- [OPM FWCI General Competencies (2023 PDF)](https://www.opm.gov/chcoc/transmittals/2023/Federal%20Workforce%20Competency%20Initiative%20-%20General%20Competencies%20and%20Competency%20Models.pdf)
- [OPM Executive Core Qualifications](https://www.opm.gov/policy-data-oversight/senior-executive-service/executive-core-qualifications/)
- [Lightcast API Docs](https://docs.lightcast.dev/apis)
- [Lightcast Skills Taxonomy Whitepaper](https://4906807.fs1.hubspotusercontent-na1.net/hubfs/4906807/The%20Lightcast%20Open%20Skills%20Taxonomy%20Aug%202023.pdf)
- [Stanford SALT Lab — Future of Work](https://futureofwork.saltlab.stanford.edu/)
- [WORKBank Paper (arXiv)](https://arxiv.org/html/2506.06576v2)
- [ESCO Classification](https://esco.ec.europa.eu/en/classification) — EU equivalent, 13,939 skills (future expansion)

## Environment

Requires a `.env` file for Lightcast API credentials:

```
LIGHTCAST_CLIENT_ID=your_client_id
LIGHTCAST_CLIENT_SECRET=your_client_secret
```

O\*NET, OPM, and WORKBank downloads are public and require no credentials.

The project connects to allUP's production API via `wrk-mcp` (configured in `.mcp.json`) for Phase 3 scoring.

## Related Projects

- **execution-graph** (`/Users/ww/dev/execution-graph`) — allUP's operations pipeline where scoring operations will eventually live
- **Response Scoring Framework** (vault project) — The master framework this exploration feeds into
