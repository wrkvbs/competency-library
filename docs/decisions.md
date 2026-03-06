# Architecture Decision Records

Significant design decisions for the competency-library, recorded as lightweight ADRs.

---

## ADR-001: Tags vs Competency Scores

**Date:** 2026-03-06
**Status:** Accepted
**Context:**

allUP needs to represent what a person is good at. Two approaches were considered:

1. **Tags** — Binary labels attached to a person (e.g., "leadership", "python", "strategic-thinking"). Simple to implement, easy to search with exact match or keyword queries.

2. **Competency scores** — Numeric or multi-dimensional assessments per competency (e.g., `leadership: { direction: positive, confidence: 0.82, evidence: [...] }`). Richer signal, supports ranking and nuanced matching.

**Decision:**

Use **competency scores**, not flat tags.

**Rationale:**

- Tags lose the signal that makes allUP's data valuable. A tag saying someone has "leadership" treats a first-time team lead and a VP of Engineering identically.
- The Response Scoring Framework defines three signal properties for every competency observation: **direction** (positive/negative/absent), **confidence** (how much to trust it), and **evidence** (the source content). Tags cannot represent any of these.
- Confidence is a product lever — low-confidence positive signals become prompts for the user to record more responses, creating an engagement loop. This requires numeric representation.
- Search ranking needs continuous scores to order results, not just filter them. "Show me the best leaders who also know distributed systems" requires comparing magnitudes, not just presence.
- Multiple observations of the same competency compound confidence over time. Tags have no mechanism for this.

**Consequences:**

- Schema is more complex than a simple tag list — each competency entry carries direction, confidence, and evidence references.
- Scoring pipeline must produce structured outputs, not just labels.
- Search/matching becomes a vector similarity or weighted scoring problem rather than set intersection.
- Storage and indexing requirements are higher than flat tags.

---

## ADR-002: Unified Schema Across External Taxonomies

**Date:** 2026-03-06
**Status:** Proposed
**Context:**

The library ingests competency data from multiple external sources (O\*NET, OPM MOSAIC, Lightcast, WORKBank) that use different taxonomies, granularities, and naming conventions.

**Decision:**

Define a single internal competency schema (`src/library/schema.rs`) that all external sources normalize into, rather than preserving each source's native taxonomy.

**Rationale:**

- Downstream consumers (execution-graph, scoring pipeline) need one interface, not four.
- Cross-source deduplication and merging requires a common representation.
- O\*NET skills, OPM competencies, and Lightcast skills often describe the same underlying ability with different names — normalization surfaces these overlaps.

**Consequences:**

- Each dataset module must implement a normalization step.
- Some source-specific nuance may be lost in translation.
- The schema must be flexible enough to accommodate all sources without becoming a lowest-common-denominator.

---

## ADR-003: (Placeholder) Competency Hierarchy Depth

**Date:** TBD
**Status:** Draft
**Context:**

How many levels of hierarchy should the competency library support? O\*NET uses a relatively flat model (abilities, skills, knowledge as peer categories). OPM uses a deeper tree. We need to decide on a target depth.

**Decision:** TBD — pending taxonomy exploration in Phase 2.

---

## ADR-004: (Placeholder) Scoring Prompt Strategy

**Date:** TBD
**Status:** Draft
**Context:**

The scoring pipeline (`src/scoring/`) uses LLM prompts to extract competency signals from responses. Key open questions: one prompt per competency vs batched extraction, structured output format, model selection.

**Decision:** TBD — pending scoring experiments.

---

## ADR-005: JD-to-Search via Competency Library

**Date:** 2026-03-06
**Status:** Accepted
**Context:**

allUP needs to connect employers (who have job descriptions) with candidates (who have video responses). Today, search is keyword-based against transcripts. We need a structured matching layer where both sides speak the same language.

**Decision:**

The competency library serves as a **shared vocabulary** between the supply side (candidate scoring) and the demand side (JD parsing). The same competency definitions used to score responses are used to extract requirements from job descriptions.

Given a JD, the system:
1. Extracts required competencies using LLM + library definitions as grounding
2. Classifies each as required vs. preferred, with importance weights
3. Infers seniority signals from scope/responsibility language
4. Generates a structured search query against candidates' competency scores

**Rationale:**

- Using the library as a controlled vocabulary prevents vocabulary mismatch between JD language and candidate scoring language. "Strong communicator" in a JD maps to the same competency as "demonstrates clear verbal articulation" extracted from a response.
- Grounding extraction in library definitions (not freeform labels) keeps the search space bounded and rankable.
- Lightcast's 33k skills taxonomy is especially useful here — JDs reference specific technologies and tools that need to map to the same skill IDs used in candidate profiles.
- This validates the library design: if a competency can't be meaningfully extracted from both a JD and a video response, it's probably the wrong granularity.

**Consequences:**

- Library definitions must include enough context for LLM-based extraction from both JDs and transcripts.
- The schema needs fields for "observability" — can this competency be assessed from a video response, from a JD, or both?
- Search becomes a weighted vector match: JD competency weights x candidate competency scores.
- JD parsing is a new CLI subcommand and module (`src/jd/` or similar).

---

## ADR-006: (Placeholder) Versioning Strategy for Library Outputs

**Date:** TBD
**Status:** Draft
**Context:**

As the competency schema evolves, downstream consumers need to know which version of the library produced a given dataset. The `library/v1/` directory suggests versioned outputs, but the full strategy (semver, date-based, content-hash) is not yet defined.

**Decision:** TBD — pending first stable schema.
