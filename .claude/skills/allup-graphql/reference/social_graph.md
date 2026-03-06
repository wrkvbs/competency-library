# Social Graph Service Reference

## Overview

The social_graph_service is the **central hub for managing all relationship data** - the nervous system connecting the career graph, interview workflow, response system, and user network. It manages user-to-organization relationships (affiliations/work history), user-to-user relationships (associations), and auto-derived relationships from the prompts system (team members). Uses Neo4j for graph storage and traversals, and OpenSearch for organization text search.

---

## Key Concepts

### Affiliation (User-to-Organization)

Work history records linking users to organizations. Key characteristics:

- **Dual-source creation**: Can come from users directly OR from invites (when an invite is accepted, its affiliations migrate to the real user)
- **Status flow**: `NEW` -> `APPROVED` or `REJECTED`. Approval triggers OpenSearch indexing if the org was auto-created
- **Visibility as privacy layer**: `PUBLIC`, `PRIVATE`, `UNLISTED` - auto-sets based on user's discoverability setting
- **Response tracking**: `responseCount` tracks how many video responses exist for this user-org pair

### Association (User-to-User)

How two people know each other. Key characteristics:

- **Directional**: From user A to user B (asymmetric - each person records their own view)
- **Anchored to shared work history**: Can optionally reference an Affiliation ("they know each other from working at the same company")
- **Duration context**: Tracks `yearsCount` (how long they worked together) and `yearsAgo` (when)

### RelationshipType (12+ types with reciprocal logic)

| Type | Reciprocal |
|------|------------|
| `BOSS` | `DIRECT_REPORT` |
| `MENTOR` | `MENTEE` |
| `INVESTOR` | `INVESTEE` |
| `COWORKER` | `COWORKER` |
| `SERVICE_PROVIDER` | `CUSTOMER`/`CLIENT` |

Other types: `FRIEND`, `FOUNDER_INVESTED_IN`, `EMPLOYEE`, `OTHER`, `YOU`, `UNKNOWN`

### TeamMember (Derived, Not User-Curated)

Auto-created relationships tracking who has recorded video responses about whom:

- **Event-driven**: Created when a user completes a video response about another user (from prompts_service)
- **Not editable**: System-managed, not user-curated like affiliations/associations
- **Increments affiliation response count**: When created, also updates the affiliation's `responseCount`

### Organization

Company or organization entity with dual storage:

- **Neo4j**: Graph relationships and traversals
- **OpenSearch**: Text search (indexed when affiliations are approved for auto-created orgs)

Can be auto-created with minimal data (just a name) and enriched later.

### Visibility as Privacy Layer

User discoverability setting controls affiliation visibility system-wide:

- When a user toggles their discoverable setting, **all their affiliations flip visibility**
- Event-driven via Lambda handler
- `PUBLIC`: Visible to everyone
- `PRIVATE`: Visible only to the user
- `UNLISTED`: Not searchable but accessible via direct link

---

## Important Relationships

```
User (users_service)
  ├─ UserProfile (stub in social_graph)
  │     ├─ Affiliations (work history)
  │     │     └─ Organization
  │     ├─ Associations (who they know)
  │     │     └─ → other UserProfile
  │     │           └─ (optional) anchored to shared Affiliation
  │     └─ TeamMembers (who recorded responses for them)
  │
  └─ Invites (pending relationships)
        ├─ Affiliations (pre-acceptance)
        └─ Associations (pre-acceptance)

Organization
  ├─ Users (affiliated)
  └─ Jobs (from jobs_service)
```

**Cross-service dependencies:**
- `users_service`: User profiles, user discoverability settings (triggers visibility updates)
- `prompts_service`: Response completion events create TeamMember relationships, increment response counts
- `jobs_service`: Jobs linked to organizations
- `asset_service`: User avatars

---

## Common Workflows

### 1. Response Completion -> TeamMember Created

```
User A completes video response about User B (prompts_service)
→ TeamMember(fromUserId: A, toUserId: B) created
→ Affiliation.responseCount incremented for the relevant user-org pair
→ PubSub event emitted
```

### 2. User Discoverability Change -> Visibility Flip

```
User toggles discoverable setting (users_service)
→ Lambda handler fires
→ All user's affiliations flip visibility (PUBLIC ↔ PRIVATE)
```

### 3. Affiliation Approval

```
createAffiliation or updateAffiliation with status: APPROVED
→ If organization was auto-created, index in OpenSearch
→ PubSub event emitted
```

### 4. Invite Acceptance -> Relationship Migration

```
acceptInvite(inviteId, userId)
→ Invite's affiliations migrate to real user
→ Invite's associations migrate to real user
→ Invite status set to ACCEPTED
```

### 5. Finding Common Ground Between Users

```graphql
query {
  userProfile(id: "user-a") {
    # Get their default affiliation in context of another user
    affiliation(otherUserId: "user-b") {
      organization { name }
    }
    # Get their relationship to another user
    association(toUserId: "user-b") {
      relationshipType
      yearsCount
    }
  }
}
```

---

## Core Types

### UserProfile

User stub with social graph data. Full user data lives in users_service.

| Field | Type | Description |
|-------|------|-------------|
| `userId` | `Id!` | User identifier |
| `fullName` | `String` | Display name |
| `affiliations` | `[Affiliation!]!` | Work history (optional `status` filter) |
| `affiliation` | `Affiliation` | Default affiliation (optional `otherUserId` for common affiliation) |
| `association` | `Association` | Relationship to another user |
| `socialLinks` | `[UserSocialLink!]!` | LinkedIn, Twitter, etc. |

### Organization

Company or organization entity.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Organization identifier |
| `name` | `String!` | Organization name |
| `users` | `[UserProfile!]!` | Users affiliated with this org |
| `jobs` | `[Job!]!` | Jobs for this organization (requires `status` filter) |

### Affiliation

User-to-organization relationship (work history). Stored as Neo4j relationship.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Affiliation identifier |
| `userId` | `Id!` | User who has this affiliation |
| `organizationId` | `Id!` | Organization |
| `organization` | `Organization` | Resolved organization |
| `title` | `String` | Job title |
| `startedAt` | `DateTime` | Start date |
| `endedAt` | `DateTime` | End date (null = current) |
| `status` | `AffiliationStatus!` | `NEW`, `APPROVED`, `SUGGESTED`, `REJECTED` |
| `visibility` | `AffiliationVisibility!` | `PUBLIC`, `PRIVATE`, `UNLISTED` |
| `responseCount` | `Int` | Number of video responses for this user-org pair |

### Association

User-to-user relationship with type and duration. Stored as Neo4j relationship.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Association identifier |
| `fromUserId` | `Id!` | Source user |
| `toUserId` | `Id!` | Target user |
| `relationshipType` | `RelationshipType!` | Type of relationship |
| `relationshipTypeDetail` | `RelationshipTypeDetail!` | Detailed type info |
| `yearsCount` | `Int` | Duration in years |
| `yearsAgo` | `Int` | How long ago |
| `affiliationId` | `Id` | Optional shared affiliation (anchor point) |
| `status` | `AssociationStatus!` | `NEW`, `APPROVED`, `REJECTED` |

### RelationshipType (enum)

`BOSS`, `COWORKER`, `FRIEND`, `DIRECT_REPORT`, `MENTOR`, `MENTEE`, `SERVICE_PROVIDER`, `CUSTOMER`, `CLIENT`, `INVESTOR`, `INVESTEE`, `FOUNDER_INVESTED_IN`, `OTHER`, `YOU`, `UNKNOWN`, `EMPLOYEE`

### TeamMember

Auto-managed relationship tracking who has recorded responses for a user.

| Field | Type | Description |
|-------|------|-------------|
| `fromUserId` | `Id!` | User who recorded the response |
| `toUserId` | `Id!` | Subject user |
| `fromUser` | `UserProfile` | Resolved recorder |
| `toUser` | `UserProfile` | Resolved subject |

### Invite

User invitation record. Carries pre-acceptance affiliations and associations.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Invite identifier |
| `digests` | `[String!]!` | Contact digests (phone/email hashes) |
| `fullName` | `String` | Invitee name |
| `fromUserId` | `Id!` | Inviting user |
| `status` | `InviteStatus!` | `NEW`, `ACCEPTED` |

### SocialLink / UserSocialLink

Platform-specific social links (LinkedIn, Twitter, etc.).

---

## Queries

### Lookup

```graphql
affiliation(id: Id!): Affiliation
association(id: Id!): Association
organization(id: Id!): Organization
userProfile(id: Id!): UserProfile
userProfileBySlug(userId: Id, slug: String): UserProfile
invite(id: Id!): Invite
```

### Batch Lookup

```graphql
affiliations(userId: Id!, status: AffiliationStatus): [Affiliation!]!
associations(userId: Id!): [Association!]!
organizations(ids: [Id!]!): [Organization!]!
userProfiles(ids: [Id!]!): [UserProfile!]!
invites(ids: [Id!]!): [Invite!]!
```

### Relationship Queries

```graphql
associationForUsers(fromUserId: Id!, toUserId: Id!): Association!
associationsForUsers(fromUserId: Id!, toUserId: Id!): [Association!]!
teamMember(fromUserId: Id!, toUserId: Id!): TeamMember!
teamMembers(userId: Id!): [TeamMember!]!
connectedUsers(userId: Id!): [UserProfile!]!
```

### Search

```graphql
searchOrganizations(query: OrganizationSearch!): [Organization!]!
searchAssociations(query: AssociationSearch!): [Association!]!
searchUserProfiles(query: UserProfileSearch!): [UserProfile!]!
searchInvites(filter: InviteSearch!): [Invite!]!
```

### Discovery

```graphql
suggestedOrganizations(userId: Id!): [Organization!]!
suggestedUsers(userId: Id!): [UserProfile!]!
topOrganizations(limit: Int): [Organization!]!
```

### List with Pagination

```graphql
listUserProfiles(
  limit: Int
  cursor: String
  sort: UserProfileSort!
  filter: [UserProfileFilterCondition!]
): UserProfileConnection!
```

---

## Query Examples

### Get a user's work history with response counts

```graphql
query {
  userProfile(id: "...") {
    fullName
    affiliations(status: APPROVED) {
      organization { name }
      title
      startedAt
      endedAt
      responseCount
    }
  }
}
```

### Find how two users know each other

```graphql
query {
  associationsForUsers(fromUserId: "user-a", toUserId: "user-b") {
    relationshipType
    yearsCount
    yearsAgo
    # If anchored to a shared work experience
    affiliation {
      organization { name }
    }
  }
}
```

### Get team members (who recorded responses for a user)

```graphql
query {
  teamMembers(userId: "...") {
    fromUser { fullName }
    toUser { fullName }
  }
}
```

### Search organizations by name

```graphql
query {
  searchOrganizations(query: { name: "Acme" }) {
    id
    name
    users { fullName }
  }
}
```

---

## Mutations

### Affiliations

```graphql
createAffiliation(payload: AffiliationCreate!): Affiliation!
updateAffiliation(payload: AffiliationUpdate!): Affiliation!
deleteAffiliation(id: Id!): Id
```

### Associations

```graphql
createAssociation(payload: AssociationCreate!): Association!
updateAssociation(payload: AssociationUpdate!): Association!
deleteAssociation(id: Id!): Id
```

### Organizations

```graphql
createOrganization(payload: OrganizationCreate!): Organization!
updateOrganization(payload: OrganizationUpdate!): Organization!
deleteOrganization(id: Id!): Id
```

### Invites

```graphql
createInvite(payload: InviteCreate!): Invite!
updateInvite(payload: InviteUpdate!): Invite!
acceptInvite(inviteId: Id!, userId: Id!): Boolean!
deleteInvite(id: Id!): Id
```

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `UserProfile` | users | Full user data, discoverability settings |
| `UserProfile.avatar` | assets | User avatar asset |
| `Organization.jobs` | jobs | Jobs posted by organization |
| `Affiliation` | prompts | Response completion events increment responseCount |
| `TeamMember` | prompts | Created when response about a user is completed |
| `Invite` | users | Migrates to real user on acceptance |
