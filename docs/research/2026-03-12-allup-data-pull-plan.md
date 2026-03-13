---
date: 2026-03-12T23:52:13Z
topic: "Pulling allUP Production Data into Competency Library"
git_commit: c0bdc787095f1d324011ade033e7967cde5e296f
branch: main
repository: competency-library
tags: [research, allup, data-pipeline, graphql]
status: complete
---

# Research: Pulling allUP Production Data into Competency Library

**Date**: 2026-03-12
**Git Commit**: c0bdc78
**Branch**: main

## Research Question

How do we pull down allUP production data (prompt tags, prompts, user profiles, resumes, responses) for ~500 users, store it locally, and make it explorable in the existing UI?

## Summary

The allUP GraphQL API has paginated list queries for all the data we need: `listPromptTagsV2`, `listPrompts`, `listUserProfiles`, and `listPromptResponses`. These all use cursor-based pagination with `limit`/`after`/`before` and support filtering. Resumes and affiliations lack global list endpoints but can be fetched per-user. The existing project has a Python script template (`python-template.py.txt`) with pagination helpers and `wrk` CLI auth that makes writing fetch scripts straightforward. The UI already has a clear pattern for adding new data sources: add a JSON file to `datasets/raw/allup/`, add a loader in `data.server.ts`, add types in `types.ts`, add a route.

## Detailed Findings

### 1. Data to Pull

| Dataset | Source Query | Scope | Estimated Volume |
|---------|-------------|-------|-----------------|
| **Prompt Tags** | `listPromptTagsV2(limit: 100)` | All tags | ~500-2000 tags |
| **Prompts** | `listPrompts(limit: 100)` | All prompts | ~200-500 prompts |
| **User Profiles** | `listUserProfiles(limit: 100)` or `userProfiles(ids: [...])` | 500 specific users | 500 profiles |
| **Prompt Responses** | `listPromptResponses(limit: 100, filter: [{field: RESPONDER_USER_ID, ...}])` | For those 500 users | ~5000-15000 responses |
| **Resumes** | `resumesForUser(userId: Id!)` | Per user | ~200-400 resumes |
| **Affiliations** | Via `UserProfile.affiliations` | Inline on profile fetch | N/A (comes with profiles) |
| **User Summaries** | `getOperationResults(topicIds: [...], operationName: "user_summary_data")` | Batch by user ID | 500 summaries |

### 2. Available GraphQL Queries

#### Prompt Tags — `listPromptTagsV2`
```graphql
listPromptTagsV2(
  limit: Int! = 100,
  after: String,
  sort: PromptTagSort! = {field: CREATED_AT, direction: ASC},
  filter: [PromptTagFilterCondition!]
): PromptTagConnection!
```
- Filter fields: `TAG`, `USER_SELECTABLE`, `PARENT_TAG_ID`, `VISIBLE_ON_PROFILE`, `DEFINES_CATEGORY`
- Key fields on PromptTag: `id`, `tag`, `description`, `internalDescription`, `userSelectable`, `structural`, `parentTagId`, `definesCategory`, `visibleOnProfile`, `featuredRole`, `color`, `synonyms`, `category`, `promptCount`, `childTags`, `properties`

#### Prompts — `listPrompts`
```graphql
listPrompts(
  limit: Int! = 100,
  after: String,
  sort: PromptSort! = {field: CREATED_AT, direction: ASC},
  filter: [PromptFilterCondition!]
): PromptConnection!
```
- Filter fields: `DURATION_SECS`, `RELATIONSHIP_TYPES`, `SUBJECT_TYPE`, `TAG_IDS`, `USE_FOR_INTERVIEW_GENERATION`
- Key fields on Prompt: `id`, `promptType`, `subjectType`, `selfText`, `otherUserText`, `tagIds`, `topicTagIds`, `order`, `priority`, `suggested`, `useForInterviewGeneration`, `relationshipTypes`, `recommendedDuration`, `maximumDuration`, `purpose`

#### User Profiles — `listUserProfiles` / `userProfiles(ids: [Id!]!)`
```graphql
listUserProfiles(
  limit: Int! = 100,
  after: String,
  sort: UserProfileSort! = {field: CREATED_AT, direction: ASC},
  filter: [UserProfileFilterCondition!]
): UserProfileConnection!
```
- For 500 specific users, batch lookup via `userProfiles(ids: [...])` is more efficient
- Key fields: `userId`, `fullName`, `shortName`, `headline`, `location`, `slug`, `createdAt`, `displayableResponseCount`, `promptTagIds`, `promptTags`, `affiliations`, `summary { summary facts responseIds }`

#### Prompt Responses — `listPromptResponses`
```graphql
listPromptResponses(
  limit: Int! = 100,
  after: String,
  sort: PromptResponseSort! = {field: CREATED_AT, direction: ASC},
  filter: [PromptResponseFilterCondition!]
): PromptResponseConnection!
```
- Filter fields: `APPROVAL`, `PROMPT_ID`, `RESPONDER_USER_ID`, `SUBJECT_ID`, `TAG_IDS`
- Key fields: `id`, `subjectId`, `responderUserId`, `promptId`, `promptText`, `approval`, `visibility`, `transcript { text }`, `responder { fullName }`, `tagIds`, `topicIds`, `createdAt`
- Can filter by `RESPONDER_USER_ID` to get responses for specific users

#### Resumes — Per-user only
```graphql
resumesForUser(userId: Id!): [Resume!]!
```
- No bulk list endpoint. Must iterate over user IDs.
- Fields: `id`, `userId`, `assetId`, `url`

#### User Summary Data — Batch operation results
```graphql
getOperationResults(
  topicIds: [Id!]!,
  operationName: String!
): [OperationResult!]!
```
- Use `operationName: "user_summary_data"` for structured profile data
- Also available inline via `UserProfile.summary { summary facts responseIds }`

### 3. Existing Patterns in This Project

**Current raw allUP data:** `datasets/raw/allup/transcripts.json` — 25 records, 41KB. Shape matches `TranscriptRecord` interface.

**Python script template** at `.claude/skills/allup-graphql/reference/python-template.py.txt`:
- Auth via `wrk -e production login admin --print`
- Pagination helper that yields nodes from cursor-based connections
- API URL: `https://api.wrkvbs.io/3/graphql`
- Uses `uv run --with requests --with pydantic script.py production`

**UI data loading pattern** (`ui/src/lib/data.server.ts`):
- `readJson<T>(filepath, fallback)` helper reads JSON from filesystem
- Each dataset has a typed loader function
- Routes import loaders in `+page.server.ts` and pass to Svelte components
- Types defined in `ui/src/lib/types.ts`

### 4. Execution-Graph Reference: How Production Fetches User Data

The seniority-profile operation (`/Users/ww/dev/execution-graph/operations/seniority-profile.md`) shows how allUP fetches user data in production:
- Uses `user_summary_data(&session, user_id, None)` to build a text blob containing profile + transcripts
- Single LLM call against that text blob
- The `user_summary_data` operation aggregates: profile, work history (affiliations), transcripts, and reference transcripts into one structured text

The resume-extraction operation shows the same pattern — takes `userSummary.text` as input which includes profile + transcripts + references.

### 5. Proposed Script Architecture

**One Python script per data type**, following the template pattern:

| Script | Output File | Strategy |
|--------|-------------|----------|
| `tools/fetch_prompt_tags.py` | `datasets/raw/allup/prompt-tags.json` | Paginate `listPromptTagsV2`, include hierarchy |
| `tools/fetch_prompts.py` | `datasets/raw/allup/prompts.json` | Paginate `listPrompts` |
| `tools/fetch_users.py` | `datasets/raw/allup/user-profiles.json` | Batch `userProfiles(ids: [...])` for user ID list, include affiliations + tags + summary inline |
| `tools/fetch_responses.py` | `datasets/raw/allup/responses.json` | Paginate `listPromptResponses` filtered by user IDs, include transcripts |
| `tools/fetch_resumes.py` | `datasets/raw/allup/resumes.json` | Loop `resumesForUser` per user |

**Alternatively**, a single `tools/fetch_allup_data.py` with subcommands:
```bash
uv run --with requests tools/fetch_allup_data.py production tags
uv run --with requests tools/fetch_allup_data.py production prompts
uv run --with requests tools/fetch_allup_data.py production users --ids-file user-ids.txt
uv run --with requests tools/fetch_allup_data.py production responses --ids-file user-ids.txt
uv run --with requests tools/fetch_allup_data.py production resumes --ids-file user-ids.txt
```

### 6. User ID List

**Curated list provided:** `datasets/raw/allup/user-ids.txt` — 1,500 users as TSV with columns:
- `user_id` — UUID (needs `user:` prefix for API calls)
- `full_name`
- `organization_name`
- `job_title`
- `category` — e.g., "Engineering"
- `has_feedback` — 0/1 flag

This file is the source of truth for which users to fetch data for. Scripts should parse this TSV to extract user IDs and prepend `user:` when calling the GraphQL API.

### 7. Safety Considerations

- All queries are **read-only** — no mutations will be used
- Auth via `wrk -e production login admin --print` — admin tokens have read access to all data
- Rate limiting: paginate with reasonable `limit` (100) and small delays between pages
- Data stored locally in `datasets/raw/allup/` (gitignored)
- Transcripts and profile data are PII — keep in gitignored directory only

### 8. UI Integration Plan

For each new data file, follow the existing pattern:

1. **Types** → `ui/src/lib/types.ts` — add interfaces for each data shape
2. **Loaders** → `ui/src/lib/data.server.ts` — add `loadPromptTags()`, `loadPrompts()`, `loadUserProfiles()`, `loadResponses()`
3. **Routes** → New routes under `ui/src/routes/`:
   - `/tags` — browse all prompt tags with hierarchy
   - `/prompts` — browse all prompts with relationships
   - `/users` — browse user profiles with responses, tags, affiliations
   - `/users/[id]` — individual user detail with all their data
4. **Cross-linking** — link from scoring results to user profiles, from users to their responses and tags

## Code References

- `tools/explore_scoring.py` — Existing scoring script, reads from `datasets/raw/allup/transcripts.json`
- `.claude/skills/allup-graphql/reference/python-template.py.txt` — Python template with auth + pagination
- `ui/src/lib/data.server.ts` — Data loading layer for UI
- `ui/src/lib/types.ts` — TypeScript type definitions
- `.claude/skills/allup-graphql/reference/prompts.md` — Prompts service API reference
- `.claude/skills/allup-graphql/reference/users.md` — Users service API reference
- `.claude/skills/allup-graphql/reference/analysis.md` — Analysis service API reference
- `/Users/ww/dev/execution-graph/operations/seniority-profile.md` — Production seniority scoring operation
- `/Users/ww/dev/execution-graph/operations/resume-extraction.md` — Production resume extraction operation

## Open Questions

1. **User ID list** — Does the user want to provide a specific list of 500 IDs, or should we grab the first 500 with responses from `listUserProfiles`?
2. **Response scope** — Pull ALL responses for the 500 users (self + references), or only self-responses (`subjectId == responderUserId`)?
3. **Transcript-only vs full response** — Current `transcripts.json` only has transcript text. Do we also want video asset URLs, approval status, visibility?
4. **Analysis results** — Should we also pull `user_summary_data` and/or `resume_extraction` operation results from the analysis service? These contain pre-computed structured data.
5. **One script or many** — Single script with subcommands vs separate scripts per data type?
6. **Incremental updates** — Should scripts support incremental fetching (only new data since last pull), or always full refresh?
