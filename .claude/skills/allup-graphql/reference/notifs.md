# Notif Service Reference

## Overview

Despite its name, the notif_service is the **deep link resolution and URL routing service** - the platform's address book and router, not a notification sender. It translates between three URL representations (deep links, web URLs, and GraphQL Navigation types) while handling platform-specific routing for mobile apps and web clients. The naming is a historical artifact.

---

## Key Concepts

### Three URL Representations

The service manages three ways to represent the same destination:

| Representation | Format | Use Case |
|----------------|--------|----------|
| **Deep Links** | `link.allup.world/{token}` | Mobile-friendly, opaque tokens for push notifications and QR codes |
| **Web URLs** | `allup.world/...` | Standard web URLs for browsers and sharing |
| **GraphQL Navigation** | `Navigation` type | Structured data telling clients what screen to render |

### Link vs SavedLink (Critical)

The service follows a philosophy of **"prefer not creating a record when it's an option"**:

| Type | Storage | When to Use |
|------|---------|-------------|
| **Link** | None (stateless) | Simple references to existing entities. Encodes GlobalId directly into token. |
| **SavedLink** | MongoDB | Complex scenarios requiring variants, parameter forwarding, or durability |

A `Link` is an intermediate representation that can reference:
- A `SavedLink` (persisted)
- A `GlobalId` (encoded directly, no DB record)
- A `DynamicLinkBehavior` (special routing like `/jawf`, `/int`)

### Platform-Specific Routing

The same link can behave differently based on device/platform:

```
Single deep link → resolves to:
  - iOS app: Open interview screen
  - Android browser: Web interview page
  - Desktop: Full web experience
```

This is achieved through **LinkVariant** - conditional routing rules attached to SavedLinks.

### LinkedResource Union

All linkable entities in the platform:

```
LinkedResource = UserProfile | Interview | Job | Applicant |
                 Organization | ChatChannel | ClipGroup |
                 PromptResponse | PromptTag | InterviewRequest | URL
```

### NavigationSession (Multi-Device Workflows)

Redis-backed temporary state with TTL for workflows spanning devices:
- User scans QR code on mobile
- Web client polls `navigationSessionNext` waiting for completion
- Mobile completes action, updates session
- Web receives Navigation and proceeds

### Dynamic Link Behaviors

Special URL patterns that route dynamically:

| Pattern | Purpose |
|---------|---------|
| `/jawf?applicant_id=123` | Job application onboarding, routes differently per platform |
| `/int` | Interview links with encoded context (interview ID, responder, subject) |

### ForwardedParam

Translates parameter names between schemas when links cross system boundaries. Enables parameter forwarding from deep links to destination URLs with name mapping.

---

## Important Relationships

```
Link (stateless encoding)
  ├─ GlobalId → direct entity reference
  ├─ SavedLink → persisted link
  │     └─ LinkVariants (per-platform routing)
  │           └─ ForwardedParams (parameter translation)
  └─ DynamicLinkBehavior → special routing rules

Navigation (resolution output)
  ├─ resource: LinkedResource (what to show)
  ├─ page: NavigationPage (where to render)
  └─ params (additional context)

NavigationSession (Redis, TTL)
  └─ Temporary state for multi-device workflows
```

**Cross-service dependencies:**
- `users_service`: UserProfile, Organization entities
- `prompts_service`: Interview, PromptTag, PromptResponse, ClipGroup entities
- `jobs_service`: Job, Applicant entities
- `social_graph_service`: ChatChannel entity

---

## Common Workflows

### 1. Simple User Link (No DB Record)

```
createLink(refId: "User:abc123")
→ GlobalId encoded directly into token
→ Returns Link with token and URL
→ No database record created
```

### 2. Context-Rich Interview Link

```
createLink(refId: "Interview:xyz789", refContext: { responderUserId: "User:abc" })
→ Encodes interview ID + who's responding
→ On resolution, Navigation includes full context
```

### 3. Platform-Specific Saved Link

```
createSavedLink(payload: {
  refId: "Interview:xyz"
  refVariants: [
    { platform: IOS, refId: "..." }
    { platform: ANDROID, refUrl: "https://..." }
    { platform: DEFAULT, refId: "..." }
  ]
})
→ Persisted with routing rules
→ Same token resolves differently per platform
```

### 4. Multi-Device QR Code Flow

```
1. Web: createNavigationSession → returns sessionId
2. Web: Display QR code encoding sessionId
3. Mobile: Scans QR, completes action, updates session
4. Web: Poll navigationSessionNext(sessionId) → receives Navigation
5. Web: Navigate to resolved destination
```

---

## Core Types

### Navigation

Resolved deep link result - tells clients what to render.

| Field | Type | Description |
|-------|------|-------------|
| `resource` | `LinkedResource` | The entity to display (union type) |
| `page` | `NavigationPage` | In-app destination |
| `params` | `NavigationParams` | Additional context parameters |

**NavigationPage values:** `RECORD_PAGE`, `CHAT_LIST`, `YOUVE_GOT_CLIPS_FLOW`, `UP_CENTER`

### Link

Serializable link representation (may or may not be persisted).

| Field | Type | Description |
|-------|------|-------------|
| `token` | `String!` | Opaque identifier for deep link URL |
| `url` | `Url!` | Full deep link URL (`link.allup.world/{token}`) |

### SavedLink

Database-persisted link with optional platform variants.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `refId` | `Id` | Default target GlobalId |
| `refUrl` | `Url` | Default target URL |
| `variants` | `[LinkVariant!]` | Platform-specific routing rules |

### LinkVariant

OS/client-specific routing rule within a SavedLink.

| Field | Type | Description |
|-------|------|-------------|
| `platform` | `Platform!` | IOS, ANDROID, WEB, DEFAULT |
| `refId` | `Id` | Target GlobalId for this platform |
| `refUrl` | `Url` | Target URL for this platform |
| `forwardedParams` | `[ForwardedParam!]` | Parameter name translations |

### LinkContext

Context parameters for link resolution.

| Field | Type | Description |
|-------|------|-------------|
| `responderUserId` | `Id` | User who should respond |
| `subjectId` | `Id` | Subject of the interaction |
| `tagId` | `Id` | Associated tag |
| `interviewId` | `Id` | Associated interview |
| `topicIds` | `[Id!]` | Related topics |

### LinkedResource (Union)

All resolvable entity types:

```graphql
union LinkedResource =
  UserProfile | Interview | Job | Applicant |
  Organization | ChatChannel | ClipGroup |
  PromptResponse | PromptTag | InterviewRequest |
  ResolvedLinkURL
```

### Messaging Types

| Type | Description |
|------|-------------|
| `Broadcast` | Customer.io broadcast definition |
| `Event` | Analytics event definition |
| `AlertPushNotification` | Input for visible push notifications (title, body, fields) |
| `BackgroundPushNotification` | Input for silent data pushes |

---

## Queries

### Link Resolution

```graphql
# Resolve a deep link token (from push notifications)
resolveLinkToken(token: String!, context: LinkContext): Navigation

# Resolve a deep link URL (from web/email)
resolveLinkUrl(url: Url!): Navigation!

# Resolve multiple GlobalIds to Navigation
resolveTopics(topics: [Id!]!): [ResolvedTopic!]!

# Look up saved link by ID
savedLink(id: Id!): SavedLink
```

### Navigation Sessions

```graphql
# Poll for navigation session readiness (multi-device workflows)
navigationSessionNext(sessionId: Id!): Navigation
```

### List Queries

```graphql
broadcasts: [Broadcast!]!
events: [Event!]!
```

---

## Query Examples

### Resolve and Navigate

```graphql
query {
  resolveLinkToken(token: "abc123xyz") {
    page
    params
    resource {
      ... on UserProfile { userId fullName }
      ... on Interview { id interviewUrl }
      ... on Job { id title }
      ... on Applicant { id status }
    }
  }
}
```

### Resolve with Context

```graphql
query {
  resolveLinkToken(
    token: "abc123xyz"
    context: { responderUserId: "User:def456" }
  ) {
    page
    resource {
      ... on Interview {
        id
        interviewUrl
      }
    }
  }
}
```

### Look Up Saved Link Details

```graphql
query {
  savedLink(id: "SavedLink:xyz") {
    id
    refId
    refUrl
    variants {
      platform
      refId
      refUrl
    }
  }
}
```

---

## Mutations

### Link Management

```graphql
# Create a deep link (prefers stateless - no DB record if possible)
createLink(payload: LinkCreate!): Link!

# Create a saved deep link (DB-persisted, for complex scenarios)
createSavedLink(payload: LinkCreate!): SavedLink!

# Update a saved link
updateSavedLink(payload: LinkUpdate!): SavedLink!

# Initialize a navigation session (multi-device workflows)
createNavigationSession: Id!
```

### Broadcasts and Events

```graphql
createBroadcast(payload: BroadcastCreate!): Broadcast!
updateBroadcast(payload: BroadcastUpdate!): Boolean!
deleteBroadcast(id: Id!): Boolean!
triggerBroadcast(payload: BroadcastTrigger!): Boolean!

createEvent(payload: EventCreate!): Event!
updateEvent(payload: EventUpdate!): Boolean!
deleteEvent(id: Id!): Boolean!
trackEvent(payload: EventTrack!): Boolean!
```

### Push Notifications

```graphql
# Visible notification to all user devices
sendAlertPushNotification(payload: AlertPushNotification!): Id!

# Silent data push for background processing
sendBackgroundPushNotification(payload: BackgroundPushNotification!): Id!
```

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `LinkedResource.UserProfile` | users | User profiles as link targets |
| `LinkedResource.Organization` | users | Organizations as link targets |
| `LinkedResource.Interview` | prompts | Interviews as link targets |
| `LinkedResource.PromptTag` | prompts | Prompt tags as link targets |
| `LinkedResource.PromptResponse` | prompts | Prompt responses as link targets |
| `LinkedResource.ClipGroup` | prompts | Clip groups as link targets |
| `LinkedResource.Job` | jobs | Jobs as link targets |
| `LinkedResource.Applicant` | jobs | Applicants as link targets |
| `LinkedResource.ChatChannel` | social_graph | Chat channels as link targets |
