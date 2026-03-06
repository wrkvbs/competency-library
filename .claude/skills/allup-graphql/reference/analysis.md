# Analysis Service Reference

## Overview

The analysis_service is a **computation engine for AI-powered insights** - it generates derived data about entities from other services using LLM calls and structured analysis. It operates on a lazy evaluation model with caching: request an analysis, it gets queued, executed (often with LLM calls), and stored for reuse. Results are immutable and versioned, enabling reliable cache invalidation and dependency tracking.

---

## Key Concepts

### Operations (The Core Abstraction)

An **Operation** is a computation that generates derived data. Operations are:
- **Auto-registered**: Implement the trait, it appears in the system
- **Versioned**: Schema changes increment version, invalidating old results
- **Self-describing**: Each defines its own options schema via JSON Schema

Think of operations as pure functions: `(topic, options, associatedTopics) -> result`

### Topics (Lightweight References)

A **Topic** is a reference to an entity in another service - just an ID + type. The analysis_service doesn't store the actual entity, just computes things about it:
- User topics point to users_service
- Job topics point to jobs_service
- Applicant topics point to jobs_service

### Associated Topics (Context Matters)

Same operation on same topic with **different associated topics = different result**. This enables contextual analysis:

```
user_summary(user_123)                    → General user summary
user_summary(user_123, [job_456])         → User summary in context of job 456
summarize_applicant(user_123, [job_789])  → User as applicant for specific job
```

The `associatedTopicIds` field captures this context. When querying, you must match the exact associatedTopicIds used during creation.

### OperationResult (Immutable Cache)

Each result is an immutable record:
- Same inputs = reuse cached result (based on freshness policy)
- Different inputs = new result
- Version mismatch = result considered stale

### Freshness Policies

Control cache reuse behavior when requesting results:

| Policy | Behavior |
|--------|----------|
| **MustBeExact** | No cache - always recompute |
| **WithinDuration** | Accept if updated within specified time |
| **AnyValid** | Accept any result with matching version |
| **PreferCached** | Use cached even if stale, trigger background refresh |

### Dependency Graph

Operations can depend on other operations:
```
job_prompts
  └─ depends on: job_criteria
  └─ depends on: job_tags
```

When `job_prompts` runs, it first ensures its dependencies are fresh (based on freshness policies). This creates a DAG of computations.

---

## Important Relationships

```
External Services                    Analysis Service
─────────────────                    ────────────────
users_service ──┐
                │                    Topic (reference)
jobs_service ───┼───────────────────→  │
                │                       └─ OperationResult
prompts_service─┘                            ├─ operationName
                                             ├─ value (JSON)
                                             └─ associatedTopicIds
```

**Cross-service dependencies:**
- `users_service`: User profile data for summaries
- `jobs_service`: Job details, applicant data
- `prompts_service`: Response content for analysis
- `search_service`: Publishes updates to trigger search index refresh

---

## Common Workflows

### 1. Discover Available Operations

Operations are **not fully described in the GraphQL schema** - use introspection queries to discover them:

```graphql
# List all available operations
query {
  listOperations {
    name
    title
    version
    optionsSchema   # JSON Schema describing accepted options
  }
}

# Get details for a specific operation
query {
  operationByName(operationName: "user_summary") {
    name
    title
    optionsSchema   # Describes what options this operation accepts
    prompts         # Prompt templates used (if applicable)
  }
}
```

The `optionsSchema` field returns a JSON Schema that describes what `operationOptions` the operation accepts.

### 2. Trigger an Operation

```graphql
mutation {
  triggerAnalysis(options: {
    operationName: "user_summary"
    topicIds: ["user_123"]
    operationOptions: { "includeJobHistory": true }  # Options per schema
    immediate: true  # Skip queue delay - use when you need result soon
  })
}
```

Operations are queued and processed asynchronously. Use `immediate: true` when you need the result quickly (e.g., user-facing request), omit it for batch/background processing.

**Important:** Operations can take significant time to complete (seconds to minutes for LLM-based operations). After triggering, you'll need to poll `getOperationResult` until a result is available. The mutation returns immediately after queuing - it does not wait for completion.

### 3. Query the Result

```graphql
query {
  getOperationResult(
    topicId: "user_123"
    operationName: "user_summary"
    associatedTopicIds: []  # Must match what was used in trigger
  ) {
    id
    value        # The actual result (JSON)
    updatedAt
  }
}
```

Returns cached result or null if not computed yet. The `associatedTopicIds` must exactly match what was used when triggering.

### 4. Contextual Analysis (Associated Topics)

For operations that need context (e.g., analyzing a user for a specific job):

```graphql
# Trigger with context
mutation {
  triggerAnalysis(options: {
    operationName: "summarize_applicant"
    topicIds: ["user_123"]
    associatedTopicIds: ["job_456"]  # The job context
  })
}

# Query with same context
query {
  getOperationResult(
    topicId: "user_123"
    operationName: "summarize_applicant"
    associatedTopicIds: ["job_456"]  # Must match
  ) { value }
}
```

### 5. Test Without Persisting

```graphql
query {
  testOperationResult(
    topicId: "user_123"
    operationName: "user_summary"
    options: { "includeJobHistory": true }
  ) {
    value
  }
}
```

Runs the operation but doesn't store the result - useful for development and testing.

---

## Operation Categories

### Profile Operations
| Operation | Description |
|-----------|-------------|
| `user_summary` | AI-generated summary of user profile |
| `user_summary_data` | Structured data extraction from profile |
| `profile_summarization` | Alternative summarization format |

### Job Operations
| Operation | Description |
|-----------|-------------|
| `job_summary` | Summary of job posting |
| `job_criteria` | Extracted qualification criteria |
| `job_tags` | Auto-generated classification tags |
| `job_dimensions` | Structured job dimensions |
| `job_memo` | AI-generated hiring memo |
| `job_prompts` | Generated interview prompts (depends on job_criteria, job_tags) |
| `job_teaser` | Short promotional summary |

### Applicant Operations
| Operation | Description |
|-----------|-------------|
| `summarize_applicant` | AI summary of applicant for specific job |
| `applicant_dimension` | Dimensional analysis of applicant fit |
| `response_detail` | Detailed analysis of interview responses |

### Prompt Operations
| Operation | Description |
|-----------|-------------|
| `prompt_tip` | Interviewer tips for a prompt |
| `prompt_tag_question` | Tag-based question generation |
| `prompt_selection` | Prompt recommendation logic |

---

## Core Types

### OperationResult

Stores the output of running an operation on a topic.

```graphql
type OperationResult {
    id: Id!
    createdAt: DateTime!
    updatedAt: DateTime!
    operationName: String!       # Name of the operation that produced this result
    operationInstanceId: Id      # Links to versioned operation schema
    topicId: Id!                 # The subject (user, job, etc.) this analysis is about
    associatedTopicIds: [Id!]!   # Related topics (e.g., job for job-match analysis)
    key: String                  # Optional key for operation variants
    traceId: String              # OpenTelemetry trace for debugging
    value: JSON!                 # The actual analysis result (schema varies by operation)
}
```

### Operation

Metadata about a registered operation.

```graphql
type Operation {
    name: String!            # Unique identifier (e.g., "user_summary", "job_match")
    version: Int!            # Schema version for result evolution
    prompts: JSONObject!     # Prompt templates used by the operation
    optionsSchema: JSON      # JSON schema for operation options
    title: String!           # Human-readable name
}
```

### AnalysisOptions (input)

Options for triggering an analysis operation.

```graphql
input AnalysisOptions {
    operationName: String!       # Which operation to run
    operationOptions: JSON       # Operation-specific parameters
    topicIds: [Id!]              # Specific topics to analyze
    associatedTopicIds: [Id!]    # Related topics for the analysis
    key: String                  # Optional variant key
    allTopics: Boolean           # Run on all valid topics for this operation
    needsUpdate: Boolean         # Only topics needing refresh
    fileUrl: String              # External data source
    immediate: Boolean           # Skip queue delay (use with caution)
}
```

---

## Queries

### Look up a single result

```graphql
# Get result for a specific topic + operation + associatedTopics
query {
  getOperationResult(
    topicId: "user_123"
    operationName: "user_summary"
    associatedTopicIds: []      # Must match stored associatedTopicIds
    key: null
  ) {
    id
    value
    updatedAt
  }
}
```

### Look up by ID

```graphql
query {
  operationResult(id: "result_abc") {
    operationName
    topicId
    value
  }
}
```

### List results

```graphql
# All results for a topic
query {
  listOperationResults(topicId: "user_123") {
    id
    operationName
    value
  }
}

# All results for an operation
query {
  listOperationResults(operationName: "job_match") {
    topicId
    associatedTopicIds
    value
  }
}
```

### Batch lookup

```graphql
query {
  getOperationResults(
    topicIds: ["user_1", "user_2", "user_3"]
    operationName: "user_summary"
  ) {
    topicId
    value
  }
}
```

### Test an operation (dev)

Run an operation without persisting, useful for testing:

```graphql
query {
  testOperationResult(
    topicId: "user_123"
    operationName: "user_summary"
    options: { "includeJobHistory": true }
  ) {
    value
  }
}
```

### List available operations

```graphql
query {
  listOperations {
    name
    version
    title
    optionsSchema
  }
}

query {
  operationByName(operationName: "user_summary") {
    name
    version
    prompts
  }
}
```

---

## Query Examples

### Get user summary with job context

```graphql
query {
  getOperationResult(
    topicId: "user_123"
    operationName: "user_summary"
    associatedTopicIds: ["job_456"]
  ) {
    value
    updatedAt
  }
}
```

### Get all analyses for a job

```graphql
query {
  listOperationResults(topicId: "job_456") {
    operationName
    value
    updatedAt
  }
}
```

### Batch fetch user summaries

```graphql
query {
  getOperationResults(
    topicIds: ["user_1", "user_2", "user_3"]
    operationName: "user_summary"
  ) {
    topicId
    value
  }
}
```

---

## Mutations

### Trigger analysis

Queue topics for analysis processing:

```graphql
mutation {
  triggerAnalysis(options: {
    operationName: "user_summary"
    topicIds: ["user_123", "user_456"]
  })
}

# Refresh all stale results
mutation {
  triggerAnalysis(options: {
    operationName: "user_summary"
    allTopics: true
    needsUpdate: true
  })
}

# With associated topics (for contextual analysis)
mutation {
  triggerAnalysis(options: {
    operationName: "summarize_applicant"
    topicIds: ["user_123"]
    associatedTopicIds: ["job_456"]
  })
}
```

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `topicId` (user) | users | User being analyzed |
| `topicId` (job) | jobs | Job being analyzed |
| `topicId` (applicant) | jobs | Applicant being analyzed |
| `associatedTopicIds` | jobs/users | Context entities for analysis |
| Pubsub notifications | search | Triggers search index refresh |
| `Applicant.summary` | jobs | Jobs service queries analysis results |
