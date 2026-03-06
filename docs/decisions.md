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

## ADR-004: Scoring Prompt Strategy — One Dimension Per Call

**Date:** 2026-03-06
**Status:** Proposed
**Context:**

The scoring pipeline (`src/scoring/`) uses LLM prompts to extract competency signals from responses. Key question: one prompt per competency vs batched extraction.

**Decision:**

Score each competency dimension in a **separate LLM call**, not bundled together.

**Rationale (from assessment methods research):**

- The halo effect is the biggest reliability threat in multi-dimension scoring — LLMs give correlated scores across dimensions when evaluated together (Competency Assessment Methods & Reliability Research, Section 5)
- LLM-as-a-Judge research shows splitting multi-aspect evaluations into separate evaluators and combining results produces better alignment with human judgment (up to 85%, exceeding human-human agreement at 81%)
- AES research found GPT-4 achieves QWK of .57 with humans at temperature 0.0 — "moderate alignment" — which improves with focused single-dimension prompts
- Each scoring call must output three-way signal: **positive / negative / absent**. Absent is explicitly not zero — missing data is not a zero score (psychometric principle)

**Consequences:**

- Higher API cost (N calls per response instead of 1)
- Each competency needs its own BARS-quality rubric with behavioral anchors at each level
- Calibration pipeline needed: 100+ human-scored responses per dimension, measure ICC/Kappa, target AI-human agreement >= human-human agreement
- Must pin model versions and monitor for drift

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

## ADR-006: BARS Rubrics + Calibration Pipeline for Scoring Validity

**Date:** 2026-03-06
**Status:** Proposed
**Context:**

allUP needs to demonstrate that AI competency scores are valid and reliable enough for hiring decisions. The assessment science literature (Competency Assessment Methods & Reliability Research) provides clear methodology.

**Decision:**

Every competency in the library must have a **Behaviorally Anchored Rating Scale (BARS)** rubric (Level 0-4 with specific observable behavioral indicators), and scoring must be validated against human expert baselines via a calibration pipeline.

**Key design constraints from research:**

1. **Rubric design** — Each level differentiated by observable behaviors, not vague descriptors. "Names concrete conflicting goals" beats "Shows some understanding."
2. **Calibration set** — 100+ responses per competency scored by 2-3 human experts. Measure human-human agreement (ICC, Kappa). AI-human agreement must meet or exceed human-human agreement.
3. **Reliability metrics** — Cohen's Kappa >= .60, ICC >= .75, adjacent agreement >= 90%
4. **Bias monitoring** — Four-fifths rule analysis across demographic proxies. DIF analysis on individual scoring dimensions. Intersectional analysis beyond single-axis comparisons.
5. **Drift monitoring** — Pin model versions. Re-score calibration set after model changes. Monitor score distributions over time.

**Rationale:**

- Structured interviews (.42 validity) are now the #1 predictor per Sackett et al. (2022), surpassing cognitive ability tests. allUP's format maps to the highest levels of interview structure.
- HireVue published criterion-related validity of r = .24 (uncorrected) — allUP can exceed this with role-specific competency profiles and BARS-quality rubrics vs. HireVue's generic competencies.
- EEOC Uniform Guidelines and state laws (NYC Local Law 144, California, Colorado) require demonstrable validity and bias auditing for automated hiring tools.

**Consequences:**

- Library schema must include BARS rubric definitions per competency (not just name + description)
- Need human expert scoring infrastructure (even a small calibration set is high-value)
- Scoring outputs must be auditable: input transcript, rubric used, score, evidence citations
- Builds toward publishable validation study (competitive advantage + regulatory defense)

---

## ADR-007: (Placeholder) Versioning Strategy for Library Outputs

**Date:** TBD
**Status:** Draft
**Context:**

As the competency schema evolves, downstream consumers need to know which version of the library produced a given dataset. The `library/v1/` directory suggests versioned outputs, but the full strategy (semver, date-based, content-hash) is not yet defined.

**Decision:** TBD — pending first stable schema.
