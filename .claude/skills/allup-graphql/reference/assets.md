# Asset Service Reference

## Overview

The asset_service is a **multi-format digital asset management and distribution platform** - not just file storage. It handles the complete asset lifecycle: secure uploads with token-based authorization, multi-representation storage (original + transcoded variants), generation-based versioning, and on-the-fly image processing. The architecture separates metadata operations (GraphQL) from blob storage (REST), enabling content negotiation and dynamic image composition for social sharing.

---

## Key Concepts

### Asset vs Representation vs Generation

These three concepts form a hierarchy:

| Concept | What It Is | Mutability |
|---------|------------|------------|
| **Asset** | Logical container grouping all representations. Has metadata HashMap, optional `user_id` owner, `private` flag. | Mutable (metadata, current generation) |
| **Representation** | Concrete file in specific format/resolution. Examples: original video, transcoded MP4, WebM, JPEG thumbnail. | Immutable once created |
| **Generation** | Version collection. Multiple representations share a `generation_id`. | Immutable |

An Asset can designate a `current_generation_id` - this enables workflows like "old avatar" vs "new avatar" where you keep history but surface the latest.

### Two-Phase Upload Architecture

Uploads require a token before the actual file transfer:

```
1. createUploadToken() → validates permissions, returns token + upload URL
2. POST /upload/{asset_id} with Authorization: Bearer <token>
3. On success: Asset + Representation created, event published
```

**Upload tokens** (12-hour TTL) authorize uploads with constraints:
- Asset ID correlation
- Content-type whitelist (e.g., only `image/jpeg`, `image/png`)
- File size limits
- Record correlation (e.g., "this upload is for User X's avatar")

### Access Tokens for Private Assets

**Access tokens** (2-day TTL) grant time-limited read access to private assets. Use `createAssetAccessToken` to generate, then append to URL or use in authorization header.

### Generation Filtering

When querying representations, generation filtering controls which versions you see:

| Filter | Behavior |
|--------|----------|
| `DEFAULT` | Current generation if set, otherwise all |
| `CURRENT` | Only current generation (empty if none set) |
| `NONE` | All generations |
| `ID` | Specific generation by ID |

This matters for assets with history - a user's avatar might have 5 generations, but you usually want the current one.

### Dynamic Image Composition

The service generates composite images on-the-fly for social sharing. Examples:
- Job preview cards with company logo + job title
- User profile cards with avatar + name

Use `imageProcessing` parameters on `Representation.url` to apply transforms: resize, crop, mask (circle/squircle), blur, color overlays.

---

## Important Relationships

```
Asset (logical container)
  ├─ metadata (JSONObject)
  ├─ user_id (owner)
  ├─ private (access control)
  ├─ current_generation_id (version pointer)
  └─ Representations
       ├─ generation_id (links to a generation)
       ├─ contentType (MIME)
       ├─ contentLength (bytes)
       ├─ meta (format-specific data)
       └─ storage references
```

**Cross-service dependencies:**
- `users_service`: Avatar assets, profile photos (extends UserProfile)
- `prompts_service`: Video response assets (extends PromptResponse)
- `jobs_service`: Preview images, share cards (extends Job)
- Lambda functions: Video transcoding pipelines

**Federation extends:**
- `UserProfile.avatar`, `UserProfile.avatarRepresentations`
- `Organization.logo`, `Organization.logoRepresentations`
- `PromptResponse.asset`, `PromptResponse.assetRepresentations`
- `Job.previewImageAssetId`, `Job.shareImageAssetId`

---

## Common Workflows

### 1. Avatar Upload

```
createUploadToken(recordType: "User", recordId: userId, contentTypes: ["image/*"])
→ POST /upload/{asset_id} with image file
→ Asset + Representation created
→ Event published to SNS
→ users_service subscribes, links avatar to UserProfile
```

### 2. Video Processing Pipeline

```
Upload original video via upload token
→ Representation created (video/quicktime or similar)
→ Lambda triggered by event
→ Transcodes to MP4, WebM representations
→ Thumbnail representations extracted
→ All linked to same generation_id
```

### 3. Social Card Generation

```graphql
query {
  asset(id: "job-preview-asset") {
    representations(filter: { mimeType: "image/*" }) {
      url(imageProcessing: {
        widthPx: 1200
        heightPx: 630
        outputFormat: JPEG
      })
    }
  }
}
```

The compositor generates the card on-demand based on template + parameters.

### 4. Version Update (New Generation)

```
1. createUploadToken with existing asset_id
2. Upload new file → creates new Representation with new generation_id
3. updateAsset(currentGenerationId: newGenerationId) to switch active version
4. Old representations remain accessible via generation filtering
```

---

## Core Types

### Asset

A logical asset container that groups multiple representations (formats/versions).

```graphql
type Asset {
  id: Id!
  currentGenerationId: Id          # Active generation for version control
  createdAt: DateTime!
  updatedAt: DateTime!
  meta: JSONObject!                # Arbitrary metadata
  representations(filter: RepresentationFilter): [Representation!]!
}
```

### Representation

A specific format or version of an asset (e.g., original video, transcoded MP4, thumbnail).

```graphql
type Representation {
  id: Id!
  assetId: Id!
  createdAt: DateTime!
  updatedAt: DateTime!
  contentType: String!             # MIME type
  contentLength: Int!              # Size in bytes
  meta: JSONObject!
  url(imageProcessing: ImageProcessing): String!   # Public URL with optional transforms
  storageUrl: String!              # Direct storage URL
}
```

### UploadToken

Token for uploading to the blob server. Include as `Authorization: Bearer <token>` header.

```graphql
type UploadToken {
  token: String!
  uploadUrl: String!               # POST multipart to this URL
}
```

### AssetReadToken

Token for accessing private assets.

```graphql
type AssetReadToken {
  token: String!
}
```

---

## Queries

```graphql
# Single lookup
asset(id: Id!): Asset
representation(id: Id!): Representation

# Batch lookup
assets(ids: [Id!]!): [Asset!]!

# Search by owner
searchAssets(payload: AssetSearch!): [Asset!]!
```

**AssetSearch input:**
```graphql
input AssetSearch {
  userId: Id    # Filter by owner
}
```

---

## Mutations

```graphql
# Metadata updates
updateAsset(payload: AssetUpdate!): Asset!
deleteAsset(id: Id!): Id!

# Representations
createRepresentation(payload: RepresentationCreate!): Representation!
updateRepresentation(payload: RepresentationUpdate!): Representation!
deleteRepresentation(id: Id!): Id!

# Tokens
createUploadToken(payload: UploadTokenRequest!): UploadToken!
createAssetAccessToken(payload: AssetReadTokenRequest!): AssetReadToken!
```

---

## Query Examples

### Get upload token for a user's avatar

```graphql
mutation {
  createUploadToken(payload: {
    recordType: "User"
    recordId: "user-id"
    contentTypes: ["image/jpeg", "image/png"]
    maxSize: 5242880
  }) {
    token
    uploadUrl
  }
}
```

### Fetch asset with specific representation

```graphql
query {
  asset(id: "asset-id") {
    id
    currentGenerationId
    representations(filter: { mimeType: "video/mp4" }) {
      id
      url
      contentLength
    }
  }
}
```

### Get thumbnail with processing

```graphql
query {
  asset(id: "asset-id") {
    representations(filter: { mimeType: "image/*" }) {
      url(imageProcessing: {
        widthPx: 200
        mask: CIRCLE
        outputFormat: JPEG
      })
    }
  }
}
```

### Filter by generation

```graphql
query {
  asset(id: "asset-id") {
    # Only current generation
    representations(filter: { generation: CURRENT }) {
      id
      contentType
    }
  }
}
```

---

## Filtering Representations

Use `RepresentationFilter` to select specific formats:

```graphql
input RepresentationFilter {
  mimeType: String        # HTTP Accept-style matching (e.g., "video/*")
  meta: JSONObject        # Exact match on metadata keys
  generation: GenerationFilter
}
```

---

## Image Processing

Request on-the-fly image transforms via the `imageProcessing` argument on `Representation.url`:

```graphql
input ImageProcessing {
  widthPx: Int
  heightPx: Int
  blur: Int
  mask: ImageMaskType         # CIRCLE, SQUIRCLE, ROUNDED_RECT
  foregroundColor: String
  backgroundColor: String
  outputFormat: ImageOutputFormat   # JPEG, PNG
}
```

### URL Query Parameters

You can also construct image processing URLs directly using query parameters:

| Parameter | Description | Format/Values |
|-----------|-------------|---------------|
| `isx` | Image size | `WIDTHxHEIGHT` (e.g., `200x200`, `200x` for width only, `x200` for height only) |
| `ibx` | Blur radius | Integer (1-4096) |
| `imx` | Mask type | `c` (circle), `s` (squircle), `r` (rounded rect) |
| `ifc` | Foreground color | Hex color (e.g., `ff0000`) - used when rasterizing SVG |
| `ibc` | Background color | Hex color - used with masks or transparent images |
| `ofx` | Output format | `jpg`, `jpeg`, or `png` |

**Example URLs:**
```
# Resize to 200x200 with circle mask
/a/{asset_id}?isx=200x200&imx=c

# Resize width to 100px, preserve aspect ratio, output as PNG
/a/{asset_id}?isx=100x&ofx=png

# Circle mask with red background
/a/{asset_id}?imx=c&ibc=ff0000

# Blur with radius 10
/a/{asset_id}?ibx=10
```

---

## Cross-Service Relationships

| Type | Field | Description |
|------|-------|-------------|
| `UserProfile` | `avatar`, `avatarRepresentations` | User profile photo |
| `Organization` | `logo`, `logoRepresentations` | Organization logo |
| `PromptResponse` | `asset`, `assetRepresentations` | Video response |
| `Resume` | `assetId`, `uploadToken` | PDF resume |
| `Job` | `previewImageAssetId`, `shareImageAssetId` | Social cards |
| `Prompt` | `videoAssetId` | Video prompt |

---

## Blob Server Endpoints

The REST API runs separately from GraphQL (port 8001 locally, port 8000 for GraphQL):

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/upload/{asset_id}` | POST | Upload with token (multipart/form-data) |
| `/a/{asset_id}` | GET | Download, content negotiation selects representation |
| `/o/{object_id}` | GET | Download specific representation by ID |
| `/c/{composite_id}` | GET | Generated composite images |
| `/avatar/{avatar_id}` | GET | Avatar images (convenience endpoint) |

Supports HTTP range requests (206 Partial Content) for video streaming.
