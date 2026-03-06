# Scripting for Large Result Sets

When query results are too large to process in the agent's context window (hundreds of items needing aggregation or transformation), fall back to scripts.

Save one-time scripts to `/tmp` to avoid cluttering the project.

---

## Python (recommended)

Use the template at `reference/python-template.py.txt`. It handles auth, pagination, and typed results.

```bash
cp .claude/skills/allup-graphql/reference/python-template.py.txt /tmp/my-query.py
# Edit QUERY and process_results()
uv run --with requests --with pydantic /tmp/my-query.py staging
```

### Quick example

```python
# Modify QUERY and process_results() in the template
QUERY = """
query($after: String) {
    listJobs(limit: 100, after: $after) {
        nodes { id title status }
        pageInfo { hasNextPage endCursor }
    }
}
"""

def process_results():
    for job in paginate(QUERY, "listJobs"):
        print(f"{job['title']}: {job['status']}")
```

---

## Bash / curl

```bash
#!/bin/bash
set -e
ENV=${1:-staging}
TOKEN=$(wrk -e $ENV login admin --print)

QUERY='
query {
  listApplicants(limit: 100) {
    nodes { id status user { fullName } }
  }
}
'

curl -s -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{\"query\": $(echo "$QUERY" | jq -Rs .)}" \
  "https://api.${ENV}.wrkvbs.io/3/graphql" | jq '.data.listApplicants.nodes'
```

### Processing with jq

```bash
# Count by status
... | jq '.data.listApplicants.nodes | group_by(.status) | map({status: .[0].status, count: length})'

# Extract specific fields
... | jq '.data.listApplicants.nodes[] | {id, name: .user.fullName}'

# Filter results
... | jq '.data.listApplicants.nodes | map(select(.status == "NEW"))'
```

---

## Authentication

The `wrk` CLI provides auth tokens for scripts:

```bash
# Get token
TOKEN=$(wrk -e staging login admin --print)

# If token is expired, re-authenticate interactively first
wrk -e staging login admin
```

## Environment URLs

| Environment | URL                                       |
| ----------- | ----------------------------------------- |
| dev         | `https://api.dev.wrkvbs.io/3/graphql`     |
| staging     | `https://api.staging.wrkvbs.io/3/graphql` |
| production  | `https://api.wrkvbs.io/3/graphql`         |

## Requirements

- Python 3.9+ and `uv` (recommended) or `pip`
- `wrk` CLI installed and on PATH
- `jq` for bash-based processing
