#!/usr/bin/env python3
"""
Competency/Skill Extraction Explorer

Local tool for iterating on how we extract skills and competencies from
allUP video response transcripts. Follows the seniority-profile pattern:
single LLM call, score + confidence + evidence per dimension.

Usage:
    # Score all transcripts
    uv run --with openai tools/explore_scoring.py

    # Score a specific transcript by index (0-based)
    uv run --with openai tools/explore_scoring.py --index 0

    # Score a specific transcript by response ID
    uv run --with openai tools/explore_scoring.py --id 604a7a4e-7fe2-45fb-bb4f-71607b6a7205

    # Use a different prompt version
    uv run --with openai tools/explore_scoring.py --prompt v2

    # Compare two prompt versions
    uv run --with openai tools/explore_scoring.py --compare v1 v2
"""

import argparse
import json
import os
import sys
import time
from pathlib import Path

from openai import OpenAI

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------

ROOT = Path(__file__).resolve().parent.parent
TRANSCRIPTS_PATH = ROOT / "datasets" / "raw" / "allup" / "transcripts.json"
RUNS_DIR = ROOT / "runs" / "explore"

# ---------------------------------------------------------------------------
# Prompt Templates
# ---------------------------------------------------------------------------

# Each version is a (system_prompt, user_template) tuple.
# user_template gets .format(prompt_text=..., transcript=..., responder=..., context=...)

PROMPTS = {}

PROMPTS["v1"] = (
    # --- System prompt (cacheable) ---
    """\
You are an expert talent evaluator. You analyze video response transcripts to extract two types of signals:

## Layer 1: Behavioral Competencies

These describe HOW someone works, observable from how they talk about their experience. Score each that you detect:

- **problem_solving** — Breaks down problems, considers tradeoffs, reasons through complexity
- **communication** — Articulates ideas clearly, structures narratives, adapts to audience
- **leadership** — Takes ownership, influences others, drives outcomes, mentors
- **collaboration** — Works across teams, builds relationships, navigates stakeholders
- **initiative** — Self-directed, identifies opportunities, acts without being told
- **adaptability** — Handles ambiguity, pivots when needed, learns from setbacks
- **analytical_thinking** — Uses data, measures outcomes, makes evidence-based decisions
- **customer_focus** — Understands user/customer needs, orients work toward their outcomes
- **attention_to_detail** — Precision, quality focus, catches issues before they escalate
- **strategic_thinking** — Sees the big picture, connects work to business goals, plans ahead

For each detected competency, provide:
- **score** (0.0-1.0): How strongly this competency is demonstrated
  - 0.0-0.2: Mentioned but no evidence
  - 0.2-0.4: Basic demonstration, limited scope
  - 0.4-0.6: Solid demonstration with specific examples
  - 0.6-0.8: Strong demonstration with depth and nuance
  - 0.8-1.0: Exceptional, with multiple strong examples
- **confidence** (0.0-1.0): How confident you are in the score
- **evidence**: 1-2 sentence excerpt or description from the transcript

## Layer 2: Specific Skills

These describe WHAT someone knows — specific technologies, tools, methodologies, domain knowledge. Extract every specific skill mentioned or demonstrated.

For each skill, assess depth:
- **surface** (0.0-0.25): Mentioned the name only, no context
- **working** (0.25-0.5): Has used it, can describe basic usage
- **deep** (0.5-0.75): Explains tradeoffs, implementation details, or teaches it
- **expert** (0.75-1.0): Shaped how it's used, built something significant, or demonstrates thought leadership

Confidence signals that INCREASE depth assessment:
- Specific project descriptions using the skill
- Explaining WHY they chose it over alternatives
- Describing tradeoffs or limitations they encountered
- Teaching or mentoring others in it
- Quantified outcomes from using it

Confidence signals that DECREASE depth assessment:
- Listing skills without context (buzzword dropping)
- Vague claims ("I'm proficient in...")
- Inconsistencies between claimed skill and described work
- Only mentioning the skill when listing tools

For each skill, provide:
- **name**: The skill name (normalized — e.g., "JavaScript" not "JS")
- **category**: One of: "technology", "methodology", "domain_knowledge", "tool", "soft_skill"
- **depth**: "surface", "working", "deep", or "expert"
- **depth_score** (0.0-1.0): Numeric depth score
- **confidence** (0.0-1.0): How confident you are in the depth assessment
- **evidence**: Brief supporting evidence from the transcript

## Output Format

Return a JSON object:

```json
{
  "behavioral_competencies": [
    {
      "name": "problem_solving",
      "score": 0.65,
      "confidence": 0.7,
      "evidence": "Described identifying high defect rates and systematically coaching team members one-on-one."
    }
  ],
  "specific_skills": [
    {
      "name": "Python",
      "category": "technology",
      "depth": "deep",
      "depth_score": 0.65,
      "confidence": 0.8,
      "evidence": "Built production ML pipeline using Python with PyTorch, chose PySide6 for frontend integration."
    }
  ],
  "overall_assessment": {
    "strongest_signals": ["list of 2-3 most confident findings"],
    "gaps": ["areas where evidence is thin or contradictory"],
    "suggested_followup_questions": ["1-2 questions that would help fill gaps"]
  }
}
```

Be conservative. If evidence is thin, use low confidence. If a skill is just name-dropped without context, score it as "surface" with low confidence. It's better to be accurately uncertain than falsely confident.

Only include behavioral competencies that have actual evidence in the transcript. Do not include competencies with zero evidence — omit them entirely.
""",
    # --- User prompt (per-request) ---
    """\
## Transcript Context

**Responder:** {responder}
**Prompt asked:** {prompt_text}
**Response type:** {context}

## Transcript

{transcript}
""",
)


# ---------------------------------------------------------------------------
# LLM Client
# ---------------------------------------------------------------------------


MODEL = os.environ.get("SCORING_MODEL", "gpt-4.1-mini")


def call_llm(system_prompt: str, user_message: str) -> dict:
    """Call LLM and parse JSON response."""
    client = OpenAI()

    response = client.chat.completions.create(
        model=MODEL,
        max_tokens=4096,
        temperature=0.0,
        response_format={"type": "json_object"},
        messages=[
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_message},
        ],
    )

    text = response.choices[0].message.content
    return json.loads(text.strip())


# ---------------------------------------------------------------------------
# Scoring
# ---------------------------------------------------------------------------


def score_transcript(transcript_record: dict, prompt_version: str = "v1") -> dict:
    """Score a single transcript record."""
    system_prompt, user_template = PROMPTS[prompt_version]

    responder = transcript_record["responder"]["fullName"]
    prompt_text = (
        transcript_record["prompt"]["selfText"]
        or transcript_record["prompt"]["otherUserText"]
        or "Unknown"
    )
    transcript = (transcript_record.get("transcript") or {}).get("text", "")

    if not transcript:
        return {
            "response_id": transcript_record["id"],
            "responder": responder,
            "error": "No transcript text",
        }

    is_self = transcript_record["subjectId"] == f"user:{transcript_record['responderUserId']}"
    context = "Self-response (talking about themselves)" if is_self else "Reference (talking about someone else)"

    user_message = user_template.format(
        responder=responder,
        prompt_text=prompt_text,
        transcript=transcript,
        context=context,
    )

    start = time.time()
    result = call_llm(system_prompt, user_message)
    elapsed = time.time() - start

    return {
        "response_id": transcript_record["id"],
        "responder": responder,
        "prompt_text": prompt_text,
        "transcript_length": len(transcript),
        "prompt_version": prompt_version,
        "llm_time_secs": round(elapsed, 2),
        "extraction": result,
    }


# ---------------------------------------------------------------------------
# Display
# ---------------------------------------------------------------------------


def print_result(result: dict):
    """Pretty-print a scoring result."""
    if "error" in result:
        print(f"\n--- {result['responder']} [{result['response_id'][:8]}] ---")
        print(f"  ERROR: {result['error']}")
        return

    extraction = result["extraction"]
    print(f"\n{'='*70}")
    print(f"  {result['responder']} [{result['response_id'][:8]}]")
    print(f"  Q: {result['prompt_text'][:80]}")
    print(f"  Transcript: {result['transcript_length']} chars | LLM: {result['llm_time_secs']}s")
    print(f"{'='*70}")

    # Behavioral competencies
    competencies = extraction.get("behavioral_competencies", [])
    if competencies:
        print(f"\n  Behavioral Competencies ({len(competencies)} detected):")
        for c in sorted(competencies, key=lambda x: x.get("score", 0), reverse=True):
            bar = "█" * int(c["score"] * 10) + "░" * (10 - int(c["score"] * 10))
            print(f"    {c['name']:22s} {bar} {c['score']:.2f}  (conf: {c['confidence']:.2f})")
            print(f"      {c['evidence'][:90]}")

    # Specific skills
    skills = extraction.get("specific_skills", [])
    if skills:
        print(f"\n  Specific Skills ({len(skills)} detected):")
        for s in sorted(skills, key=lambda x: x.get("depth_score", 0), reverse=True):
            depth_label = s.get("depth", "?")
            print(f"    {s['name']:22s}  [{s.get('category', '?'):15s}]  depth={depth_label:8s} ({s.get('depth_score', 0):.2f})  conf={s.get('confidence', 0):.2f}")
            if s.get("evidence"):
                print(f"      {s['evidence'][:90]}")

    # Overall
    overall = extraction.get("overall_assessment", {})
    if overall:
        print(f"\n  Assessment:")
        if overall.get("strongest_signals"):
            print(f"    Strongest: {', '.join(overall['strongest_signals'])}")
        if overall.get("gaps"):
            print(f"    Gaps: {', '.join(overall['gaps'])}")
        if overall.get("suggested_followup_questions"):
            print(f"    Follow-up: {overall['suggested_followup_questions'][0]}")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main():
    parser = argparse.ArgumentParser(description="Explore competency/skill scoring from transcripts")
    parser.add_argument("--index", type=int, help="Score transcript at this index (0-based)")
    parser.add_argument("--id", type=str, help="Score transcript with this response ID")
    parser.add_argument("--prompt", type=str, default="v1", help="Prompt version to use")
    parser.add_argument("--compare", nargs=2, metavar=("V1", "V2"), help="Compare two prompt versions")
    parser.add_argument("--all", action="store_true", help="Score all transcripts")
    parser.add_argument("--save", action="store_true", help="Save results to runs/explore/")
    args = parser.parse_args()

    # Load transcripts
    with open(TRANSCRIPTS_PATH) as f:
        transcripts = json.load(f)

    print(f"Loaded {len(transcripts)} transcripts")

    # Determine which to score
    if args.id:
        targets = [t for t in transcripts if t["id"] == args.id]
        if not targets:
            print(f"No transcript found with ID {args.id}")
            sys.exit(1)
    elif args.index is not None:
        targets = [transcripts[args.index]]
    elif args.all:
        targets = transcripts
    else:
        # Default: score first 3 as a sample
        targets = transcripts[:3]
        print("Scoring first 3 transcripts (use --all for all, --index N for specific)")

    prompt_versions = [args.prompt]
    if args.compare:
        prompt_versions = args.compare

    all_results = []

    for version in prompt_versions:
        if version not in PROMPTS:
            print(f"Unknown prompt version: {version}. Available: {list(PROMPTS.keys())}")
            sys.exit(1)

        print(f"\n--- Prompt version: {version} ---")

        for i, t in enumerate(targets):
            name = t["responder"]["fullName"]
            has_text = bool((t.get("transcript") or {}).get("text"))
            print(f"\n[{i+1}/{len(targets)}] Scoring {name}...", end=" ", flush=True)

            if not has_text:
                print("(no transcript, skipping)")
                continue

            try:
                result = score_transcript(t, version)
                all_results.append(result)
                print(f"done ({result['llm_time_secs']}s)")
                print_result(result)
            except Exception as e:
                print(f"ERROR: {e}")
                all_results.append({
                    "response_id": t["id"],
                    "responder": name,
                    "prompt_version": version,
                    "error": str(e),
                })

    # Save results
    if args.save or args.all:
        RUNS_DIR.mkdir(parents=True, exist_ok=True)
        timestamp = time.strftime("%Y%m%d-%H%M%S")
        versions = "-".join(prompt_versions)
        output_path = RUNS_DIR / f"run-{timestamp}-{versions}.json"
        with open(output_path, "w") as f:
            json.dump(all_results, f, indent=2)
        print(f"\nResults saved to {output_path}")


if __name__ == "__main__":
    main()
