
# Competency Assessment Methods & Reliability — The Science of Measuring People Against Frameworks

> **Summary:** Comprehensive research into HOW to reliably assess people against competency frameworks, covering structured interviews, psychometric instruments, reference checking, AI-assisted evaluation, scoring rubric design, and validity/reliability methodology. This builds on the prior O*NET/OPM/Smart & Street research (which covered WHAT to measure) and directly informs how allUP should design its AI-powered scoring operations for video response transcripts.

---

## 1. Structured Interviews & Behavioral Assessment

### Structured vs. Unstructured Interviews: The Core Finding

The single most important finding in personnel selection science is that **structure dramatically improves interview validity**. Multiple meta-analyses converge on this:

| Study | Unstructured Validity | Structured Validity | Ratio |
|-------|----------------------|--------------------| ------|
| Schmidt & Hunter (1998) | .38 | .51 | 1.3x |
| McDaniel et al. (1994) | .33 | .44 | 1.3x |
| Huffcutt & Arthur (1994) | .20 (no structure) | .57 (max structure) | 2.9x |
| Conway, Jako & Goodman (1995) | .34 (low) | .67 (high) | 2.0x |
| **Sackett et al. (2022, revised)** | **.19** | **.42** | **2.2x** |

The Sackett et al. (2022) revision is critical — it corrected systematic overcorrection for range restriction in prior meta-analyses. The core validity estimates dropped by .10-.20 points across the board, but **structured interviews emerged as the single highest-validity predictor**, surpassing cognitive ability tests for the first time in the literature.

**What "structure" means:**
1. **Standardized questions** — all candidates answer the same questions
2. **Job-relevant content** — questions derived from job analysis
3. **Standardized scoring** — anchored rating scales with defined levels
4. **Trained raters** — raters calibrated on what good/bad answers look like
5. **Consistent administration** — same order, same follow-ups

**Implication for allUP:** The video response format is inherently highly structured — everyone answers the same prompt, the scoring rubric is applied consistently by AI, and the questions can be designed around job-relevant competencies. allUP's format maps to the highest levels of interview structure in the Huffcutt & Arthur taxonomy.

### Behavioral Event Interviews (BEI) and the STAR Method

The Behavioral Event Interview (BEI), developed by David McClelland and colleagues at McBer/Hay Group, is a structured interview technique based on the principle that **past behavior is the best predictor of future behavior**. The STAR method (Situation, Task, Action, Result) was introduced by DDI in 1974 as a framework for both asking and evaluating behavioral questions.

**How BEI works:**
- Interviewer asks for specific examples of past behavior related to a competency
- Candidate describes a real situation with concrete details
- Evaluator rates the response against behavioral indicators at defined proficiency levels
- Multiple examples across multiple competencies build a composite picture

**Validity evidence:**
- Research conducted 2003-2011 showed competency-based interviews have a correlation coefficient of r = .802 (p < .001) between evaluators, demonstrating strong inter-rater reliability
- BEI-based competency assessment is one of the most widely validated interview approaches in I/O psychology

**STAR as a scoring framework:**
The STAR method provides natural anchoring points for evaluating response quality:
- **Situation** — Did they describe a specific, real context?
- **Task** — Was their role/responsibility clear?
- **Action** — What specifically did THEY do (not the team)?
- **Result** — Was there a concrete, ideally measurable outcome?

Responses that hit all four elements are inherently higher-evidence than those that stay abstract.

**Implication for allUP:** The STAR framework maps directly to allUP's existing quality dimensions:
- "Detail Level" = Specificity of Situation and Task
- "Story/Narrative" = Presence of all four STAR elements
- "Relevance" = Whether the STAR content addresses the prompt
- "Authenticity" = Whether the details feel genuinely recalled vs. fabricated

allUP should consider designing prompts that explicitly elicit STAR-structured responses, and training scoring operations to detect and reward STAR completeness.

### Behavioral vs. Situational Questions

The evidence on which question type is more valid is nuanced:

| Question Type | How It Works | Best For | Validity |
|--------------|-------------|----------|----------|
| **Behavioral (past behavior)** | "Tell me about a time when..." | Complex, senior roles | Higher validity for managerial/professional positions |
| **Situational (hypothetical)** | "What would you do if..." | Entry-level, standardized roles | Higher overall mean validity across diverse jobs |

Huffcutt et al. (2001) found behavioral description interviews have higher validity for complex roles, while situational interviews are less predictive for higher-level positions. The explanation is that experienced professionals have richer behavioral histories to draw from, while entry-level candidates may not yet have relevant examples.

**Implication for allUP:** For the professional/managerial users allUP targets, behavioral prompts ("Tell me about a time...") are likely more valid than hypothetical ones ("What would you do if..."). This aligns with the existing emphasis on stories and narratives in the quality dimensions.

### Assessment Centers

Assessment centers use multiple methods (role-plays, group exercises, presentations, in-trays, interviews) to evaluate candidates against competency dimensions. Meta-analytic findings:

- Overall Assessment Rating (OAR) corrected validity: **r = .29** for predicting job performance
- Validity is higher when: multiple evaluation devices are used, psychologists serve as assessors (vs. managers), and construct-based dimension scoring replaces holistic ratings
- Single dimensions like "problem solving" can yield comparable predictive utility to full composites when paired with personality or cognitive tests

**Implication for allUP:** Assessment centers' key insight is that **multiple methods measuring the same competency produce more reliable assessment than a single method**. For allUP, this means combining transcript analysis + resume data + video quality signals for the same competency dimension should produce more reliable composite scores than any single source.

---

## 2. Psychometric Assessment Methods

### The Schmidt & Hunter Hierarchy (Revised by Sackett 2022)

The landmark Schmidt & Hunter (1998) meta-analysis ranked 19 selection methods by predictive validity. Sackett et al. (2022) revised these estimates, correcting for systematic overcorrection of range restriction. The updated hierarchy:

| Method | Original (Schmidt & Hunter 1998) | Revised (Sackett et al. 2022) | Notes |
|--------|----------------------------------|-------------------------------|-------|
| **Structured interviews** | .51 | **.42** (highest) | Now the #1 predictor |
| **Work sample tests** | .54 | ~.33 | Reduced but still strong |
| **Cognitive ability (GMA)** | .51 | ~.31 | No longer the clear winner |
| **Job knowledge tests** | .48 | ~.31 | |
| **Integrity tests** | .41 | ~.31 | |
| **Conscientiousness** | .31 | ~.22 | |
| **Reference checks** | .26 | ~.22 | When structured |
| **Unstructured interviews** | .38 | **.19** | Still widely used despite low validity |
| **Job experience (years)** | .18 | ~.13 | Weakest predictor |

**Key insight from Sackett (2022):** Most predictor validities were substantially overestimated in prior work. The rank ordering is largely preserved, but the absolute magnitudes are lower. Structured interviews emerged as the strongest single predictor.

**Sackett also introduced finer-grained categories:**
- Empirically keyed vs. rationally keyed biodata
- Knowledge-based vs. behavioral tendency-based situational judgment tests (SJTs)
- Ability-based vs. personality-based emotional intelligence measures
- Contextualized vs. decontextualized personality measures

These distinctions matter because lumping dissimilar methods together obscures real validity differences.

### Validated Instruments

| Instrument | What It Measures | Reliability | Validity | Notes |
|-----------|-----------------|-------------|----------|-------|
| **Hogan HPI** | Normal personality (7 scales) | .81 test-retest | .29 alone; .54 combined with HDS+MVPI | Based on Big Five; 250+ criterion-related validation studies |
| **Hogan HDS** | Dark-side personality (11 scales) | .70 test-retest | Incremental over HPI | Measures "derailers" — traits that emerge under stress |
| **Hogan MVPI** | Motives, values, preferences | .79 test-retest | Incremental over HPI | Predicts culture fit and job satisfaction |
| **SHL OPQ32** | Personality (32 facets) | Comparable to Hogan | Up to 74% accuracy (SHL claims) | Very granular personality model |
| **Gallup StrengthsFinder** | 34 talent themes | Moderate | Limited criterion-related validation | More developmental than selection-focused |
| **DISC** | 4 behavioral styles | Low-moderate | Low for selection | Better for team dynamics than hiring |

**Key finding:** When the HPI is combined with HDS and MVPI, predictive validity jumps from .29 to .54 — a dramatic increase from combining complementary measures of personality, derailers, and motivation. This supports the multi-signal approach.

### Situational Judgment Tests (SJTs)

SJTs present work-related scenarios and ask candidates to choose among response options. Meta-analytic findings:

- Valid predictors of job performance, particularly for leadership and interpersonal skills
- SJTs measuring teamwork and leadership have relatively high validities
- Provide incremental validity over cognitive ability and personality tests
- Lower adverse impact than cognitive ability tests
- Two types of response instructions: "behavioral tendency" (what would you do?) vs. "knowledge" (what should you do?) — with different construct-level implications

**Implication for allUP:** allUP's prompts function similarly to open-ended SJTs — presenting a work-relevant scenario and asking for a response. The difference is that allUP uses free-form video responses rather than multiple-choice, which should capture richer behavioral signal but is harder to score consistently.

---

## 3. Reference Checking & 360-Degree Feedback

### Making Reference Checks Actually Predictive

Traditional reference checks ("confirm dates and title") have near-zero predictive validity. But structured reference checks show meaningful validity:

**Hedricks et al. (2013) on structured reference checks:**
- Internal consistency: .96-.98 (N = 8,000)
- 2-week test-retest reliability: .74 (N = 1,298)
- Uncorrected correlation with supervisory ratings: **r = .35** (N = 223)

**The Topgrading TORC (Threat of Reference Check) method:**
- Candidates are told upfront that reference checks will be conducted thoroughly
- Candidates arrange their own reference calls (with people they've reported to)
- The "threat" itself acts as a screening mechanism — candidates who fear exposure tend to self-select out
- References are structured around the scorecard competencies, not just "what was their title?"

**What makes reference checks predictive:**
1. **Structured questions** anchored to specific competencies
2. **Behavioral anchoring** — ask for examples, not impressions
3. **Multiple references** — 5-7 per candidate (Smart & Street recommend 7)
4. **Candidate-arranged** — the candidate provides specific references from each past role
5. **Honest signaling** — TORC creates incentives for truthful self-presentation

**Implication for allUP:** The "endorsement" feature (where someone records a video about a candidate) is essentially a video reference check. To maximize validity, endorsement prompts should be structured around specific competencies, not open-ended "tell us about this person." The TORC principle also suggests that merely having the endorsement feature creates a truthfulness incentive for candidates' own responses.

### 360-Degree Feedback

360-degree feedback collects assessments from multiple perspectives (supervisor, peers, direct reports, self). Research findings:

- Kluger & DeNisi (1996) meta-analysis (607 effect sizes, N = 23,663): feedback interventions had a significant overall effect, but **performance actually declined in one-third of studies**
- Self-ratings are generally significantly higher than ratings from others
- 360 assessments with multiple competencies are often highly intercorrelated with a small number of underlying factors
- Most effective when used for development rather than evaluation decisions
- Military experience has raised validity and reliability concerns for employment decisions

**Implication for allUP:** 360-degree feedback's lesson is that **multi-source assessment is valuable but tricky**. The self-report bias in 360 feedback (people rate themselves higher) mirrors the challenge in allUP's video responses — candidates will present themselves favorably. The correction mechanism is cross-referencing with resume data, endorsements, and behavioral evidence specificity.

---

## 4. AI-Assisted Assessment

### Current State of AI in Hiring

The AI hiring assessment market is significant and growing. Key players and research:

**HireVue (Liff et al., 2024 — Journal of Applied Psychology):**
- Convergent validity with target constructs: average r = .58
- Test-retest reliability: average r = .72
- Minimal subgroup differences: Cohen's d >= -.14
- Criterion-related validity (uncorrected): r = .24 across five organizational samples
- Assessment is based on transcripts of video responses (they dropped facial analysis in 2021 after criticism)

**Pymetrics/Harver:**
- Uses 12 neuroscience-based games to measure 90 cognitive, social, and emotional traits
- Open-sourced Audit-AI, a bias auditing tool
- Adopted by Unilever, LinkedIn, Accenture
- Criticism: training data from 30K+ aggregated interviews may not be job-specific; four-fifths rule compliance does not guarantee fairness; concerns about neurodiverse individuals being disadvantaged

### Validity and Bias Concerns

**Known validity issues:**
1. AI systems trained on historical hiring decisions perpetuate existing biases
2. Generic training data combined with generic competencies creates "abstraction of an abstraction" (CDT criticism of HireVue)
3. Most AI hiring vendors treat bias mitigation methods as proprietary, limiting independent verification
4. Four-fifths rule compliance is necessary but not sufficient for fairness

**Key bias risks for AI assessment:**
- **Proxy discrimination** — seemingly neutral features (vocabulary, speaking pace) may correlate with protected characteristics
- **Training data bias** — if "successful" past hires were demographically skewed, the model learns those patterns
- **Construct-irrelevant variance** — accent, dialect, production quality affecting competency scores
- **Intersectional effects** — standard four-fifths testing doesn't catch combined group impacts

### Regulatory Landscape

**Federal (EEOC):**
- May 2022: Technical guidance on AI and ADA compliance
- The EEOC has emphasized that existing anti-discrimination laws (Title VII, ADA, ADEA) apply to AI hiring tools
- January 2025: EEOC removed AI-related guidance from website under new administration — regulatory uncertainty
- Employers must be able to show AI tools are job-related and consistent with business necessity (Uniform Guidelines on Employee Selection Procedures, 1978)

**State-level (more restrictive):**
- **New York City (Local Law 144):** Requires bias audits of automated employment decision tools, published annually
- **Illinois:** Requires disclosure and consent for AI video interview analysis
- **California:** Most detailed — unlawful to use automated-decision systems that discriminate; requires meaningful human oversight, proactive bias testing, 4-year record retention
- **Colorado:** First state to enact broad algorithmic bias legislation for consequential decisions

**SIOP Guidance (2023):**
"AI-based assessments used to make hiring and promotion decisions require the same level of scrutiny and should meet the same standards that traditional employment tests have been subjected to for decades."

### LLMs for Evaluating Interview Responses

This is the most directly relevant area for allUP. Current research:

**LLM-as-a-Judge framework:**
- LLMs assess quality of outputs based on predefined criteria
- Sophisticated judge models can align with human judgment up to **85%**, which exceeds typical human-to-human agreement (81%)
- Best practices: write precise evaluation prompts; split multi-aspect evaluations into separate evaluators; combine results for overall judgment

**LLM-as-an-Interviewer (2024 research, arXiv 2412.10424):**
- Novel paradigm using multi-turn interactions where the LLM provides feedback and follow-up questions
- Evaluates initial response quality, adaptability to feedback, and ability to address follow-ups
- Early-stage but directly relevant to allUP's potential for adaptive follow-up prompting

**Automated Essay Scoring (AES) as precedent:**
- AES systems are considered reliable when computer scores agree with one human rater as well as raters agree with each other
- Key metrics: Quadratic Weighted Kappa (QWK), Mean Absolute Error (MAE), Pearson Correlation Coefficient (PCC)
- GPT-4 achieved QWK of .57 with human evaluators at temperature 0.0 — "moderate alignment"
- AES less reliable than human readers for complex assessments
- High-stakes assessments (GMAT) still require at least one human scorer

**Implication for allUP:** The LLM-as-a-Judge research validates allUP's approach of using AI to evaluate responses. The 85% human agreement rate is an achievable benchmark. allUP should:
1. Establish human expert baseline scores for a calibration set
2. Measure AI-human agreement using QWK and ICC
3. Aim for AI-human agreement >= human-human agreement
4. Split scoring into separate dimensions (as the research recommends) rather than single holistic scores
5. Use temperature 0 for scoring consistency

---

## 5. Designing Reliable Scoring Rubrics

### Behaviorally Anchored Rating Scales (BARS)

BARS are the gold standard for reliable competency assessment rubrics. They use specific behavioral examples as anchoring points rather than generic descriptors.

**BARS development process:**
1. **Critical Incident Technique** — Collect examples of effective and ineffective behavior from subject matter experts
2. **Q-sort** — Sort incidents into homogeneous behavioral dimensions
3. **Retranslation** — Have a second group sort incidents to verify dimension assignment (incidents that don't achieve consensus are dropped)
4. **Scaling** — Rate remaining incidents on a numerical scale to create anchor points
5. **Validation** — Test the scale's reliability with independent raters

**Reliability evidence:**
- Intra-rater reliability generally exceeds inter-rater reliability for BARS
- Inter-rater reliability is moderate — exceeded 0.5 for only one measure in some medical education studies
- Regular calibration sessions significantly improve consistency

**BARS vs. allUP's current rubric approach:**

allUP's Stakeholder Communication rubric in the Competency Graph Vision document already follows BARS principles:
- Level 0-4 scale
- Each level defined by observable behavioral indicators
- Specific language about what differentiates levels

This is exactly the right approach. The research validates it. The challenge is scaling this to all competency dimensions.

### What Makes a Rubric Reliable

1. **Behavioral specificity** — "Names concrete conflicting goals" beats "Shows some understanding"
2. **Clear level differentiation** — Raters should be able to distinguish adjacent levels without difficulty
3. **Exhaustive coverage** — Every reasonable response should map to exactly one level
4. **Calibration training** — Raters (or AI prompts) trained on exemplar responses at each level
5. **Separate dimensions** — Score each competency independently; composite scores come later
6. **"No signal" as a valid rating** — Absence of evidence is not evidence of absence (see below)

### Calibrating AI Against Human Expert Ratings

**The calibration process for AI scoring:**

1. **Build a calibration set** — 100+ responses per competency dimension, scored by 2-3 expert human raters
2. **Measure human agreement** — Calculate ICC and Cohen's Kappa among human raters
3. **Score with AI** — Run the same responses through the AI scoring operation
4. **Measure AI-human agreement** — Calculate the same metrics (ICC, Kappa) between AI and each human
5. **Benchmark target:** AI-human agreement >= average human-human agreement
6. **Error analysis** — Categorize disagreements to identify systematic AI biases
7. **Iterate** — Refine prompts and rubric definitions based on error patterns
8. **Drift monitoring** — Periodically re-score calibration set to detect prompt/model drift

**Key metrics:**

| Metric | What It Measures | Target |
|--------|-----------------|--------|
| **Cohen's Kappa** | Agreement between two raters, adjusted for chance | >= .60 (substantial) |
| **ICC (Intra-class Correlation)** | Consistency among 2+ raters for ordinal/interval data | >= .75 (good) |
| **QWK (Quadratic Weighted Kappa)** | Agreement weighted by distance between ratings | >= .60 |
| **Pearson r** | Linear correlation between AI and human scores | >= .70 |
| **Exact agreement rate** | Percentage of identical scores | >= 60% |
| **Adjacent agreement rate** | Percentage within 1 level | >= 90% |

### Handling "No Signal" (Absent Evidence)

This is a critical design challenge allUP has already identified. The research supports three-way signal classification:

**The measurement theory distinction:**
- **Positive signal** — Response demonstrates the competency
- **Negative signal** — Response demonstrates lack of the competency (e.g., describes a situation where they failed at the competency)
- **Absent signal** — Response simply does not address this competency

**Why this matters:**
In psychometrics, **missing data is not the same as a zero score**. When a test item is "not reached" or "omitted," ability estimates become biased if the missing response is treated as incorrect. The same applies to competency assessment from video responses.

**How to handle absent signal:**
1. **Do not penalize** — An absent signal should not lower a competency score
2. **Track explicitly** — Record "no signal" as a distinct value (not null, not zero)
3. **Use for prompting** — Absent signals on high-priority competencies become "next best question" suggestions
4. **Aggregate carefully** — When computing profile-level scores, weight by number of signals, not number of responses
5. **Report confidence** — Fewer signals = lower confidence, even if all signals are positive

**Implication for allUP:** The existing confidence model (direction + confidence + evidence) already handles this well conceptually. The implementation should ensure that scoring operations explicitly output "absent" when a competency is not addressed, rather than forcing a low score.

---

## 6. Validity and Reliability in Assessment

### Types of Validity

| Type | Definition | How to Establish | Importance for allUP |
|------|-----------|-----------------|---------------------|
| **Content validity** | Test items reflect job-relevant knowledge/skills | Job analysis + expert review of rubric alignment | HIGH — prompts must target real job competencies |
| **Construct validity** | Test measures what it claims to measure | Convergent/discriminant validity studies; factor analysis | HIGH — do scoring operations actually measure the competency they claim? |
| **Criterion validity** | Test scores predict job performance | Concurrent (current employees) or predictive (hiring outcomes) studies | HIGHEST — the ultimate question: do allUP scores predict on-the-job success? |

**For allUP specifically:**
- **Content validity** can be established now by mapping prompts to O*NET competency definitions and having I/O psychologists review rubrics
- **Construct validity** can be established by showing that different prompts targeting the same competency produce correlated scores, and that prompts targeting different competencies produce divergent scores
- **Criterion validity** requires hiring outcome data — this is the hardest to establish and requires tracking which scored candidates get hired and perform well

### Inter-Rater Reliability

**Common measures:**

| Measure | When to Use | Range | Interpretation |
|---------|------------|-------|---------------|
| **Cohen's Kappa** | 2 raters, categorical data | -1 to 1 | <.20 poor, .21-.40 fair, .41-.60 moderate, .61-.80 substantial, .81-1.0 near-perfect |
| **Fleiss' Kappa** | 3+ raters, categorical data | -1 to 1 | Same scale as Cohen's |
| **ICC** | 2+ raters, ordinal/interval data | 0 to 1 | <.50 poor, .50-.75 moderate, .75-.90 good, >.90 excellent |
| **Krippendorff's Alpha** | Any number of raters, any scale | 0 to 1 | <.667 discard, .667-.800 tentative, >.800 reliable |

**Establishing inter-rater reliability for allUP's AI scoring:**
- Treat the AI as one "rater" and human experts as other "raters"
- Calculate pairwise agreement metrics
- The AI is reliable if its agreement with any one human >= average human-human agreement

### Minimum Sample Sizes for Validation

| Validation Type | Minimum N | Recommended N | Notes |
|----------------|----------|--------------|-------|
| **Calibration set per competency** | 50 | 100+ | Need sufficient range of quality levels |
| **Criterion validity study** | 100 events + 100 non-events | 200+ per group | Need enough hiring outcomes |
| **Adverse impact analysis** | Varies by group size | 80+ per demographic group | Four-fifths rule requires adequate representation |
| **Factor analysis (construct validity)** | 10x the number of variables | 300+ | To confirm rubric dimensions are distinct |

**Practical reality for allUP:** With 25K candidates and 250K responses, allUP has more than enough volume for calibration and construct validity. The bottleneck for criterion validity is hiring outcome data — allUP needs to track what happens after candidates are surfaced to employers.

### Detecting and Controlling for Bias

**The Four-Fifths Rule (EEOC):**
Selection rate for any group must be at least 80% of the rate for the group with the highest selection rate. Example: if 10% of white candidates pass and 5% of Black candidates pass, that's 50% — well below the 80% threshold, indicating adverse impact.

**Beyond the four-fifths rule:**
- **Z-test / Fisher's Exact Test** — Statistical significance of group differences
- **Differential Item Functioning (DIF)** — Whether individual scoring dimensions behave differently across demographic groups
- **Intersectional analysis** — Testing for combined effects (e.g., Black women vs. white men) beyond single-axis comparisons

**Common rater biases (also apply to AI):**

| Bias | Description | AI Risk | Mitigation |
|------|------------|---------|-----------|
| **Halo effect** | One strong trait inflates all ratings | HIGH — LLMs may rate all dimensions similarly based on overall response quality | Score dimensions independently in separate prompts |
| **Leniency/severity** | Systematic over/under-rating | MODERATE — prompt engineering can drift | Calibration checks against human expert baselines |
| **Central tendency** | Avoiding extreme ratings | HIGH for LLMs — they often hedge toward middle scores | Include extreme exemplars in rubric definitions; monitor score distributions |
| **Similarity bias** | Favoring people "like me" | PRESENT — LLMs trained on dominant culture text may favor certain communication styles | Audit across demographic groups; test for vocabulary/dialect effects |
| **Construct-irrelevant variance** | Non-competency factors affecting scores | HIGH — accent, fluency, production quality contaminating competency scores | Separate quality dimensions from competency dimensions; explicitly instruct AI to ignore surface features |

---

## 7. Implications for allUP's Approach

### How allUP's Format Compares to Traditional Methods

allUP's video response format is a hybrid of:
- **Asynchronous video interview (AVI)** — one-way, recorded, standardized
- **Situational judgment test** — work-relevant prompt requiring a substantive response
- **Behavioral interview** — elicits past experience through STAR-type prompts
- **Work sample** — the response itself demonstrates communication competency

**Structural advantages:**
- AVIs have inherent structure advantages — standardized content, consistent administration
- Interview ratings from video are less inflated than face-to-face ratings (better psychometric properties)
- AI scoring eliminates inter-rater variability (perfect reliability within a single model run)
- Scale: can assess 250K responses consistently, impossible with human interviewers

**Structural disadvantages:**
- No follow-up questioning (traditional interviews can probe deeper)
- One-sided presentation (no interviewer to calibrate difficulty)
- Limited research on AVI psychometric properties specifically (the literature is still nascent)
- Candidates may game the format once patterns become known

**Expected validity range:**
Based on the literature, allUP's approach should fall between structured interviews (.42) and HireVue's automated assessment (.24) in criterion validity. The key differentiator will be **prompt design quality** and **rubric specificity**. HireVue's criticism centers on generic competencies and generic training data — allUP can do better by:
1. Using O*NET-linked, role-specific competency profiles
2. Building rubrics with BARS-quality behavioral anchors
3. Establishing human expert calibration baselines per competency
4. Tracking criterion validity through hiring outcomes

### Designing Prompts (Lessons from BEI/STAR)

The research is clear: **how you ask the question determines how much signal you get.**

**High-signal prompt characteristics:**
1. **Behavioral, not hypothetical** (for experienced professionals): "Tell me about a time when..." beats "What would you do if..."
2. **Specific competency targeting**: Each prompt should map to 1-2 primary competencies
3. **STAR elicitation**: Structure the prompt to invite all four elements
4. **Difficulty calibration**: Questions should differentiate between proficiency levels, not just detect presence/absence
5. **Minimal coaching**: Don't tell candidates what you're looking for (reduces social desirability bias)

**Prompt design template (derived from research):**

```
Target competency: [O*NET-mapped competency]
Question type: Behavioral / Situational / Values-based
STAR elicitation: Does it naturally prompt Situation, Task, Action, Result?
Difficulty level: Entry / Mid / Senior / Executive
Expected differentiation: What separates a Level 2 from Level 3 response?
```

**Prompt-to-competency mapping quality metrics:**
- Over time, measure which prompts produce the most signal (highest variance in scores)
- Low-signal prompts (everyone scores Level 2) should be revised or retired
- High-signal prompts (good distribution across levels) should be promoted

### Combining Multiple Signals

The research on incremental validity shows that **combining multiple valid predictors improves prediction, but the combination method matters**:

- Cognitive ability (.51) + work sample (.54) = combined validity of .63 (24% increase)
- HPI alone (.29) + HDS + MVPI = combined validity of .54 (86% increase)
- But: regression-weighted composites outperform unit weighting and multiple hurdles
- And: in some cases, a second predictor can decrease validity if the combination method is wrong

**For allUP, the signal sources are:**

| Source | What It Captures | Expected Validity Contribution |
|--------|-----------------|-------------------------------|
| **Transcript content** | Competencies, experience, motivations, achievements | PRIMARY — richest signal |
| **Resume data** | Credentials, timeline, titles, skills listed | COMPLEMENTARY — fills gaps in transcript signal |
| **Response quality** | Communication ability, professionalism | MODERATE — directly observable competency |
| **Video signals** | Presence, energy, production quality | SUPPLEMENTARY — contributes to overall impression |
| **Cross-response consistency** | Pattern reliability across multiple responses | CONFIDENCE BOOSTER — multiple consistent signals increase confidence |
| **Endorsements** | External validation of competencies | HIGH VALUE — equivalent to structured reference check |

**Recommended combination approach:**
1. Score each dimension independently from each source
2. Weight by source reliability (transcript > resume > video for most competencies)
3. Use consistency across sources as a confidence multiplier
4. Do NOT combine into a single composite score for everything — maintain dimension-level profiles
5. Composite scores only at match-time (Layer 3), when you know what dimensions matter for the role

### Biggest Validity Threats and Mitigations

| Threat | Risk Level | Mitigation |
|--------|-----------|-----------|
| **Construct-irrelevant variance** — Surface features (accent, vocabulary sophistication, production quality) contaminating competency scores | HIGH | Separate quality dimensions from signal dimensions (already in the framework); instruct AI to ignore surface features; audit scores by demographic proxies |
| **Social desirability bias** — Candidates presenting idealized versions of themselves | HIGH | Triangulate with resume data; reward specificity (vague answers get low confidence); endorsements provide external validation; TORC-like incentive from knowing endorsements exist |
| **Prompt-competency misalignment** — Prompts that don't actually elicit the target competency | MODERATE | Construct validity studies (do different prompts targeting the same competency correlate?); expert review of prompt-rubric alignment |
| **AI scoring inconsistency across model versions** — Scores change when the underlying LLM is updated | HIGH | Pin model versions; maintain calibration sets; re-validate after model changes; monitor score distributions over time |
| **Halo effect in AI scoring** — LLM gives correlated scores across dimensions for the same response | HIGH | Score dimensions in separate API calls; explicitly instruct to ignore other dimensions; measure inter-dimension correlations and flag if too high |
| **Range restriction** — allUP's self-selected user base may not represent full applicant range | MODERATE | Acknowledge in validity claims; use within-group norming; expand user base diversity |
| **Criterion contamination** — If employers see allUP scores before hiring, the scores influence the criterion | MODERATE | Track whether employers viewed scores before hiring decision; control for in validity analyses |
| **Faking/coaching** — Candidates learn what "good" answers sound like | MODERATE-HIGH | Vary prompts; look for authentic markers (filler words, self-corrections); cross-reference with resume; flag suspiciously polished responses |

---

## Connections to Existing Vault Notes

- **Response Scoring & Competency Framework** — The master framework this research directly supports. The quality vs. signal distinction, confidence model, and three-layer architecture are all validated by the assessment science literature.
- **ONET OPM Smart Street Competency Models Research** — The "what to measure" complement to this "how to measure" research. That note's gap analysis and recommended additions should be implemented using the rubric design principles described here.
- **Competency Graph Vision** — The Stakeholder Communication rubric example already follows BARS principles. The Prompt-to-Competency Map building block maps to this research's prompt design guidance. The Evidence Engine maps to the calibration and scoring methodology. The Continuous Feedback Loops building block maps to the validity monitoring framework.
- **Job Fit Tools** — The Hiring Process Designer tool should incorporate structured interview and BARS rubric design. The Candidate Fit Analyzer should use the competency matching methodology grounded in validated assessment science.
- **How to Hire** (Vinod Khosla clipping) — Khosla's emphasis on "value creation vs. value protection" roles aligns with the behavioral vs. trait distinction in assessment. His "growth velocity over experience" maps to Smart & Street's outcomes-first approach.
- **Lessons from Keith Rabois Essay 2 How to Interview an Executive** — Rabois's executive assessment traits (ownership mentality, strategic thinking, talent magnetism) map directly to OPM ECQ competencies that this research identifies as highly measurable from video transcripts.

---

## Sources

### Meta-Analyses & Foundational Research
- [Schmidt & Hunter (1998) — Validity and Utility of Selection Methods (85 Years of Research)](https://home.ubalt.edu/tmitch/645/session%204/Schmidt%20&%20Oh%20validity%20and%20util%20100%20yrs%20of%20research%20Wk%20PPR%202016.pdf) — The landmark meta-analysis ranking 19 selection methods
- [Sackett et al. (2022) — Revisiting Meta-Analytic Estimates of Validity in Personnel Selection](https://psycnet.apa.org/record/2022-17327-001) — Critical revision correcting range restriction overcorrection
- [Sackett et al. (2022) — Revisiting Selection Systems Design](https://www.cambridge.org/core/journals/industrial-and-organizational-psychology/article/revisiting-the-design-of-selection-systems-in-light-of-new-findings-regarding-the-validity-of-widely-used-predictors/A20984B138319E3D432E643978BF026D) — Companion paper on implications for selection system design
- [SIOP Summary — Updated Validity Estimates](https://www.siop.org/tip-article/is-cognitive-ability-the-best-predictor-of-job-performance-new-research-says-its-time-to-think-again/) — Accessible summary of Sackett et al. findings
- [Criteria Corp — Updated Employee Selection Science](https://www.criteriacorp.com/blog/updates-employee-selection-science) — Practitioner-friendly summary of revised validity estimates

### Structured Interviews
- [McDaniel et al. (1994) — Criterion Validity of Interviews Meta-Analysis](https://home.ubalt.edu/tmitch/645/articles/McDanieletal1994CriterionValidityInterviewsMeta.pdf) — Journal of Applied Psychology meta-analysis
- [McGill — Structured vs. Unstructured Interviews](https://www.mcgill.ca/psychology/files/psychology/structuredinterviews.pdf) — Research overview on improving accuracy
- [Wingate (2025) — Interview Criterion-Related Validity for Distinct Constructs](https://onlinelibrary.wiley.com/doi/10.1111/ijsa.12494) — Latest meta-analysis on interview construct validity
- [Huffcutt et al. (2001) — Behavioral vs. Situational Questions for Higher-Level Positions](https://onlinelibrary.wiley.com/doi/abs/10.1111/j.1744-6570.2001.tb00225.x) — Behavioral questions more valid for complex roles

### Behavioral Event Interviews & STAR
- [McClelland (1998) — Identifying Competencies with Behavioral-Event Interviews](https://skillup.coreconsulting.it/home/pdfs/McClelland,%20D.%20C.%20(1998).%20Identifying%20competencies%20with%20behavioral-event%20interviews..pdf) — Foundational BEI methodology
- [DDI — STAR Method for Behavioral Interviewing](https://www.ddi.com/solutions/behavioral-interviewing/star-method) — The originators of STAR (1974)
- [OPM — Structured Interviews](https://www.opm.gov/policy-data-oversight/assessment-and-selection/other-assessment-methods/structured-interviews/) — Federal government structured interview guidance

### Psychometric Instruments
- [Hogan Assessments — Validity and Reliability Guide](https://www.hoganassessments.com/blog/quick-dirty-guide-validity-reliability/) — Hogan's own reliability/validity evidence
- [Hogan Personality Inventory Technical Manual](https://www.crownedgrace.com/wp-content/uploads/2016/04/Hogan-Personality-Inventory.pdf) — Detailed psychometric properties
- [Master International — Insights from Sackett et al. (2022, 2023)](https://www.master-hr.com/insights/insights-from-sackett-et-al-2023/) — Updated validity estimates for instruments

### AI in Hiring Assessment
- [Liff et al. (2024) — Psychometric Properties of Automated Video Interview Competency Assessments](https://www.hirevue.com/wp-content/uploads/2024/03/Psychometric-Properties-of-Automated-Video-Interview-Competency-Assessments-Liff-et-al.-2024.pdf) — HireVue's published validation study (Journal of Applied Psychology)
- [SIOP (2023) — Considerations for Validation of AI-Based Assessments](https://www.siop.org/wp-content/uploads/2024/06/Considerations-and-Recommendations-for-the-Validation-and-Use-of-AI-Based-Assessments-for-Employee-Selection-January-2023.pdf) — Professional guidelines for AI assessment validation
- [CDT — HireVue AI Explainability Statement Critique](https://cdt.org/insights/hirevue-ai-explainability-statement-mostly-fails-to-explain-what-it-does/) — Critical analysis of AI assessment transparency
- [LLM-as-an-Interviewer (2024, arXiv 2412.10424)](https://arxiv.org/abs/2412.10424) — Research on multi-turn LLM interview evaluation

### Asynchronous Video Interviews
- [Dunlop et al. (2025) — Asynchronous Video Interviews in Recruitment and Selection](https://onlinelibrary.wiley.com/doi/10.1111/ijsa.70010) — IJSA special issue overview
- [Beyond Traditional Interviews — Psychometric Analysis of AVIs](https://www.sciencedirect.com/science/article/pii/S074756322300479X) — NLP personality prediction from async video

### Scoring Rubrics & BARS
- [PMC — Reliability of BARS for Assessing Non-Technical Skills](https://pmc.ncbi.nlm.nih.gov/articles/PMC9090385/) — BARS reliability evidence
- [AIHR — Behaviorally Anchored Rating Scale Guide](https://www.aihr.com/blog/behaviorally-anchored-rating-scale/) — Practitioner guide to BARS development
- [PMC — Harnessing LLMs for Multi-Dimensional Writing Assessment](https://pmc.ncbi.nlm.nih.gov/articles/PMC11305227/) — LLM scoring reliability and human alignment

### Reference Checking
- [Themetiss Group — Strategic Value of Reference Checks](https://www.themetissgroup.com/blog/reference-advantage) — Evidence-based reference checking
- [Brad Smart — The TORC Technique](https://www.linkedin.com/pulse/torc-technique-powerful-free-hiring-method-brad-smart) — Topgrading reference check method

### Regulatory & Bias
- [EEOC — Uniform Guidelines on Employee Selection Procedures (Q&A)](https://www.eeoc.gov/laws/guidance/questions-and-answers-clarify-and-provide-common-interpretation-uniform-guidelines) — Four-fifths rule and selection procedure requirements
- [ABA — Navigating the AI Employment Bias Maze](https://www.americanbar.org/groups/business_law/resources/business-law-today/2024-april/navigating-ai-employment-bias-maze/) — Legal compliance guidelines
- [ABA — EEOC and States Regulation of Algorithmic Bias](https://www.americanbar.org/groups/business_law/resources/business-lawyer/2024-2025-winter/eeoc-states-regulation-algorithmic-bias-high-risk/) — Comprehensive regulatory overview
- [Fisher Phillips — Comprehensive Review of AI Workplace Law (2025)](https://www.fisherphillips.com/en/news-insights/comprehensive-review-of-ai-workplace-law-and-litigation-as-we-enter-2025.html) — Current legal landscape
- [HR Defense Blog — AI in Hiring: Legal Developments for 2026](https://www.hrdefenseblog.com/2025/11/ai-in-hiring-emerging-legal-developments-and-compliance-guidance-for-2026/) — Forward-looking regulatory guidance

### Validity & Reliability Methodology
- [OPM — Assessment Strategy Design](https://www.opm.gov/policy-data-oversight/assessment-and-selection/assessment-strategy/) — Federal government assessment best practices
- [SIOP — Principles for Validation and Use of Personnel Selection Procedures (5th ed.)](https://www.apa.org/ed/accreditation/personnel-selection-procedures.pdf) — The definitive professional guidelines
- [Criteria Corp — Validity of Pre-Employment Tests](https://www.criteriacorp.com/resources/definitive-guide-validity-of-preemployment-tests/validity-pre-employment-tests) — Content, construct, and criterion validity explained

---

## Specific Recommendations for allUP's Scoring Operation Design

### Immediate Actions (This Quarter)

1. **Adopt BARS methodology for all scoring rubrics** — Every competency scoring operation should have Level 0-4 behavioral anchors with specific observable indicators, following the Stakeholder Communication example in Competency Graph Vision.

2. **Build a calibration pipeline** — For each scoring operation:
   - Have 2-3 domain experts score 100+ responses
   - Measure human-human agreement (ICC, Kappa)
   - Measure AI-human agreement against the same calibration set
   - Target: AI-human agreement >= human-human agreement

3. **Score dimensions separately** — Each competency dimension should be scored in a separate AI call, not bundled. This reduces halo effect and allows independent validation per dimension.

4. **Implement three-way signal classification** — Every scoring operation must output direction (positive/negative/absent), not just a numeric score. Absent is explicitly not zero.

5. **Design prompts using BEI/STAR principles** — Map each prompt to 1-2 target competencies. For professional users, favor behavioral ("Tell me about a time...") over situational ("What would you do if...") framing.

### Medium-Term (Next 2 Quarters)

6. **Establish construct validity** — Show that different prompts targeting the same competency produce correlated scores (convergent validity) and that prompts targeting different competencies produce divergent scores (discriminant validity).

7. **Begin adverse impact monitoring** — Track score distributions by available demographic proxies. Apply four-fifths rule analysis. Conduct DIF analysis on individual scoring dimensions.

8. **Build the feedback loop** — Track which scored candidates get hired, and eventually, which perform well. This is the path to criterion validity, which is the ultimate validation.

9. **Model version pinning and drift monitoring** — Pin LLM versions for scoring operations. Maintain calibration sets. Re-validate after any model change. Monitor score distributions over time for drift.

### Long-Term (6+ Months)

10. **Criterion validity study** — Once you have enough hiring outcome data (100+ hires with performance data), conduct a formal criterion validity study correlating allUP scores with job performance.

11. **Publish validation research** — Following HireVue's example (Liff et al., 2024), publish allUP's validation evidence in a peer-reviewed journal. This becomes both a competitive advantage and a regulatory defense.

12. **Consider I/O psychology advisory board** — Engage 1-2 I/O psychologists to review rubrics, validate methodology, and advise on regulatory compliance. This is both scientifically valuable and provides legal defensibility.

---

## Open Questions for Deeper Research

1. **What is the optimal number of prompts per competency to achieve reliable assessment?** The 360 feedback literature suggests diminishing returns after 5-7 items. For video responses, the answer may be lower (2-3?) given response richness.

2. **How does response length interact with scoring reliability?** Very short responses (<30 seconds) may not provide enough signal. Very long responses (>3 minutes) may introduce noise. What's the optimal range?

3. **Can cross-response consistency be used as a reliability indicator?** If someone scores high on "accountability" in one response and low in another, is the average more or less reliable than a single high-quality response?

4. **What is the adverse impact profile of allUP's current scoring?** This is an empirical question that requires demographic data analysis.

5. **How do prompted vs. unprompted competency signals differ in validity?** A response to a targeted prompt should yield stronger signal, but unprompted signals (someone spontaneously demonstrates leadership while answering a technical question) may be more authentic.

6. **What's the right approach for scoring endorsement videos differently from self-presentations?** The reference checking literature suggests structured questions and behavioral anchoring — but the endorser's credibility and relationship to the candidate also matter.

7. **How should allUP handle candidates who clearly coach their responses?** The "scripted detection" heuristics in the framework are a start, but the line between preparation and gaming is blurry.

---

*Research conducted 2026-03-06*
