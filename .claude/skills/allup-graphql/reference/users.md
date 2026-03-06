# Users Service Reference

## Overview

The users_service is a **multi-faceted user identity and lifecycle management system** - not just authentication. It handles the full user journey: SMS-based passwordless auth (working around Cognito limitations), user/profile identity separation, privacy-aware contact discovery, invite resolution, granular onboarding orchestration, and visibility/intent controls for discovery surfaces.

---

## Key Concepts

### User vs UserProfile (Critical Distinction)

These are **two separate entities** with different purposes:

| Type | Contains | Use When |
|------|----------|----------|
| **User** | Cognito auth record - email, phone, verification status, onboarding state | Need auth state, contact info, or sensitive data |
| **UserProfile** | Public-facing data - name, avatar, headline, visibility, social links | Displaying user info to others |

A User always has a UserProfile (via `user.profile`), but you can query UserProfile directly without loading User.

### Custom SMS-Based Passwordless Auth

The auth flow works around Cognito limitations with a **signup vs login distinction**:

- **Signup**: `initiateSignupAuth` - creates new Cognito user
- **Login**: `initiateLoginAuth` - authenticates existing user
- Both use SMS confirmation codes, not passwords
- JWT tokens track session state
- Phone number is the primary identifier

### UserDigests (Privacy-Aware Contact Discovery)

UserDigests enable "who has this person in their contacts?" lookups without storing plaintext:

- Hashed versions of email/phone stored with owning user
- When a new user signs up, system finds users with matching digest
- Triggers "someone you know joined" push notifications
- **Never exposes the actual contact info** - only signals the connection

### Invite System with Auto-Resolution

Invites are created with digests, not user IDs:

```
Create invite with digests → User registers with matching phone/email → Invite auto-resolves to UserProfile
```

This allows inviting people before they're on the platform.

### Onboarding Director (WebSocket-Based)

The onboarding system is a stateful WebSocket connection:

- **URL**: `/onboarding_director/{version}/{user_id}`
- Streams UI components to client progressively
- Tracks **17+ granular requirements** separately in `UserOnboardingState`
- Requirements marked complete persist across sessions
- Supports debug mode for development

### Grant Tokens (Privilege Escalation)

Pre-validated tokens that grant permissions when applied:

- User receives token (via link, invitation, etc.)
- On login, token is applied to user
- **Auto-creates**: hiring team roles, affiliations, org memberships
- Enables onboarding flows that need specific permissions

### Discovery Surfaces

Controls where users appear in search and listings:

| Surface | Description |
|---------|-------------|
| `Google` | Public search engines |
| `AllUpSearch` | Internal search results |
| `TagPages` | Skill/category tag pages |
| `CompanyPages` | Organization member listings |

Users can enable/disable each surface independently.

### Intent Flags

Temporary, expiring user state markers:

- Displayed on profiles (e.g., "Actively looking")
- Have color coding for UI display
- Expire automatically after set duration
- Used for job search status, availability signals

---

## Important Relationships

```
User (Cognito auth)
  └─ UserProfile (public data)
       ├─ Affiliations → Organizations
       ├─ Associations → Other UserProfiles
       ├─ IntentFlags (temporary states)
       ├─ SocialLinks (external profiles)
       ├─ PromptTags (from prompts_service)
       └─ Applicants (from jobs_service)

Organization
  ├─ OrganizationMembers (roles: ADMIN)
  ├─ Jobs (from jobs_service)
  └─ Affiliations ← UserProfiles
```

**Cross-service dependencies:**
- `prompts_service`: Interview requests created on signup, PromptTags for skills
- `social_graph_service`: Affiliations, associations, team relationships
- `assets_service`: Avatar storage and representations
- `chat_service`: Support channel via `User.chatStatus`
- `jobs_service`: Organization jobs, applicant relationships
- `analysis_service`: UserSummary for AI-generated descriptions

---

## Common Workflows

### 1. Sign Up Flow
```
initiatePhoneAuth(phone, isSignup: true)
→ Cognito sends SMS code
→ confirmPhoneAuth(code) returns JWT
→ User created event fires
→ Triggers: welcome messages, interview requests (prompts_service)
```

### 2. Contact Join Notification
```
User A signs up with phone number
→ System hashes phone → searches UserDigests
→ Finds users who have this hash in contacts
→ Push notification sent to those users
```

### 3. Invite Resolution
```
createInvite(digests: [hashed_phone])
→ Invite stored with NEW status
→ Later: user registers with matching phone
→ System auto-resolves invite (status: ACCEPTED)
→ fromUser can now see resolved UserProfile
```

### 4. Onboarding Progression
```
Client connects: /onboarding_director/1/{user_id}
→ Director streams UI components
→ Client completes requirement
→ Client sends completion message
→ Persisted in UserOnboardingState
→ Next component streamed
```

### 5. Grant Token Application
```
User receives token (via URL parameter, deep link)
→ User logs in
→ applyGrantToken mutation called
→ System validates token, creates:
   - HiringTeamMember roles
   - Affiliations
   - Org memberships
→ User has new permissions immediately
```

---

## Core Types

### User

Cognito user record with authentication state. Contains sensitive data (email, phone).

```graphql
type User {
  id: Id!
  name: String!
  email: String!
  phone: String!
  profile: UserProfile
  onWaitlist: Boolean!
  needsOnboarding: Boolean!
  onboardingState: UserOnboardingState!
  onboardingDirectorUrl(directorApiVersion: Int!, debugMode: Boolean): String
  chatStatus: ChatStatus!
  deviceTokens: [UserDeviceToken!]!
}
```

### UserProfile

Public-facing user profile. Primary type for displaying user information.

```graphql
type UserProfile {
  userId: Id!
  fullName: String
  shortName: String
  headline: String
  pronouns: UserPronouns!
  slug: String
  publicProfileUrl: String
  avatarId: Id
  avatarUrl: String!
  avatarRepresentations(filter: RepresentationFilter): [Representation!]!
  createdAt: DateTime!
  trending: Boolean

  # Relationships
  affiliations(status: AffiliationStatus): [Affiliation!]!
  affiliation(otherUserId: Id): Affiliation  # Default or shared affiliation
  association(otherUserId: Id!): Association
  intentFlags: [UserIntentFlag!]!
  socialLinks: [UserSocialLink!]!
  promptTags(category: PromptTagCategory): [PromptTag!]!
  applicants: [Applicant!]!
}
```

### Organization

Company or organization entity.

```graphql
type Organization {
  id: Id!
  name: String!
  source: String!
  shortDescription: String
  homepageUrl: String
  employeeCount: String
  location: String
  logoId: Id
  logoUrl: String
  logoRepresentations(filter: RepresentationFilter): [Representation!]!
  users(limit: Int): [UserProfile!]!
  jobs(status: JobStatus!): [Job!]!
  shareUrl: String!
  organizationClaimUrl: String!
}
```

### Affiliation

Links a user to an organization with role/title.

```graphql
type Affiliation {
  id: Id!
  userId: Id!
  organizationId: Id!
  organization: Organization
  title: String
  startedAt: DateTime
  endedAt: DateTime
  status: AffiliationStatus!  # NEW, APPROVED, SUGGESTED, REJECTED
  visibility: AffiliationVisibility!
}
```

### Association

Relationship between two users (colleague, mentor, friend, etc.).

```graphql
type Association {
  id: Id!
  fromUserId: Id!
  toUserId: Id!
  relationshipType: RelationshipType!
  yearsCount: Int
  yearsAgo: Int
  affiliationId: Id
  affiliation: Affiliation
  status: AssociationStatus!
  fromUser: UserProfile
  toUser: UserProfile
}
```

---

## Supporting Types

```graphql
type UserIntentFlag {
  intentFlagId: Id!
  text: String!
  color: String!
  expiresAt: DateTime
}

type UserSocialLink {
  socialLinkId: Id!
  name: String!
  username: String!
  url: String!
  color: String
}

type Resume {
  id: Id!
  userId: Id!
  assetId: Id
  url: String
  applicants: [Applicant!]!
}

type Invite {
  id: Id!
  digests: [String!]!
  fromUserId: Id!
  fullName: String
  status: InviteStatus!  # NEW, ACCEPTED
}

type OrganizationMember {
  organizationId: Id!
  user: UserProfile!
  roles: [OrganizationMemberRole!]!  # ADMIN
}
```

---

## Queries

### Lookups

```graphql
# Single user by ID
user(id: Id!): User
userProfile(id: Id!): UserProfile

# Multiple users by ID
users(ids: [Id!]!): [User!]!
userProfiles(ids: [Id!]!): [UserProfile!]!

# Alternative lookups
userProfileBySlug(userId: Id, slug: String): UserProfile
userByPhoneNumber(phoneNumber: String!): User
currentUser: User

# Organizations
organization(id: Id!): Organization
organizations(ids: [Id!]!): [Organization!]!

# Affiliations
affiliation(id: Id!): Affiliation
affiliations(userId: Id!, status: AffiliationStatus): [Affiliation!]!

# Associations
association(id: Id!): Association
associations(userId: Id!): [Association!]!
associationForUsers(fromUserId: Id!, toUserId: Id!): Association!
```

### List & Search Queries

```graphql
# Paginated user profiles
listUserProfiles(
  limit: Int! = 100
  after: String
  before: String
  sort: UserProfileSort!
  filter: [UserProfileFilterCondition!]
): UserProfileConnection!

# Discovery
trendingUserProfiles(limit: Int): [UserProfile!]!
helpfulUserProfiles(limit: Int): [UserProfile!]!
suggestedUsers(userId: Id!): [UserProfile!]!
connectedUsers(userId: Id!): [UserProfile!]!

# Organizations
searchOrganizations(query: OrganizationSearch!): [Organization!]!
topOrganizations(limit: Int): [Organization!]!
suggestedOrganizations(userId: Id!): [Organization!]!

# Associations
searchAssociations(query: AssociationSearch!): [Association!]!
```

---

## Query Examples

### Get current user with onboarding state

```graphql
query {
  currentUser {
    id
    email
    phone
    needsOnboarding
    onboardingState {
      completedRequirements
      # ... specific requirement fields
    }
    onboardingDirectorUrl(directorApiVersion: 1)
    profile {
      fullName
      avatarUrl
    }
  }
}
```

### Get user profile with affiliations in context

```graphql
query {
  userProfile(id: "...") {
    fullName
    headline
    avatarUrl
    intentFlags { text color expiresAt }
    affiliations(status: APPROVED) {
      organization { name logoUrl }
      title
    }
    # Get affiliation relevant to viewing context
    affiliation(otherUserId: "viewer_id") {
      organization { name }
      title
    }
  }
}
```

### Search organizations

```graphql
query {
  searchOrganizations(query: { name: "Acme" }) {
    id
    name
    logoUrl
    employeeCount
    users(limit: 5) { fullName avatarUrl }
  }
}
```

### Get user with social connections

```graphql
query {
  userProfile(id: "...") {
    fullName
    socialLinks { name url username }
    promptTags { name category }
    applicants { job { title organization { name } } status }
  }
  associations(userId: "...") {
    toUser { fullName }
    relationshipType
    affiliation { organization { name } }
  }
}
```

---

## Common Patterns

**Fetching user with affiliation context**: Use `affiliation(otherUserId:)` on UserProfile to get the most relevant affiliation when displaying in context of another user.

**Profile vs User**: Use `UserProfile` for display purposes (public data). Use `User` only when you need auth state, onboarding, or contact info.

**Pagination**: `listUserProfiles` returns `UserProfileConnection` with `nodes` and `pageInfo`. Filter on `FULL_NAME`, `EMAIL`, or `PHONE` fields.

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `User.profile` | - | Links auth record to public profile |
| `UserProfile.affiliations` | social_graph | User's org memberships |
| `UserProfile.applicants` | jobs | User's job applications |
| `UserProfile.promptTags` | prompts | Skills/tags from interviews |
| `UserProfile.avatarRepresentations` | assets | Avatar image variants |
| `User.chatStatus` | chat | Support channel state |
| `Organization.jobs` | jobs | Jobs posted by org |
| On signup | prompts | Interview requests created |
| On signup | social_graph | Contact discovery via digests |
