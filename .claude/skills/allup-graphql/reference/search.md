# Search Service Reference

## Overview

The search_service provides search endpoints across the allUP platform, with two distinct paradigms: traditional text search for discovery/navigation, and RAG (Retrieval-Augmented Generation) semantic search for hiring workflows. Note that AI/RAG capabilities are transitioning to analysis_service, and some search endpoints may eventually fold back into domain services.

---

## Key Concepts

### Two Search Paradigms

The service supports fundamentally different search approaches:

| Paradigm | Query | Use Case | Returns |
|----------|-------|----------|---------|
| **Traditional Search** | `search()` | Discovery, navigation, typeahead | Raw resource IDs across multiple indices |
| **RAG Search** | `ragSearchV2()` | Hiring workflows, candidate discovery | User-grouped results with AI-generated context |

Traditional search is fast and stateless - simple text matching. RAG search is semantic, understanding meaning and relationships in interview responses.

### RAG Embeddings Architecture

RAG search operates on `RagPromptResponse` - the fundamental unit containing:
- Interview transcript text
- 3072-dimensional embedding (OpenAI `text-embedding-3-large`)
- Subject/responder information
- Associated tags

These embeddings enable semantic similarity matching: "leadership experience" finds responses about "managing teams" even without exact keyword matches.

### Relevance vs Search Score (Critical)

Two different scores with different meanings:

| Score | Source | Meaning | Range |
|-------|--------|---------|-------|
| `relevance` | LLM assessment | How well does this response answer the query? | 0-1 (filtered at 0.7 minimum) |
| `searchScore` | Vector similarity | Raw cosine similarity between query and response embeddings | 0-1 |

A high `searchScore` with low `relevance` means: semantically similar content, but not actually answering the question. Results are filtered at 0.7 relevance threshold.

### Response Backfilling

For each user in initial results, the system finds additional related responses using average embeddings. This is why a single search can surface 5+ responses per user - it finds the best match, then backfills with contextually related content from the same person.

### Job-Scoped Search

When `jobId` is provided to RAG search:
- Results constrained to job applicants only
- Uses job-specific prompts for context
- Authorization changes (hiring team members only)
- Enables applicant-specific workflows

Without `jobId`, search operates across all indexed responses the user can access.

---

## Important Relationships

```
Search Indices
  ├─ prompt_responses (RAG core - embeddings)
  ├─ user_profiles
  ├─ prompts
  ├─ prompt_tags
  ├─ organizations_saved
  └─ applicant

RAG Pipeline
  Query → Preprocessing → KNN Search → Backfilling → LLM Summary → Filter/Rank
                              ↓
                        OpenSearch
                        (FAISS HNSW)
```

**Cross-service dependencies:**
- `prompts_service`: Indexes prompt responses, provides transcript data
- `users_service`: User profile data for result enrichment
- `jobs_service`: Applicant filtering when job-scoped
- `analysis_service`: (AI capabilities migrating there)

---

## Common Workflows

### 1. RAG Search Pipeline

```
1. Query preprocessing (optional embedding generation)
2. KNN vector search (OpenSearch, FAISS HNSW, up to 100 responses)
3. Response backfilling (find related responses per user)
4. LLM summarization (GPT-4o-mini generates explanation, highlights, relevance score)
5. Filter (relevance >= 0.7) and rank
```

### 2. Job-Scoped Candidate Search

```graphql
ragSearchV2(query: "...", options: { jobId: "..." })
→ Search constrained to job applicants
→ Results grouped by user with AI summaries
→ Each record contains evidence responses
```

### 3. Discovery Search

```graphql
search(query: { text: "..." })
→ Fast text matching across all indices
→ Returns raw resource union (users, orgs, tags, prompts, applicants)
→ Use for navigation/typeahead
```

---

## Core Types

### Input Types

| Type | Description |
|------|-------------|
| `SearchQuery` | Simple text search input with `text: String!` |
| `PromptQuery` | Search prompts by `text: String!` |
| `PromptTagQuery` | Search tags by `tag: String!` |
| `RagQueryOptions` | RAG search options: `jobId`, `sort`, prompts for preprocessing/summary |
| `SearchPromptOptions` | Prompt search options: `embeddingSearch` (slower), `interviewGenerationOnly` |
| `SearchPromptTagOptions` | Tag search options: `embeddingSearch`, `onlyUserVisible` |
| `UserProfileSearch` | Search user profiles by `fullName: String!` |

### Output Types

| Type | Description |
|------|-------------|
| `SearchResult` | Wrapper containing a `SearchResource` union |
| `SearchResource` | Union of `UserProfile`, `Organization`, `PromptTag`, `Prompt`, `Applicant` |
| `RagSearchUserResult` | RAG results with `queryId`, `records`, and `suggestions` |
| `RagSearchUserResultRecord` | Individual user match with `userProfile`, `summary`, `details`, `responses`, `relevance`, `searchScore` |
| `RagPromptResponse` | Core RAG unit: interview answer with transcript, embedding, subject/responder info, tags |
| `SearchPromptsResults` | Contains `prompts: [Prompt!]!` |
| `SearchPromptTagsResults` | Contains `tags: [PromptTag!]!` |
| `Location` | Simple type with `id: String!` and `label: String!` |

### RagSearchUserResultRecord (Detail)

The primary output unit for RAG search:

| Field | Type | Description |
|-------|------|-------------|
| `userProfile` | `UserProfile!` | Matched user |
| `summary` | `String!` | AI-generated explanation of why this user matches |
| `details` | `String` | Additional context |
| `responses` | `[RagPromptResponse!]!` | Evidence - the actual interview responses |
| `relevance` | `Float!` | LLM confidence score (0-1, filtered at 0.7) |
| `searchScore` | `Float!` | Raw vector similarity score |

### Enums

| Enum | Values |
|------|--------|
| `RagSearchSort` | `RELEVANCE`, `SEARCH_SCORE` |

---

## Queries

### Multi-Index Search

```graphql
# Search across users, orgs, tags, prompts, applicants
search(query: SearchQuery!): [SearchResult!]!
```

### RAG Semantic Search

```graphql
# Primary RAG search - returns user-grouped results with AI summaries
ragSearchV2(query: String!, options: RagQueryOptions!): RagSearchUserResult!

# Preview preprocessed query (debugging)
ragPreprocess(query: String!, options: RagQueryOptions!): JSON!

# Autocomplete suggestions
ragSearchSuggestions(
  prefix: String!
  previousSuggestions: [String!]
  options: RagQueryOptions
): [String!]!
```

### Prompt and Tag Search

```graphql
searchPrompts(
  query: PromptQuery!
  options: SearchPromptOptions
): SearchPromptsResults!

searchPromptTags(
  query: PromptTagQuery!
  options: SearchPromptTagOptions
): SearchPromptTagsResults!
```

### Location Search

```graphql
searchLocations(query: SearchQuery!): [Location!]!
```

### User Profile Search

```graphql
searchUserProfiles(query: UserProfileSearch!): [UserProfile!]!
```

---

## Query Examples

### RAG Search with Job Context

```graphql
query {
  ragSearchV2(
    query: "leadership experience"
    options: { jobId: "job-123" }
  ) {
    queryId
    records {
      userProfile { userId fullName }
      summary
      details
      responses {
        transcript
        tags { name }
      }
      relevance
      searchScore
    }
    suggestions
  }
}
```

### Basic Multi-Index Search

```graphql
query {
  search(query: { text: "engineering" }) {
    resource {
      ... on UserProfile { userId fullName }
      ... on Organization { id name }
      ... on PromptTag { id name }
    }
  }
}
```

### Prompt Search with Embeddings

```graphql
query {
  searchPrompts(
    query: { text: "leadership" }
    options: { embeddingSearch: true }
  ) {
    prompts { id text }
  }
}
```

### RAG Search Sorted by Vector Similarity

```graphql
query {
  ragSearchV2(
    query: "startup experience"
    options: {
      sort: SEARCH_SCORE  # Use raw vector similarity instead of LLM relevance
    }
  ) {
    records {
      userProfile { fullName }
      searchScore
      relevance
    }
  }
}
```

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `SearchResult.resource` (UserProfile) | users | User profiles in search results |
| `SearchResult.resource` (Organization) | users | Organizations in search results |
| `SearchResult.resource` (Applicant) | jobs | Applicants in search results |
| `RagSearchUserResultRecord.userProfile` | users | User data in RAG results |
| `RagPromptResponse` | prompts | Interview responses being searched |
| `RagQueryOptions.jobId` | jobs | Job-scoped search constrains to applicants |

---

## Notes

- RAG search uses OpenSearch KNN vector search with FAISS HNSW algorithm
- Minimum relevance threshold is 0.7 - results below this are filtered out
- Embedding search options (`embeddingSearch: true`) are slower - avoid for typeahead/autocomplete
- Results are grouped by user in RAG search, with up to 100 initial responses before backfilling
- AI capabilities are transitioning to analysis_service - expect future changes to RAG endpoints
