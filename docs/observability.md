# Observability

## Metrics

`GET /metrics` renders the Prometheus registry.

| Metric | Type | Labels |
|--------|------|--------|
| `gateway_requests_total` | counter | `route`, `status` |
| `gateway_request_duration_seconds` | histogram | `route` |
| `gateway_node_duration_seconds` | histogram | `node` (pipeline stage) |
| `gateway_tokens_total` | counter | `kind` (prompt/completion) |
| `gateway_cache_hits_total` | counter | — |
| `gateway_ledger_write_failures_total` | counter | — |
| `gateway_upstream_connect_retries_total` | counter | `account` |

`gateway_requests_total` is recorded by router middleware, so every response —
including error statuses and the realtime WebSocket upgrade — is counted, which
makes error-rate dashboards possible. All labels are bounded (route templates,
status codes, protocol/stage names) — no per-key or per-model cardinality.

## Access log

One structured line per request goes to stdout (via `tracing`; control level
with `RUST_LOG`), carrying `ak`, `product`, `model`, `protocol`, `account`,
`status`, `prompt_tokens`, `completion_tokens`, `total_tokens`, and
`latency_ms`.

## Billing ledger

`GET /internal/ledger?limit=N` returns the most recent `N` billing records
(newest first); `count` is always the true total, independent of the page size.
Records persist when a SQLite store is configured and can be capped with
`storage.ledger_max_rows`. Each record has the access key, product, model,
protocol, account, token counts, cost, and the PTU-spillover flag.
