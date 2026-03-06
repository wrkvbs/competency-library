# CLAUDE.md

## Purpose

Disposable exploration repo for building a competency scoring system for allUP. This is research-grade code -- expect experimentation, rough edges, and frequent pivots. The goal is to ingest external competency taxonomies, build a unified competency library, and score allUP user responses against it.

**Key distinction:** Tags remain allUP's identity layer (how users describe themselves). Competency scores are a new assessment layer (what their responses reveal about their abilities).

**JD-to-Search use case:** The competency library also serves as the shared vocabulary between candidates and employers. Given a job description, extract the competencies it requires (using the library as grounding), then generate a structured search against allUP's candidate pool ranked by competency match. The library must be designed so that the same competency definitions work for both scoring responses (supply side) and parsing JDs (demand side).

## Architecture

```
competency-library/
├── src/
│   ├── main.rs              # CLI entry point (clap)
│   ├── lib.rs               # Module exports
│   ├── datasets/            # Download + process external taxonomies
│   │   ├── onet.rs          # O*NET (occupations, skills, abilities, knowledge)
│   │   ├── opm.rs           # OPM MOSAIC (federal competencies)
│   │   ├── lightcast.rs     # Lightcast API (labor market skills taxonomy)
│   │   └── workbank.rs      # WORKBank (competency assessments)
│   ├── library/             # Unified competency library
│   │   └── schema.rs        # Canonical competency types
│   └── scoring/             # Score allUP responses against library
│       ├── fetch.rs         # Fetch responses via wrk-mcp
│       └── runner.rs        # Scoring pipeline
├── datasets/
│   ├── raw/                 # Downloaded source files (gitignored)
│   │   ├── onet/
│   │   ├── opm/
│   │   ├── lightcast/
│   │   └── workbank/
│   └── processed/           # Cleaned/normalized output
├── library/
│   └── v1/                  # Versioned unified competency library
├── runs/                    # Scoring run outputs (gitignored)
├── docs/                    # Research notes and decision log
└── .claude/
    ├── agents/allup-graphql.md   # Agent for querying allUP API
    └── skills/allup-graphql/     # Skill with reference docs
```

## CLI Usage

```bash
# Download and process datasets
cargo run -- download onet
cargo run -- download opm
cargo run -- download lightcast
cargo run -- download workbank

# Process raw data into normalized format
cargo run -- process onet
cargo run -- process opm
cargo run -- process lightcast
cargo run -- process workbank

# Build unified library from processed datasets
cargo run -- build-library

# Score user responses
cargo run -- score --user <user-id>
cargo run -- score --sample 100
```

## Querying allUP Data

Use the `allup-graphql` agent for any queries against the allUP backend. It handles schema lookup, auth, and query construction.

**Delegate, don't query directly.** The agent has reference docs for every service domain (users, prompts, jobs, analysis, search, etc.) at `.claude/skills/allup-graphql/reference/`.

The wrk-mcp server is configured to hit production (`-e production` in `.mcp.json`).

## Test Users

Use these accounts for scoring experiments:

| Name | User ID |
|------|---------|
| Scott Mace | `8f0035ba-54ca-4c64-84a5-03bf2fa9b207` |
| Weston Westenborg | `78dbb4c3-683f-4c30-b66d-a125528ddf8b` |
| Dave Grijalva | `d1ab149e-0981-4e7d-a236-d3b8bfaa8690` |
| Joe Fernandez | `cb1ef5ac-f2e9-44c9-9d04-221e72f118d2` |

## Vault Research

This project connects to the Response Scoring Framework research in the Obsidian vault:

- **Project home:** `$OBSIDIAN_VAULT_ROOT/01_Projects/Response Scoring Framework/`
- **Framework doc:** `Response Scoring & Competency Framework.md`
- **O*NET/OPM research:** `Research/ONET OPM Smart Street Competency Models Research.md`
- **Lightcast research:** `Research/Lightcast Labor Market Data Research.md`
- **Seniority matching:** `Research/Proposal for a Multidimensional Seniority Matching System.pdf`
- **Stanford SALT Lab:** `Research/Stanford SALT Lab - Future of Work with AI Agents.md`

## Environment

- **Rust edition:** 2021
- **Key deps:** serde, reqwest (rustls), tokio, csv, calamine (Excel), zip, clap, dotenvy
- **Auth:** `.env` file for API keys (Lightcast, etc.) -- gitignored
- **Raw datasets:** `datasets/raw/` is gitignored (large, re-downloadable)
- **Run outputs:** `runs/` is gitignored

## Development Notes

- This is exploration code. Optimize for speed of iteration, not production quality.
- Each dataset module should be self-contained: download raw files, parse them, output normalized JSON/YAML.
- The library schema will evolve as we understand what the datasets actually contain.
- Scoring is Phase 3 -- get the library built first.
