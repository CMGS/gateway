# Governance

Per-request controls run as pipeline stages before and after the engine call.
All limits are enforced in-process by default; set `storage.redis_url` to share
them across replicas (see [Deployment](deployment.md)).

## Access keys

Each `access_keys` entry is one credential with its own governance:

```yaml
access_keys:
  - ak: ak-demo-123
    product: demo            # product group (for product-level QPM)
    qps: 100                 # per-key request rate
    daily_token_quota: 1000000
    tokens_per_minute: 600   # optional TPM window
```

## Limits

| Limit | Scope | Config |
|-------|-------|--------|
| QPS | per access key | `access_keys[].qps` |
| Daily tokens | per access key | `access_keys[].daily_token_quota` (reset by a background task) |
| TPM | per access key | `access_keys[].tokens_per_minute` |
| QPM | per model | `models[].qpm` |
| QPM | per product | `products[].qpm` |

Exceeding any limit returns `429`. QPS uses a smooth GCRA limiter in-process (a
fixed 1s window in Redis); the token/window counters are fixed windows. When
Redis is configured and unreachable, limits fail open (requests pass) and a
warning is logged — a persistent outage never silently wedges the gateway.

## Request cache

A model with `cache_ttl_seconds` set caches non-streaming responses for that
TTL (bounded, moka-backed). A cache hit short-circuits account selection, the
engine call, and billing.

```yaml
models:
  - name: cached-mini
    protocol: openai-chat
    cache_ttl_seconds: 60
```

## Content safety

`security.dlp_redact` redacts emails and phone numbers from inbound and
outbound content; `security.blocklist` rejects requests containing listed terms
with a `content_filter` finish (not billed).

```yaml
security:
  dlp_redact: true
  blocklist: ["badword"]
```
