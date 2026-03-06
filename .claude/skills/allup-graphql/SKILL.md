---
name: allup-graphql
description: Query the allUP GraphQL API efficiently. Use when you need to fetch data from allUP services, explore the API schema, or build queries against the federated GraphQL API.
---

# allUP GraphQL API

Use this skill when querying the allUP backend API.

## Reference Documentation

Before writing a query, read the reference doc for the service domain you're querying. These contain type definitions, query signatures, relationship maps, and domain-specific examples.

| Service | Domain | Reference |
|---------|--------|-----------|
| **users** | Profiles, orgs, auth | `reference/users.md` |
| **prompts** | Prompts, interviews, clips, responses | `reference/prompts.md` |
| **jobs** | Job postings, applicants, hiring | `reference/jobs.md` |
| **analysis** | Computed data, summaries | `reference/analysis.md` |
| **search** | OpenSearch-backed search | `reference/search.md` |
| **assets** | File storage, media | `reference/assets.md` |
| **social_graph** | Connections, relationships | `reference/social_graph.md` |
| **notifs** | Notifications | `reference/notifs.md` |
| **config** | Configuration values | `reference/config.md` |
| **ops** | Operations, logging | `reference/ops.md` |
| **media_pipeline** | Media processing | `reference/media_pipeline.md` |

Reference files are at `.claude/skills/allup-graphql/reference/` relative to the project root.

---

## MCP Tools

### `allup_call_graphql` (primary tool)

Execute read-only GraphQL queries. Mutations and subscriptions are blocked.

**Critical constraints:**

1. **Always use the `query` keyword** — anonymous queries like `{ user { id } }` are rejected by the server. Write `query { user { id } }` or `query GetUser { user { id } }`.

2. **`arguments` only supports scalar string values** — array types and objects cannot be passed as variables. For non-scalar arguments, inline the values directly in the query string.

**Scalar argument (use variables):**

```
allup_call_graphql(
  query: "query ($slug: String!) { userProfileBySlug(slug: $slug) { userId fullName } }",
  arguments: { "slug": "example" }
)
```

**Array argument (inline the values):**

```
allup_call_graphql(
  query: "query { promptResponses(ids: [\"id-1\", \"id-2\", \"id-3\"]) { id transcript { text } } }",
  arguments: {}
)
```

**Batch fetch pattern** — use plural queries (e.g., `promptResponses(ids:)`) with inline IDs to fetch multiple records in a single call instead of making N individual requests.

### `allup_query_schema`

Search the schema and get only matching types with their dependencies. Use this when the reference docs don't have the exact query shape you need.

```
allup_query_schema(pattern: "Applicant", match_type: "contains")
```

**Match types:**
- `contains` (default) - pattern anywhere in type/field name
- `prefix` - pattern at start of name
- `exact` - exact match (case-insensitive)

Prefer this over `allup_get_schema`, which returns the full 5000+ line schema.

### `allup_authenticate`

Initiate OAuth authentication. Call this if a query fails with auth errors.

The user may need to visit the returned URL in their browser to complete OAuth.

---

## Querying Patterns

### Request only needed fields

```graphql
query {
  applicant(id: "...") {
    id
    status
    user { fullName }
  }
}
```

### Look up by slug

```graphql
query ($slug: String!) {
  userProfileBySlug(slug: $slug) { userId fullName }
}
```

### List with filters

```graphql
query {
  listJobs(filter: [{ field: STATUS, op: EQUALS, value: { status: ACTIVE } }]) {
    nodes { id title status }
  }
}
```

### Batch fetch by IDs

```graphql
query {
  promptResponses(ids: ["id-1", "id-2"]) {
    id transcript { text } prompt { selfText }
  }
}
```

---

## Pagination

The API uses cursor-based pagination for all list queries.

```graphql
query {
  listApplicants(limit: 20, sort: { field: CREATED_AT, direction: DESC }) {
    nodes { id status }
    pageInfo { hasNextPage endCursor }
  }
}
```

To get the next page, pass `after: pageInfo.endCursor` from the previous response.

**Key details:**
- Default limit is 100. Always specify a reasonable limit.
- Cannot use both `after` and `before` simultaneously.
- Default sort is ASC — use `direction: DESC` for newest-first.
- Filter operations: `EQUALS`, `CONTAINS`, `STARTS_WITH`, `GT`, `LT`, `IN`, `IS_NULL`, `IS_NOT_NULL`

---

## Scripting (large result sets only)

For results too large to process in context (hundreds of items needing aggregation), fall back to scripts. See `reference/scripting.md` for bash and Python templates, auth setup, and examples. **For normal queries, always use the MCP tools directly.**

---

## Environments

- `dev` - Development
- `staging` - Staging
- `production` - Production

The MCP server is configured with an environment. Override per-query if needed.
