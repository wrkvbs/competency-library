# Jobs Service Reference

## Overview

The jobs_service is the **hiring workflow management system** - not just job postings. It orchestrates the entire recruitment process: job lifecycle, applicant journey, hiring team coordination, and structured evaluation via a tag-based review system.

---

## Key Concepts

### Job Lifecycle

Jobs progress through states: **Draft → Open → InReview → Closed**. The transition from Open to InReview is tied to submission deadlines, which operate on a **weekly cohort system** (resets Tuesday, Pacific Time, with ~11 day windows).

A job's status can appear different to different applicants based on their individual deadline - use `cohort_status` for the applicant-relative view.

### Applicant State Machine

Applicants follow a defined progression with transition rules:

```
Applied → Completed → Submitted → Accepted/Rejected → Hired
                                ↘ Withdrawn
```

- **Applied**: Initial state when application created
- **Completed**: All required prompts answered (checked via prompts_service)
- **Submitted**: Application finalized for review
- **Accepted/Rejected**: Hiring decision made
- **Hired**: Final successful outcome

Transitions are enforced - you can't skip states or go backwards.

### The Tag System (Critical)

ApplicantTags have **three categories** that serve different purposes:

| Category | Purpose | Example |
|----------|---------|---------|
| **State** | Workflow position - where is this applicant in the review process? | "Needs Review", "Phone Screen", "Final Interview" |
| **Bucket** | Categorization - how strong is this applicant? | "All Star", "Strong", "Maybe", "Not a Fit" |
| **Feedback** | Notes - what did reviewers observe? | "Great communicator", "Needs technical growth" |

State tags define `nextStateTagIds` - the valid transitions from that state. This creates a configurable workflow per job.

### Review-Driven State Updates

When a hiring team member creates an **ApplicantReview** with tags:
1. The system finds the most recent State tag across all reviews
2. The system finds the most recent Bucket tag across all reviews
3. Applicant's `stateTagId` and `bucketTagId` are updated automatically

This means **reviews drive applicant state** - the most recent review's tags "win."

### Hiring Team Roles

Two distinct roles with different permissions and behavior:

| Role | Who | Behavior |
|------|-----|----------|
| **HiringManager** | Company-side decision maker | Full access, receives Slack notifications on state changes |
| **MemberPlacement** | Internal allUP staff | Support role, excluded from certain notifications |

Only one HiringManager per job (enforced).

### Cohort Deadlines

Applicants are grouped into weekly cohorts for fair evaluation:
- Default deadline: Job's `submissionDeadline`
- Can be extended per-applicant
- Deadline calculation: Tuesday start + ~10 days 23h 45m (Pacific Time)

This batching ensures applicants in the same cohort are evaluated together.

---

## Important Relationships

```
Organization
  └─ Job
       ├─ HiringTeamMembers (who evaluates)
       ├─ ApplicantTags (the evaluation taxonomy)
       ├─ Prompts (interview questions from prompts_service)
       └─ Applicants
            └─ ApplicantReviews (feedback that drives state)
```

**Cross-service dependencies:**
- `prompts_service`: Interview prompts, response completion status
- `analysis_service`: AI-generated applicant summaries
- `users_service`: User profiles, organizations
- `assets_service`: Resume storage

---

## Common Workflows

### 1. Application Submission
```
createApplicant(jobId, userId)
→ Applicant created (status: APPLIED)
→ Submission deadline assigned (job default or extended)
→ Slack notification to hiring team
```

### 2. Application Completion
```
User answers required prompts (prompts_service)
→ Applicant.status updates to COMPLETED
→ Available for review once deadline passes
```

### 3. Review and Evaluation
```
Hiring team member creates ApplicantReview with tags
→ System auto-derives stateTagId and bucketTagId from reviews
→ Slack notification if state changed (unless reviewer is MemberPlacement)
```

### 4. Check Available State Transitions
```graphql
query {
  applicant(id: "...") {
    stateTag { tag }
    nextStateTags { id tag }  # Valid next states based on current state
  }
}
```

---

## Core Types

### Job

A job posting owned by an organization.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `title` | `String!` | Job title |
| `status` | `JobStatus!` | DRAFT, OPEN, IN_REVIEW, CLOSED |
| `organizationId` | `Id!` | Owning organization |
| `submissionDeadline` | `DateTime!` | Default application deadline |
| `location` | `String` | Location text |
| `workplace` | `Workplace` | Remote/hybrid/onsite |
| `employmentType` | `EmploymentType` | Full-time, part-time, etc. |

**Key resolvers:**
- `applicants(stateTagId: Id)` - All applicants (optionally filtered)
- `applicant(userId: Id!)` - Single applicant by user
- `applicantCount(states, stateTagIds)` - Count without loading
- `hiringTeam(role: HiringTeamRole)` - Team members
- `organization` - Parent organization
- `prompts` / `supplementalPrompts` - Interview prompts
- `applicantTags` / `applicantBucketTags` / `applicantStateTags` - Tag categories

### Applicant

A user's application to a job.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `jobId` | `Id!` | Associated job |
| `userId` | `Id!` | Applying user |
| `status` | `ApplicantStatus!` | Current lifecycle state |
| `bucketTagId` | `Id` | Current categorization (derived from reviews) |
| `stateTagId` | `Id` | Current workflow state (derived from reviews) |
| `submissionDeadline` | `DateTime!` | Deadline for this applicant |

**Statuses:** APPLIED, COMPLETED, NOT_COMPLETED, SUBMITTED, REJECTED, ACCEPTED, WITHDRAWN, HIRED

**Key resolvers:**
- `user` - User profile
- `job` - Parent job
- `reviews` - All reviews for this applicant
- `summary` - AI-generated summary (from analysis_service)
- `bucketTag` / `stateTag` - Current tags
- `nextStateTags` - Valid state transitions from current state
- `interviewUrl(sessionId)` - Interview link

### ApplicantTag

Tags for organizing and evaluating applicants within a job.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `tag` | `String!` | Display name |
| `category` | `ApplicantTagCategory!` | FEEDBACK, BUCKET, or STATE |
| `order` | `Float!` | Sort order |
| `nextStateTagIds` | `[Id!]!` | Valid transitions (STATE tags only) |

### ApplicantReview

Hiring team feedback on an applicant. Creating a review auto-updates the applicant's state/bucket tags.

| Field | Type | Description |
|-------|------|-------------|
| `applicantId` | `Id!` | Associated applicant |
| `reviewerId` | `Id!` | Hiring team member who reviewed |
| `tagIds` | `[Id!]!` | Applied tags (any category) |
| `notes` | `String` | Optional text feedback |

### HiringTeamMember

Links users to jobs with specific roles.

| Field | Type | Description |
|-------|------|-------------|
| `jobId` | `Id!` | Associated job |
| `user` | `UserProfile!` | Team member |
| `roles` | `[HiringTeamRole!]!` | HIRING_MANAGER, MEMBER_PLACEMENT |

### JobPost

External job board posting linked to a job.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `jobId` | `Id!` | Parent job |
| `status` | `JobPostStatus!` | DRAFT, ACTIVE, PAUSED, CLOSED |
| `destinations` | `[JobPostDestination!]!` | LINKEDIN, etc. |

---

## Queries

### Lookups

```graphql
# Single job
job(id: Id!): Job

# Multiple jobs by ID
jobs(ids: [Id!]!): [Job!]!

# Single applicant
applicant(id: Id!): Applicant

# Multiple applicants
applicants(ids: [Id!]!): [Applicant!]!

# Applicants for a user
applicantsForUser(userId: Id!): [Applicant!]!
```

### Lists with Pagination

```graphql
# Jobs with filtering
listJobsV2(
  limit: Int = 25
  cursor: String
  sort: JobSort = { field: CREATED_AT, direction: ASC }
  filter: [JobFilterCondition!]
): JobConnection!

# Applicants with filtering
listApplicants(
  limit: Int = 25
  cursor: String
  sort: ApplicantSort
  filter: [ApplicantFilterCondition!]
): ApplicantConnection!

# Job posts
listJobPosts(
  limit: Int = 25
  cursor: String
  sort: JobPostSort
  filter: [JobPostFilterCondition!]
): JobPostConnection!
```

### Filter Fields

**Job:** STATUS, ORGANIZATION_ID, ORGANIZATION_NAME, TITLE

**Applicant:** STATUS, JOB_ID, USER_ID, REQUIRES_SPONSORSHIP, BUCKET_TAG_ID, STATE_TAG_ID, SUBMISSION_DEADLINE

**JobPost:** JOB_ID, STATUS

---

## Query Examples

### Get applicants for a job with review state

```graphql
query {
  job(id: "...") {
    applicants {
      id
      status
      user { fullName }
      bucketTag { tag }
      stateTag { tag }
      nextStateTags { id tag }
    }
  }
}
```

### Filter applicants by workflow state

```graphql
query {
  listApplicants(
    filter: [
      { field: JOB_ID, value: { id: "..." } }
      { field: STATE_TAG_ID, value: { id: "..." } }
    ]
  ) {
    nodes {
      id
      user { fullName }
      stateTag { tag }
    }
  }
}
```

### Get job with hiring team

```graphql
query {
  job(id: "...") {
    title
    hiringTeam { user { fullName } roles }
    hiringManager { fullName }
  }
}
```

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `Job.organization` | users | Organization owning the job |
| `Job.prompts` | prompts | Interview prompts for applicants |
| `Applicant.user` | users | Applying user profile |
| `Applicant.summary` | analysis | AI-generated applicant summary |
| `Applicant.resume` | assets | Uploaded resume |
