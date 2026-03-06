# Config Service Reference

## Overview

The config_service surfaces **environment-specific configuration values** - particularly mappings between behavior and IDs/tags that differ across dev/staging/production. It supports feature flagging, A/B testing with deterministic user bucketing, and gradual rollouts. Configs are namespaced by domain+key and can be either static values or bucketed experiments.

---

## Key Concepts

### ConfigEntry and ConfigEntryValue

A ConfigEntry is a configuration item uniquely identified by its **domain + key** combination (unique constraint enforced). The value can be one of two types:

| Mode | Type | Use Case |
|------|------|----------|
| **Value (static)** | `valueJson` | Feature toggles, defaults, UI copy - all users see the same value |
| **BucketedValue (A/B)** | `bucketed` | A/B tests, gradual rollouts - users split by ratio |

### Bucketing Algorithm

For A/B tests, user assignment is **deterministic**:

1. CRC32 hash of `(user_id + campaign)`
2. Compare: `hash < (ratio * u32::MAX)`
3. Result: Bucket A (true) or Bucket B (false)

This means the same user always sees the same variant - as long as the campaign name and ratio remain unchanged.

### Campaigns (Critical for A/B)

The **campaign** field serves as the bucketing salt. Key properties:

- **Immutable in effect**: Changing a campaign name redistributes ALL users to new buckets
- **Analytics integration**: Campaign name is sent to Segment for tracking
- **Coordinated experiments**: Two BucketedValues with the **same campaign + same ratio** will assign users to the **same bucket** for both configs

This coordination is powerful - you can ensure a user in bucket A for "new checkout flow" is also in bucket A for "new checkout copy."

### Serial (Cache Invalidation)

The `serial` field in ConfigValuesResult is a timestamp of the latest config update. Clients can use this to detect when their cached configs are stale.

### Caching Behavior

- **Server-side**: 60-second in-memory cache refresh
- **Per-instance**: Each service instance has its own cache (horizontal scaling implication)
- **Client-side**: Use `serial` field to detect changes

---

## Important Relationships

```
ConfigEntry (domain + key)
  └─ ConfigEntryValue
       ├─ Value (static JSON)
       └─ BucketedValue
            ├─ ratio (0-1)
            ├─ campaign (bucketing salt)
            ├─ valueAJson
            └─ valueBJson

Client Request
  └─ ConfigClientInfo (userId)
       └─ Resolved ConfigValue (server computes bucket)
```

**Cross-service dependencies:**
- `users_service`: User IDs for bucketing
- `Segment`: Campaign names sent for analytics tracking

---

## Common Workflows

### 1. Feature Flag (Static)
```
createConfigEntry(domain: "features", key: "dark_mode", valueJson: "true")
→ All users see the same value
→ Change value to roll out/back instantly
```

### 2. A/B Test
```
createConfigEntry(
  domain: "experiments",
  key: "checkout_v2",
  bucketed: {
    ratio: 0.5,
    campaign: "checkout_redesign_2024",
    valueAJson: "\"control\"",
    valueBJson: "\"variant\""
  }
)
→ 50% of users see "control", 50% see "variant"
→ Same user always sees same variant
```

### 3. Gradual Rollout
```
Start: ratio: 0.1 → 10% of users get new feature
Week 2: ratio: 0.25 → 25% of users
Week 3: ratio: 0.5 → 50% of users
Final: Convert to static Value when confident
```

Note: Changing ratio DOES redistribute users (unlike changing campaign, which is a full reshuffle).

### 4. Coordinated Experiment
```
Config 1: checkout_flow (campaign: "checkout_v2", ratio: 0.3)
Config 2: checkout_copy (campaign: "checkout_v2", ratio: 0.3)
→ Users in bucket A for flow are ALSO in bucket A for copy
→ Enables testing multiple related changes together
```

---

## Access Control (Three Tiers)

| Level | Who | Capabilities |
|-------|-----|--------------|
| **Admin** | Internal team | Full CRUD on all configs |
| **Service-to-service** | Backend services | Fetch configs on behalf of users (requires `configs/config` scope) |
| **User self-serve** | End users | Can only fetch configs for themselves |

---

## Core Types

### ConfigEntry

Admin-facing stored configuration entry.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Id!` | Unique identifier |
| `domain` | `String!` | Namespace (e.g., "features", "experiments") |
| `key` | `String!` | Config key within domain |
| `entryType` | `ConfigEntryType!` | VALUE or BUCKETED |
| `value` | `ConfigEntryValue!` | The actual config value |

### ConfigEntryValue (Union)

| Variant | Fields | Description |
|---------|--------|-------------|
| `valueJson` | `String!` | Static JSON-encoded value |
| `bucketed` | `BucketedConfigEntryValue!` | A/B test configuration |

### BucketedConfigEntryValue

| Field | Type | Description |
|-------|------|-------------|
| `ratio` | `Float!` | 0-1, fraction of users in bucket A |
| `campaign` | `String!` | Bucketing salt (also sent to Segment) |
| `valueAJson` | `String!` | Value for bucket A users |
| `valueBJson` | `String!` | Value for bucket B users |

### ConfigValue

Client-facing resolved value (after bucketing computed server-side).

| Field | Type | Description |
|-------|------|-------------|
| `domain` | `String!` | Config namespace |
| `key` | `String!` | Config key |
| `valueJson` | `String!` | Resolved value for this user |
| `variesOn` | `ConfigVariesOn` | What caused variation (USER if bucketed) |

### ConfigValuesResult

| Field | Type | Description |
|-------|------|-------------|
| `serial` | `Int!` | Timestamp for cache invalidation |
| `values` | `[ConfigValue!]!` | Resolved config values |

### Input Types

| Type | Fields | Description |
|------|--------|-------------|
| `ConfigKey` | `domain`, `key` | Lookup key for fetching configs |
| `ConfigClientInfo` | `userId` | User context for bucket assignment |

---

## Queries

```graphql
# Client API - fetch resolved config values
getConfigValues(keys: [ConfigKey!]!, clientInfo: ConfigClientInfo!): ConfigValuesResult!

# Admin API - list all config entries
listConfigEntries: [ConfigEntry!]!
```

---

## Mutations (Admin)

```graphql
# Create new config entry
createConfigEntry(payload: ConfigEntryCreate!): ConfigEntry!

# Update existing entry (lookup by domain/key)
updateConfigEntry(payload: ConfigEntryUpdate!): ConfigEntry!

# Delete by ID
deleteConfigEntry(id: Id!): Boolean!
```

---

## Query Examples

### Fetch config values for a user

```graphql
query {
  getConfigValues(
    keys: [
      { domain: "features", key: "dark_mode" }
      { domain: "experiments", key: "checkout_v2" }
    ]
    clientInfo: { userId: "user_123" }
  ) {
    serial
    values {
      domain
      key
      valueJson
      variesOn
    }
  }
}
```

### List all configs (admin)

```graphql
query {
  listConfigEntries {
    id
    domain
    key
    entryType
    value {
      ... on ConfigEntryValueJson {
        valueJson
      }
      ... on BucketedConfigEntryValue {
        ratio
        campaign
        valueAJson
        valueBJson
      }
    }
  }
}
```

### Create a feature flag

```graphql
mutation {
  createConfigEntry(payload: {
    domain: "features"
    key: "new_dashboard"
    valueJson: "false"
  }) {
    id
    domain
    key
  }
}
```

### Create an A/B test

```graphql
mutation {
  createConfigEntry(payload: {
    domain: "experiments"
    key: "pricing_page_v2"
    bucketed: {
      ratio: 0.2
      campaign: "pricing_redesign_q1"
      valueAJson: "\"control\""
      valueBJson: "\"variant\""
    }
  }) {
    id
    domain
    key
  }
}
```

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `ConfigClientInfo.userId` | users | User for bucket assignment |
| `campaign` field | Segment (external) | Analytics tracking for experiments |
