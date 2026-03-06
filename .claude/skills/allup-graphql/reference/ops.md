# Ops Service Reference

## Overview

The ops_service is the **operational management layer** for the allUP platform - not user-facing product functionality. It provides system observability through process status tracking, infrastructure event handling, and a generic annotation system. While it participates in GraphQL federation, most of its work happens outside the supergraph: Lambda handlers for AWS events, automated deployments, Slack notifications, and incident management.

---

## Key Concepts

### Two Distinct Subsystems

The ops_service has two fundamentally different data models with different persistence characteristics:

| Subsystem | Storage | TTL | Purpose |
|-----------|---------|-----|---------|
| **ProcessStatus** | Redis | 10 minutes | Ephemeral runtime state of service instances |
| **AdminComments** | MongoDB | Permanent | Persistent annotations on any entity |

### ProcessStatus Lifecycle

ProcessStatus provides a **real-time view of running service instances** across the infrastructure:

```
Service Startup
  → opsSetProcessStatus (register instance)
  → Every N seconds: heartbeat update (refreshes lastUpdated)
  → Graceful shutdown: opsDeregisterProcessStatus
  → Crash/timeout: TTL expires after 10 minutes of silence
```

The 10-minute TTL acts as an automatic cleanup mechanism - if a service crashes without deregistering, it simply disappears from the registry after 10 minutes.

Key fields in ProcessStatus:
- `instanceId` - Unique identifier for this running instance
- `serviceName` - Which service (e.g., "jobs_service", "users_service")
- `buildSha` - Git commit SHA for version tracking
- `serviceDataJson` - Extensible JSON blob for service-specific metrics

### AdminComments (Generic Annotation System)

AdminComments enable **thread-based internal comments on any entity** in the system. The `topic` field is a generic GlobalId that can reference any entity type - users, jobs, applicants, organizations, etc.

This is for internal team use (hiring notes, support annotations) - not user-visible comments.

### DeploymentReport

A scheduled job that generates a **cross-service deployment snapshot** showing which branches are ahead of production/staging. Queries GitHub API to compare branch states across all service repositories.

- Runs on cron: 9am weekdays (Pacific Time)
- **Dev environment only** - disabled in staging/production
- Sends summary to Slack #eng-notifications

### Lambda Event Handlers (Outside Federation)

Most ops_service functionality lives outside GraphQL in Lambda functions:

| Handler | Trigger | Action |
|---------|---------|--------|
| **ECS Deployment Complete** | ServiceDeploymentCompleted | Slack notification |
| **ECS Task Crash** | TaskStateChange (STOPPED) | Slack notification with crash details |
| **ECR Autodeploy** | ECR image push | Force-update ECS service to new image |
| **CloudWatch Alarms** | Alarm state change | Create SSM incident (production only) |
| **DMS Auto-Recovery** | Replication task failure | Auto-resume replication |

### Environment-Specific Behavior

| Feature | Dev | Staging | Production |
|---------|-----|---------|------------|
| Deployment report cron | Yes | No | No |
| SSM incident creation | No | No | Yes (when ARN configured) |
| Slack notifications | Yes | Yes | Yes |

---

## Important Relationships

```
OpsProcessStatus
  └─ Tracks instances of → All Services

AdminCommentMessage
  ├─ topic → Any Entity (GlobalId)
  └─ author → UserProfile (users_service)

Lambda Handlers
  ├─ ECS Events → Slack #eng-notifications
  ├─ ECR Push → ECS Service Update
  ├─ CloudWatch Alarm → SSM Incident (production)
  └─ DMS Failure → Auto-Recovery
```

**Cross-service dependencies:**
- `users_service`: AdminComment author resolves to UserProfile
- `All services`: ProcessStatus tracks their running instances via heartbeats

---

## Common Workflows

### 1. Service Heartbeat Pattern
```
Service starts up
→ Calls opsSetProcessStatus with instance details
→ Periodically calls opsSetProcessStatus to refresh TTL
→ On shutdown: opsDeregisterProcessStatus
→ If crashed: Redis TTL expires after 10 minutes
```

### 2. Viewing Active Instances
```graphql
query {
  opsListProcessStatuses(filter: { serviceName: "jobs_service" }) {
    instanceId
    buildSha
    startedAt
    lastUpdated
  }
}
```

### 3. Adding Internal Notes to an Entity
```graphql
mutation {
  adminCreateComment(payload: {
    topicId: "Applicant:abc123"  # Any GlobalId
    message: "Spoke with candidate, they need 2 weeks notice"
  }) {
    id
    createdAt
    author { fullName }
  }
}
```

### 4. Reviewing Comments on an Entity
```graphql
query {
  adminCommentsForTopic(topicId: "Applicant:abc123") {
    message
    author { fullName }
    createdAt
  }
}
```

---

## Core Types

### OpsProcessStatus

Ephemeral record of a running service instance (Redis-backed, 10-minute TTL).

| Field | Type | Description |
|-------|------|-------------|
| `instanceId` | `Id!` | Unique identifier for this instance |
| `serviceName` | `String!` | Service name (e.g., "jobs_service") |
| `repository` | `String` | Source repository |
| `buildSha` | `String` | Git commit SHA |
| `startedAt` | `DateTime!` | When instance started |
| `lastUpdated` | `DateTime!` | Last heartbeat time |
| `serviceDataJson` | `String!` | Custom service-specific metrics (JSON) |

```graphql
type OpsProcessStatus {
  instanceId: Id!
  serviceName: String!
  repository: String
  buildSha: String
  startedAt: DateTime!
  lastUpdated: DateTime!
  serviceDataJson: String!
}
```

### AdminCommentMessage

Persistent thread-based comment on any entity (MongoDB-backed).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `topic` | `Id!` | GlobalId of the entity this comment is attached to |
| `authorId` | `Id!` | User who wrote the comment |
| `author` | `UserProfile!` | Federated user profile |
| `message` | `String!` | Comment text |
| `createdAt` | `DateTime!` | When created |
| `updatedAt` | `DateTime!` | When last modified |

```graphql
type AdminCommentMessage {
  id: Id!
  topic: Id!
  authorId: Id!
  author: UserProfile!  # Federated from users_service
  message: String!
  createdAt: DateTime!
  updatedAt: DateTime!
}
```

### ListProcessStatusesFilter

```graphql
input ListProcessStatusesFilter {
  serviceName: String
  repository: String
}
```

---

## Queries

```graphql
# Health check
opsPing: String!  # Returns "PONG"

# List running service instances
opsListProcessStatuses(filter: ListProcessStatusesFilter): [OpsProcessStatus!]!

# Single admin comment by ID
adminComment(id: Id!): AdminCommentMessage

# All comments for an entity
adminCommentsForTopic(topicId: Id!): [AdminCommentMessage!]!
```

---

## Mutations

```graphql
# Process status registration (called by services)
opsSetProcessStatus(status: OpsProcessStatusInput!): Boolean!
opsDeregisterProcessStatus(processId: Id!): Boolean!

# Admin comments CRUD
adminCreateComment(payload: AdminCommentCreate!): AdminCommentMessage!
adminUpdateComment(payload: AdminCommentUpdate!): AdminCommentMessage!
adminDeleteComment(id: Id!): Boolean!
```

---

## Query Examples

### List all running instances of a service

```graphql
query {
  opsListProcessStatuses(filter: { serviceName: "prompts_service" }) {
    instanceId
    buildSha
    startedAt
    lastUpdated
  }
}
```

### Get comments for an applicant

```graphql
query {
  adminCommentsForTopic(topicId: "Applicant:abc123") {
    id
    message
    author { fullName email }
    createdAt
  }
}
```

### Check service health across instances

```graphql
query {
  opsListProcessStatuses {
    serviceName
    instanceId
    buildSha
    lastUpdated
  }
}
```

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `AdminCommentMessage.author` | users | Author's UserProfile |
| `AdminCommentMessage.topic` | any | Generic GlobalId reference to any entity |
| `OpsProcessStatus` | all | Tracks running instances of all services |
