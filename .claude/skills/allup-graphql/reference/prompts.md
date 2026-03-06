# Prompts Service Reference

## Overview

The prompts_service is the **lived feedback collection and video interview platform** - not just prompts/interviews. It manages the complete lifecycle of video-based feedback: prompts (question templates), responses (video answers with approval workflows), interviews (recording sessions linking responders to subjects), and a hierarchical tag system for categorization and behavior control.

---

## Key Concepts

### Interview

An Interview links a **responder** to a **subject** with a defined `relationship_type` (Boss, Coworker, Mentor, etc.). The relationship fundamentally drives content - affecting which prompts appear, how they're worded, and how the UI displays results.

Interviews contain multiple prompts and track completion status: **New → Partial → Complete**. Specialized interview types exist for different contexts: `Onboarding`, `Retake`, `Debug`, `TopicInterview`.

### Response Lifecycle (Critical)

Responses have a complex approval workflow that's separate from visibility:

```
New → Approved/ImpliedApproval/Rejected/Replaced
```

**Two independent gates determine visibility:**
1. **Approval state** - Has the response been approved?
2. **Visibility setting** - Public, Private, or HiringTeam

Both must pass for a response to be visible to a given viewer.

**Special cases:**
- Self-responses use `ImpliedApproval` (automatically approved)
- **Alive window**: New responses expire after ~15 minutes if not completed

### Subject Type Polymorphism

Prompts and interviews can target different subject types:
- **Person** (User or Invite)
- **Organization**
- **Job**
- **PromptTag** (topic-based interviews)

The subject type affects prompt text resolution, available prompts, and display logic.

### PromptTag Hierarchy

Tags form a hierarchical graph via `parent_tag_id` with five categories: Attributes, Achievements, Skills, Domains, Interests.

**Special `wrk:` prefix tags control behavior:**
| Pattern | Purpose |
|---------|---------|
| `wrk:topic:*` | Topic classification for interview planning |
| `wrk:strategy:*` | Strategy hints for Interview Director |
| `wrk:sort` | Sorting/ordering behavior |

### Interview Director (WebSocket Protocol)

The Interview Director is a **separate WebSocket-based system** for real-time interview orchestration - not part of the main GraphQL API. It plans interviews using configurable strategies:

- `BasicCompanyInterview`
- `JobInterviewStrategy`
- `OnboardingStrategy`

Connect via the `directorUrl` field on Interview. The Director handles dynamic prompt generation, response collection, and session state.

### Request System

Requests are permission/invitation containers that target a subject and request a response from a specific responder. They're used to invite someone to provide feedback about a subject.

---

## Important Relationships

```
Subject (User/Invite/Organization/Job)
  └─ Interviews
       ├─ Responder (who is recording)
       ├─ RelationshipType (how responder relates to subject)
       └─ Responses
            ├─ Prompt (the question answered)
            ├─ Asset (video from asset_service)
            ├─ Transcript
            ├─ Approval state
            ├─ Visibility
            └─ PromptResponseTags (auto-tagged by Lambda)

PromptTag (hierarchical)
  └─ parent_tag_id → PromptTag
  └─ child_tags → [PromptTag]
```

**Cross-service dependencies:**
- `jobs_service`: Job interviews, applicant responses
- `users_service`: User profiles, invites, organizations
- `assets_service`: Video storage and conversion
- `analysis_service`: Response analysis and tagging

---

## Common Workflows

### 1. Interview Request and Completion
```
Interview request created
→ Interview created (linking responder to subject with relationship_type)
→ Interview Director generates prompt components via WebSocket
→ Responder records responses
→ Response created (status: NEW)
→ Video asset uploaded to asset_service
→ Lambda triggers on upload
→ Response gets video_asset_id, status updates
```

### 2. Response Approval
```
Response created (approval: New)
→ If self-response: ImpliedApproval (automatic)
→ Otherwise: Explicit rating/approval action
→ Approval state: Approved/Rejected/Replaced
```

### 3. Auto-Tagging
```
Response created with video
→ Lambda runs analysis
→ PromptResponseTags populated
→ Tags include source tracking (which system applied them)
```

### 4. Get Responses for a Profile
```graphql
query {
  clipGroupsForProfile(subjectId: "...") {
    subject
    responses {
      id
      asset { url }
      responder { fullName }
      tags { tag category }
    }
  }
}
```

---

## Core Types

### Prompt

A question template that can be personalized based on subject and responder context.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `promptType` | `PromptType!` | `SYSTEM` or `USER` |
| `subjectType` | `PromptSubjectType!` | `PERSON`, `ORGANIZATION`, `PROMPT_TAG`, `JOB` |
| `selfText` | `String` | Text when asking about yourself |
| `otherUserText` | `String` | Text when asking about someone else |
| `text(subjectId, responderUserId, topicIds)` | `String!` | Resolved prompt text for context |
| `tagIds` | `[Id!]!` | Associated tags |
| `topicTagIds` | `[Id!]` | Topic-level tags for categorization |
| `purpose` | `String` | Purpose field for additional tagging |
| `relationshipTypes` | `[RelationshipType!]` | Applicable relationships |
| `recommendedDuration` | `Int!` | Suggested response length (seconds) |
| `maximumDuration` | `Int!` | Max response length (seconds) |

### PromptResponse

A video response to a prompt with approval and visibility controls.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `promptId` | `Id!` | Parent prompt |
| `subjectId` | `Id!` | Who the response is about |
| `responderUserId` | `Id!` | Who recorded the response |
| `interviewId` | `Id` | Parent interview (if any) |
| `status` | `PromptResponseStatus!` | Processing status |
| `approval` | `PromptResponseApproval!` | NEW, APPROVED, IMPLIED_APPROVAL, REJECTED, REPLACED |
| `visibility` | `PromptResponseVisibility!` | PUBLIC, PRIVATE, HIRING_TEAM |
| `videoAssetId` | `Id` | Video asset reference |
| `asset` | `Asset` | Resolved video asset |
| `transcript` | `Transcript` | Text transcript |
| `tags` | `[PromptTag!]!` | Associated tags |

### Interview

A recording session containing multiple prompt responses, linking responder to subject.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `subjectId` | `Id!` | Who the interview is about |
| `responderUserId` | `Id!` | Who is recording |
| `relationshipType` | `RelationshipType!` | Relationship between subject and responder |
| `interviewType` | `InterviewType` | ONBOARDING, RETAKE, DEBUG, TOPIC_INTERVIEW |
| `status` | `InterviewCompletionStatus!` | `NEW`, `PARTIAL`, `COMPLETE`, `UNKNOWN` |
| `prompts(excludeCompleted)` | `[Prompt!]!` | Interview prompts |
| `responses` | `[PromptResponse!]!` | Recorded responses |
| `directorUrl(directorApiVersion)` | `String!` | WebSocket URL for Interview Director |

### PromptTag

A hierarchical tag for categorizing prompts, responses, and users.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `tag` | `String!` | Display name |
| `category` | `PromptTagCategory` | `ATTRIBUTES`, `ACHIEVEMENTS`, `SKILLS`, `DOMAINS`, `INTERESTS`, `ROLES_AND_RESPONSIBILITIES` |
| `userSelectable` | `Boolean!` | Can users select this tag |
| `parentTag` | `PromptTag` | Parent in hierarchy |
| `childTags` | `[PromptTag!]!` | Children in hierarchy |
| `relatedTags` | `[PromptTag!]!` | Related tags |

### InterviewRequest

Permission/invitation container targeting a subject and requesting a response.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `subjectId` | `Id!` | Who the request is about |
| `requestedResponderId` | `Id` | Invited responder |
| `status` | `InterviewRequestStatus` | Request status |

### ClipGroup

A grouped collection of responses for presentation (e.g., by participant or topic).

| Field | Type | Description |
|-------|------|-------------|
| `subject` | `String` | Title/description |
| `responses` | `[PromptResponse!]!` | Responses in order |
| `participants` | `[UserProfile!]!` | Unique responders |
| `tags(category)` | `[PromptTag!]!` | Unique tags |
| `totalResponseCount` | `Int!` | Total available responses |

---

## Queries

### Single Lookup
```graphql
prompt(id: Id!): Prompt
promptTag(id: Id!): PromptTag
promptResponse(id: Id!): PromptResponse
interview(id: Id!): Interview
interviewRequest(id: Id!): InterviewRequest
```

### Batch Lookup
```graphql
prompts(ids: [Id!]!): [Prompt!]!
promptTags(ids: [Id!]!): [PromptTag!]!
promptResponses(ids: [Id!]!): [PromptResponse!]!
interviewRequests(ids: [Id!]!): [InterviewRequest!]!
```

### List with Pagination
```graphql
listPrompts(limit, after, before, sort, filter): PromptConnection!
listPromptTagsV2(limit, after, before, sort, filter): PromptTagConnection!
listPromptResponses(limit, after, before, sort, filter): PromptResponseConnection!
```

### Search
```graphql
searchPrompts(query: PromptQuery!, options): SearchPromptsResults!
searchPromptTags(query: PromptTagQuery!, options): SearchPromptTagsResults!
searchPromptResponses(query: PromptResponseSearch!, limit): [PromptResponse!]!
searchInterviews(query: InterviewSearch!): [Interview!]!
```

### User-Scoped
```graphql
interviewsForUser(userId: Id!, excludeCompleted: Boolean!): [Interview!]!
interviewRequestsForUser(userId: Id!, excludeCompleted): [InterviewRequest!]!
suggestedPrompts(userId: Id!, otherUserId, suggestedType): [SuggestedPrompt!]!
newPromptResponses(subjectId: Id!): NewPromptResponses!
unviewedPromptResponses(subjectId: Id!): [UnviewedPromptResponses!]!
```

### ClipGroup Queries
```graphql
clipGroupsForApplicant(applicantId: Id!): [ClipGroup!]!
clipGroupsForJob(jobId: Id!): [ClipGroup!]!
clipGroupsForProfile(subjectId: Id!): [ClipGroup!]!
clipGroupsForProfileTags(subjectId: Id!, category): [ClipGroup!]!
clipGroupsForUnviewedResponses(subjectId: Id!): [ClipGroup!]!
clipGroupsByParticipant(subjectId, responderUserId): [ClipGroup!]!
```

---

## Query Examples

### Get interview with responses and approval state

```graphql
query {
  interview(id: "...") {
    id
    status
    relationshipType
    responder { fullName }
    subject { ... on UserProfile { fullName } }
    responses {
      id
      approval
      visibility
      asset { url }
      prompt { text }
    }
  }
}
```

### Filter responses by approval and visibility

```graphql
query {
  listPromptResponses(
    filter: [
      { field: SUBJECT_ID, value: { id: "..." } }
      { field: APPROVAL, value: { approval: APPROVED } }
      { field: VISIBILITY, value: { visibility: PUBLIC } }
    ]
  ) {
    nodes {
      id
      responder { fullName }
      prompt { text }
    }
  }
}
```

### Get tag hierarchy

```graphql
query {
  promptTag(id: "...") {
    tag
    category
    parentTag { tag }
    childTags { id tag category }
    relatedTags { id tag }
  }
}
```

---

## Common Enums

### RelationshipType
Describes how responder relates to subject:
`BOSS`, `COWORKER`, `FRIEND`, `DIRECT_REPORT`, `MENTOR`, `MENTEE`, `SERVICE_PROVIDER`, `CUSTOMER`, `CLIENT`, `INVESTOR`, `INVESTEE`, `FOUNDER_INVESTED_IN`, `OTHER`, `YOU`, `UNKNOWN`

### PromptSubjectType
What a prompt is about: `PERSON`, `ORGANIZATION`, `PROMPT_TAG`, `JOB`

### PromptTagCategory
Tag classification: `ATTRIBUTES`, `ACHIEVEMENTS`, `SKILLS`, `DOMAINS`, `INTERESTS`, `ROLES_AND_RESPONSIBILITIES`

### PromptResponseApproval
Approval lifecycle: `NEW`, `APPROVED`, `IMPLIED_APPROVAL`, `REJECTED`, `REPLACED`

### PromptResponseVisibility
Access control: `PUBLIC`, `PRIVATE`, `HIRING_TEAM`

### InterviewCompletionStatus
Interview progress: `NEW`, `PARTIAL`, `COMPLETE`, `UNKNOWN`

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `PromptResponse.asset` | assets | Video storage |
| `Interview.subject` | users | User being interviewed about |
| `Interview.responder` | users | User recording the interview |
| `ClipGroup.applicant` | jobs | Applicant responses for hiring |
| `ClipGroup.job` | jobs | Job-specific interview responses |
| `PromptResponse` | analysis | Response analysis and auto-tagging |
| `Interview.subject` | users | Can be Invite (not yet registered user) |
| `Prompt.subjectType=JOB` | jobs | Job-specific prompts |
