#!/usr/bin/env python3
"""
Fetch allUP production data for competency library exploration.

Usage:
    uv run --with requests tools/fetch_allup_data.py production tags
    uv run --with requests tools/fetch_allup_data.py production prompts
    uv run --with requests tools/fetch_allup_data.py production users
    uv run --with requests tools/fetch_allup_data.py production responses
    uv run --with requests tools/fetch_allup_data.py production resumes
    uv run --with requests tools/fetch_allup_data.py production all
"""

import argparse
import json
import os
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import requests

# =============================================================================
# Configuration
# =============================================================================

API_URLS = {
    "dev": "https://api.dev.wrkvbs.io/3/graphql",
    "staging": "https://api.staging.wrkvbs.io/3/graphql",
    "production": "https://api.wrkvbs.io/3/graphql",
}

SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
RAW_ALLUP = REPO_ROOT / "datasets" / "raw" / "allup"

# =============================================================================
# Auth Helper (cached 10 min)
# =============================================================================

_token_cache: dict[str, tuple[str, float]] = {}


def get_token(env: str) -> str:
    cached = _token_cache.get(env)
    if cached and time.time() - cached[1] < 600:
        return cached[0]
    result = subprocess.run(
        ["wrk", "-e", env, "login", "admin", "--print"],
        capture_output=True, text=True, check=True,
    )
    token = result.stdout.strip()
    _token_cache[env] = (token, time.time())
    return token


# =============================================================================
# GraphQL Client with retry
# =============================================================================


def execute_query(env: str, query: str, variables: dict | None = None, retries: int = 3) -> dict:
    token = get_token(env)
    for attempt in range(retries):
        resp = requests.post(
            API_URLS[env],
            headers={"Authorization": f"Bearer {token}", "Content-Type": "application/json"},
            json={"query": query, "variables": variables or {}},
            timeout=60,
        )
        if resp.status_code in (429, 502, 503, 504) and attempt < retries - 1:
            wait = 2 ** (attempt + 1)
            print(f"  Retry {attempt + 1}/{retries} after {resp.status_code}, waiting {wait}s...")
            time.sleep(wait)
            continue
        resp.raise_for_status()
        result = resp.json()
        if "errors" in result:
            raise Exception(f"GraphQL errors: {json.dumps(result['errors'], indent=2)}")
        return result["data"]
    raise Exception("Max retries exceeded")


def paginate(env: str, query: str, connection_path: str, variables: dict | None = None, delay: float = 0.2):
    variables = dict(variables or {})
    cursor = None
    while True:
        variables["after"] = cursor
        data = execute_query(env, query, variables)
        connection = data
        for key in connection_path.split("."):
            connection = connection[key]
        nodes = connection["nodes"]
        yield from nodes
        page_info = connection["pageInfo"]
        if not page_info["hasNextPage"]:
            break
        cursor = page_info["endCursor"]
        if delay:
            time.sleep(delay)


# =============================================================================
# User IDs
# =============================================================================


def load_user_ids() -> list[str]:
    path = RAW_ALLUP / "user-ids.txt"
    ids = []
    with open(path) as f:
        for i, line in enumerate(f):
            if i == 0:
                continue  # skip header
            parts = line.strip().split("\t")
            if parts:
                ids.append(parts[0])
    return ids


def load_user_id_set() -> set[str]:
    return set(load_user_ids())


# =============================================================================
# Output helpers
# =============================================================================


def save_json(data: Any, filename: str) -> Path:
    path = RAW_ALLUP / filename
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w") as f:
        json.dump(data, f, indent=2, default=str)
    print(f"  Saved {path} ({os.path.getsize(path) / 1024:.0f} KB)")
    return path


def load_checkpoint(cmd: str) -> dict:
    path = RAW_ALLUP / f".{cmd}-progress.json"
    if path.exists():
        with open(path) as f:
            return json.load(f)
    return {}


def save_checkpoint(cmd: str, data: dict):
    path = RAW_ALLUP / f".{cmd}-progress.json"
    with open(path, "w") as f:
        json.dump(data, f, indent=2, default=str)


def clear_checkpoint(cmd: str):
    path = RAW_ALLUP / f".{cmd}-progress.json"
    if path.exists():
        path.unlink()


# =============================================================================
# Subcommands
# =============================================================================

# -- tags --

TAGS_QUERY = """
query($after: String) {
    listPromptTagsV2(limit: 100, after: $after) {
        nodes {
            id tag description internalDescription category
            userSelectable structural parentTagId definesCategory
            visibleOnProfile featuredRole color synonyms
            promptCount properties
            childTags { id tag category }
        }
        pageInfo { hasNextPage endCursor }
    }
}
"""


def fetch_tags(env: str, **kwargs):
    print("Fetching prompt tags...")
    tags = list(paginate(env, TAGS_QUERY, "listPromptTagsV2"))
    print(f"  Got {len(tags)} tags")
    save_json({"fetched_at": datetime.now(timezone.utc).isoformat(), "count": len(tags), "tags": tags}, "prompt-tags.json")


# -- prompts --

PROMPTS_QUERY = """
query($after: String) {
    listPrompts(limit: 100, after: $after) {
        nodes {
            id promptType subjectType selfText otherUserText
            tagIds topicTagIds order priority suggested
            useForInterviewGeneration relationshipTypes
            recommendedDuration maximumDuration purpose
        }
        pageInfo { hasNextPage endCursor }
    }
}
"""


def fetch_prompts(env: str, **kwargs):
    print("Fetching prompts...")
    prompts = list(paginate(env, PROMPTS_QUERY, "listPrompts"))
    print(f"  Got {len(prompts)} prompts")
    save_json({"fetched_at": datetime.now(timezone.utc).isoformat(), "count": len(prompts), "prompts": prompts}, "prompts.json")


# -- users --

USERS_QUERY = """
query($ids: [Id!]!) {
    userProfiles(ids: $ids) {
        userId fullName shortName headline pronouns { form subjective objective posessive } slug createdAt trending
        promptTags { id tag category }
        affiliations(status: APPROVED) {
            id
            organization { id name shortDescription location }
            title startedAt endedAt status
        }
        intentFlags { intentFlagId text color expiresAt }
        socialLinks { socialLinkId name username url }
    }
}
"""


def fetch_users(env: str, limit: int = 0, **kwargs):
    user_ids = load_user_ids()
    if limit:
        user_ids = user_ids[:limit]
    print(f"Fetching {len(user_ids)} user profiles...")

    checkpoint = load_checkpoint("users")
    profiles = checkpoint.get("profiles", [])
    done_ids = {p["userId"] for p in profiles}
    remaining = [uid for uid in user_ids if f"user:{uid}" not in done_ids and uid not in done_ids]

    batch_size = 50
    for i in range(0, len(remaining), batch_size):
        batch = remaining[i : i + batch_size]
        api_ids = [f"user:{uid}" for uid in batch]
        print(f"  Batch {i // batch_size + 1}/{(len(remaining) + batch_size - 1) // batch_size} ({len(batch)} users)")
        data = execute_query(env, USERS_QUERY, {"ids": api_ids})
        batch_profiles = data["userProfiles"]
        profiles.extend(batch_profiles)
        save_checkpoint("users", {"profiles": profiles})
        time.sleep(0.2)

    print(f"  Got {len(profiles)} profiles")
    save_json({"fetched_at": datetime.now(timezone.utc).isoformat(), "count": len(profiles), "profiles": profiles}, "user-profiles.json")
    clear_checkpoint("users")


# -- responses --

RESPONSES_BATCH_QUERY = """
query($ids: [Id!]!) {
    promptResponses(ids: $ids) {
        id promptId subjectId responderUserId approval visibility
        promptText
        transcript { text }
        responder { fullName }
        tagIds topicIds createdAt
    }
}
"""


def load_response_ids() -> list[str]:
    path = RAW_ALLUP / "response-ids.txt"
    ids = []
    with open(path) as f:
        for i, line in enumerate(f):
            if i == 0:
                continue  # skip header
            rid = line.strip()
            if rid:
                ids.append(rid)
    return ids


def fetch_responses(env: str, limit: int = 0, **kwargs):
    response_ids = load_response_ids()
    if limit:
        response_ids = response_ids[:limit]
    print(f"Fetching {len(response_ids)} responses via promptResponses batch API...")

    checkpoint = load_checkpoint("responses")
    all_responses = checkpoint.get("responses", [])
    fetched_count = checkpoint.get("fetched_count", 0)
    remaining_ids = response_ids[fetched_count:]

    batch_size = 200
    total_batches = (len(remaining_ids) + batch_size - 1) // batch_size
    for i in range(0, len(remaining_ids), batch_size):
        batch = remaining_ids[i : i + batch_size]
        api_ids = [f"prompt-response:{rid}" for rid in batch]
        batch_num = i // batch_size + 1
        print(f"  Batch {batch_num}/{total_batches} ({len(batch)} IDs)")
        try:
            data = execute_query(env, RESPONSES_BATCH_QUERY, {"ids": api_ids})
            batch_responses = data["promptResponses"]
            all_responses.extend(batch_responses)
        except Exception as e:
            print(f"  Warning: batch {batch_num} failed: {e}")
        fetched_count += len(batch)
        save_checkpoint("responses", {"responses": all_responses, "fetched_count": fetched_count})

    print(f"  Got {len(all_responses)} responses")
    save_json({"fetched_at": datetime.now(timezone.utc).isoformat(), "count": len(all_responses), "responses": all_responses}, "responses.json")
    clear_checkpoint("responses")


# -- resumes --

RESUME_QUERY = """
query($userId: Id!) {
    resumesForUser(userId: $userId) {
        id userId assetId url
    }
}
"""


def fetch_resumes(env: str, limit: int = 0, delay: float = 0.2, **kwargs):
    user_ids = load_user_ids()
    if limit:
        user_ids = user_ids[:limit]
    print(f"Fetching resumes for {len(user_ids)} users...")

    checkpoint = load_checkpoint("resumes")
    all_resumes = checkpoint.get("resumes", [])
    done_ids = set(checkpoint.get("done_ids", []))
    remaining = [uid for uid in user_ids if uid not in done_ids]

    for i, uid in enumerate(remaining):
        if i % 100 == 0 and i > 0:
            print(f"  Progress: {i}/{len(remaining)} users, {len(all_resumes)} resumes found")
            save_checkpoint("resumes", {"resumes": all_resumes, "done_ids": list(done_ids)})
        try:
            data = execute_query(env, RESUME_QUERY, {"userId": f"user:{uid}"})
            resumes = data["resumesForUser"]
            if resumes:
                all_resumes.extend(resumes)
            done_ids.add(uid)
        except Exception as e:
            print(f"  Warning: failed for user {uid}: {e}")
            done_ids.add(uid)
        if delay:
            time.sleep(delay)

    print(f"  Got {len(all_resumes)} resumes from {len(done_ids)} users")
    save_json({"fetched_at": datetime.now(timezone.utc).isoformat(), "count": len(all_resumes), "resumes": all_resumes}, "resumes.json")
    clear_checkpoint("resumes")


# =============================================================================
# CLI
# =============================================================================

COMMANDS = {
    "tags": fetch_tags,
    "prompts": fetch_prompts,
    "users": fetch_users,
    "responses": fetch_responses,
    "resumes": fetch_resumes,
}


def main():
    parser = argparse.ArgumentParser(description="Fetch allUP production data")
    parser.add_argument("env", choices=["dev", "staging", "production"], help="Target environment")
    parser.add_argument("command", choices=[*COMMANDS.keys(), "all"], help="What to fetch")
    parser.add_argument("--limit", type=int, default=0, help="Limit results (0 = no limit)")
    parser.add_argument("--delay", type=float, default=0.2, help="Delay between requests (seconds)")
    args = parser.parse_args()

    print(f"Environment: {args.env}")
    print(f"API: {API_URLS[args.env]}\n")

    if args.command == "all":
        for name, fn in COMMANDS.items():
            fn(args.env, limit=args.limit, delay=args.delay)
            print()
    else:
        COMMANDS[args.command](args.env, limit=args.limit, delay=args.delay)


if __name__ == "__main__":
    main()
