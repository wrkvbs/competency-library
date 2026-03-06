# Media Pipeline Service Reference

## Overview

The media_pipeline_service is a **video processing orchestration engine** that transforms raw video into richly annotated, searchable media. It coordinates AWS Step Functions workflows to run parallel Lambda handlers (transcoding, transcription, AI annotation, face detection) and stores results as typed Layers on Assets. The layers - time-aligned metadata like transcripts, tags, and AI-generated highlights - are the real value; raw video is less useful than the extracted intelligence.

---

## Key Concepts

### Workflow

A Step Functions state machine definition that describes a video processing pipeline. Workflows are **templates** - reusable logic that can be instantiated multiple times for different assets.

| Field | Purpose |
|-------|---------|
| `stateMachineResource` | AWS ARN of the Step Functions state machine |
| `stateMachineDefinition` | JSON definition of the state machine logic |
| `version` | Version identifier for the workflow |
| `recordType` | Type of record this workflow processes |

### Generation

A UUID that tracks one end-to-end processing run. Generations solve the versioning problem: if a video is re-processed, old annotations don't mix with new ones. Every representation created during processing gets the generation ID in its metadata, and assets track their `currentGenerationId`.

### Layer

Time-aligned metadata extracted during processing. Layers are **the real value** of the pipeline - they make raw video searchable and usable. Each layer contains segments with `in_point`/`out_point` timing plus metadata content.

| Layer Type | What It Contains |
|------------|------------------|
| `transcription/words` | Word-level transcript with precise timing |
| `transcription/sentences` | Sentence-grouped transcript |
| `annotations` | AI-generated highlights and removal marks |
| `tags` | Semantic tags extracted from content |
| `face_detection` | Recognized faces with timing |

Layers are stored as special Representations with content type `application/vnd.wrkvbs.layer.v1+json` and `layer_type` in metadata.

### Representation

A specific format or version of an asset. Representations include both media formats (video/mp4, audio/m4a, image/jpeg) and extracted data (layers). Key metadata fields:

| Field | Purpose |
|-------|---------|
| `contentType` | MIME type (e.g., "video/mp4", "application/vnd.wrkvbs.layer.v1+json") |
| `layer_type` | For layer representations, the type (e.g., "transcription/words") |
| `generation_id` | Which processing run created this representation |

### AI Enhancement Pipeline

The pipeline includes multiple AI-powered handlers:

| Handler | What It Does |
|---------|--------------|
| `transcribe_video` | AWS Transcribe for speech-to-text |
| `recognize_video` | AWS Rekognition for face detection |
| `auto_tag_video` | Reads transcript sentences, creates semantic tags |
| `annotate_video` | Fine-tuned GPT-3.5 annotates with highlights/removals |

---

## Important Relationships

```
Asset (video upload)
  ├─ Representations (formats)
  │    ├─ video/mp4 (transcoded versions)
  │    ├─ image/jpeg (thumbnails)
  │    └─ application/vnd.wrkvbs.layer.v1+json (layers)
  │         ├─ transcription/words
  │         ├─ transcription/sentences
  │         ├─ annotations
  │         ├─ tags
  │         └─ face_detection
  └─ currentGenerationId → tracks which processing run is current

Workflow (template) → starts execution for Asset
```

**Cross-service dependencies:**
- `prompts_service`: Video responses trigger processing; receives `VideoPubsubMessage` on completion to mark response COMPLETE
- `assets_service`: Stores all representations (shared Asset/Representation types)
- `users_service`: User avatars processed through the pipeline

---

## Common Workflows

### 1. Prompt Response Video Processing
```
User uploads video response (prompts_service)
→ SQS message triggers handle_asset_uploads
→ Workflow starts with asset ID
→ Parallel execution:
   - transcode_video (multiple formats)
   - transcribe_video (speech-to-text)
   - recognize_video (face detection)
→ Sequential AI enhancement:
   - auto_tag_video reads transcript → creates tags
   - annotate_video reads transcript → highlights/removals
→ All results stored as layer representations
→ VideoPubsubMessage published
→ prompts_service marks response COMPLETE
```

### 2. Reprocess with Trimming
```
New workflow started for existing asset
→ Reads existing annotation layer
→ Finds removal marks (sections to cut)
→ Creates trimmed MP4 based on removal marks
→ New representations stored with new generation ID
```

### 3. Query Layers for Playback
```graphql
# Get transcript for video player overlay
query {
  asset(id: "...") {
    representations(filter: {
      mimeType: "application/vnd.wrkvbs.layer.v1+json"
      meta: { layer_type: "transcription/words" }
    }) {
      url
    }
  }
}
```

---

## Core Types

### Asset

Core entity representing an uploaded file (video, image, etc).

```graphql
type Asset {
    id: Id!
    currentGenerationId: Id
    createdAt: DateTime!
    updatedAt: DateTime!
    meta: JSONObject!
    representations(filter: RepresentationFilter): [Representation!]!
}
```

Assets have multiple **representations** - different formats/versions of the same content (e.g., original upload, transcoded versions, thumbnails, layers).

### Representation

A specific format/version of an asset.

```graphql
type Representation {
    id: Id!
    assetId: Id!
    createdAt: DateTime!
    updatedAt: DateTime!
    contentType: String!    # MIME type (e.g., "video/mp4", "image/jpeg")
    contentLength: Int!
    meta: JSONObject!
    url(imageProcessing: ImageProcessing): String!
    storageUrl: String!
}
```

### Workflow

State machine definition for processing pipelines.

```graphql
type Workflow {
    id: Id!
    stateMachineDefinition: String!
    stateMachineResource: String!
    version: String!
    recordType: String!
}
```

### MediaTime

Scalar representing time positions in media (used for transcript segments, annotations).

---

## Queries

```graphql
# Look up single asset
asset(id: Id!): Asset

# Look up multiple assets
assets(ids: [Id!]!): [Asset!]!

# Search assets by owner
searchAssets(payload: AssetSearch!): [Asset!]!

# Look up representation
representation(id: Id!): Representation

# Look up workflow
workflow(id: Id!): Workflow
```

---

## Query Examples

### Get video playback URL

```graphql
query {
    asset(id: "...") {
        representations(filter: { mimeType: "video/mp4" }) {
            url
            contentType
            meta
        }
    }
}
```

### Get transcript layer

```graphql
query {
    asset(id: "...") {
        representations(filter: {
            mimeType: "application/vnd.wrkvbs.layer.v1+json"
            meta: { layer_type: "transcription/words" }
        }) {
            url
            meta
        }
    }
}
```

### Get all layers for current generation

```graphql
query {
    asset(id: "...") {
        currentGenerationId
        representations(filter: {
            mimeType: "application/vnd.wrkvbs.layer.v1+json"
            generation: { behavior: CURRENT }
        }) {
            meta  # Contains layer_type
            url
        }
    }
}
```

### Get sized image/thumbnail

```graphql
query {
    asset(id: "...") {
        representations(filter: { mimeType: "image/*" }) {
            url(imageProcessing: { widthPx: 200, heightPx: 200 })
        }
    }
}
```

### Starting a processing workflow

```graphql
mutation {
    startWorkflow(payload: {
        assetId: "..."
        representations: [
            { id: "...", contentType: "video/mp4" }
        ]
    }) {
        id
    }
}
```

---

## Mutations

```graphql
# Asset operations
updateAsset(payload: AssetUpdate!): Asset!
deleteAsset(id: Id!): Id!

# Representation operations
createRepresentation(payload: RepresentationCreate!): Representation!
updateRepresentation(payload: RepresentationUpdate!): Representation!
deleteRepresentation(id: Id!): Id!

# Upload tokens
createUploadToken(payload: UploadTokenRequest!): UploadToken!
createAssetAccessToken(payload: AssetReadTokenRequest!): AssetReadToken!

# Workflow operations
createWorkflow(payload: WorkflowCreate!): Workflow!
startWorkflow(payload: WorkflowStart!): Workflow!
```

---

## Filtering Patterns

### RepresentationFilter

Filter representations by format, generation, or metadata:

```graphql
input RepresentationFilter {
    mimeType: String              # HTTP Accept-style (e.g., "video/*")
    meta: JSONObject              # Exact match on key/value pairs
    generation: GenerationFilter  # Which generation to return
}
```

### GenerationFilter

Control which version of representations to return:

```graphql
enum GenerationFilterBehavior {
    DEFAULT   # Current generation if exists, else all
    CURRENT   # Only current generation
    NONE      # All generations
    ID        # Specific generation by ID
}
```

### ImageProcessing

Transform images at request time:

```graphql
input ImageProcessing {
    widthPx: Int
    heightPx: Int
    blur: Int
    mask: ImageMaskType
    foregroundColor: String
    backgroundColor: String
    outputFormat: ImageOutputFormat
}
```

---

## Cross-Service Relationships

| This Service | Related Service | Relationship |
|--------------|-----------------|--------------|
| `Asset` | prompts | `PromptResponse.videoAssetId` links to video assets |
| `Asset` | prompts | Response status: NEW → PENDING → COMPLETE as pipeline processes |
| `Asset` | users | `UserProfile.avatarRepresentations` stored as assets |
| `Asset` | jobs | `Job.previewImageAssetId`, `Job.shareImageAssetId` |
| Various | all | Icons and logos via `iconAssetId` fields |
