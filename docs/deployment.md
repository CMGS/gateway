# Deployment

## Binary

Install a tagged release (Linux/macOS, x86_64/arm64) with the generated script:

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/CMGS/gateway/releases/latest/download/ap-server-installer.sh | sh
```

Or build from source:

```bash
make release            # target/release/ap, built --locked
AP_GATEWAY_CONF=/etc/gateway.yaml ./target/release/ap
```

### Environment

| Variable | Effect |
|----------|--------|
| `AP_GATEWAY_CONF` | config file path; unset uses the embedded demo config |
| `AP_HOST` | override `listen.host` (containers set `0.0.0.0`) |
| `AP_PORT` | override `listen.port` |
| `AP_TRANSPORT` | `mock` (zero egress) / `http` (no mock) / unset (auto-route) |
| `RUST_LOG` | log level, e.g. `info`, `ap_views=debug` |
| provider key vars | named by each account's `api_key_env` |

The process drains on SIGINT/SIGTERM (graceful shutdown of in-flight requests).

## Docker

```bash
docker build -t gateway .
docker run -p 8080:8080 gateway            # embedded demo config
docker run -p 8080:8080 \
  -v $PWD/conf/gateway.yaml:/etc/gateway.yaml \
  -e AP_GATEWAY_CONF=/etc/gateway.yaml \
  -e OPENAI_API_KEY=sk-... \
  gateway
```

The image is a slim non-root runtime, binds `0.0.0.0`, and has a `/health`
HEALTHCHECK. Tagged `v*` pushes publish a multi-arch image to
`ghcr.io/cmgs/gateway`.

## Multi-replica

State that must be shared across replicas has a backend:

```yaml
storage:
  sqlite_path: /var/lib/ap/store.db    # durable ledger/files/batches
  ledger_max_rows: 1000000             # prune oldest billing rows past the cap
  redis_url: "redis://redis:6379"      # shared rate limits + quotas
```

- **Durable records** (ledger, files, batches): SQLite when `sqlite_path` is
  set (survives restarts), otherwise in-memory. Orphaned `running` batch jobs
  from a dead process are swept to `failed` on startup.
- **Rate limits & quotas**: shared in Redis when `redis_url` is set (keys
  namespaced under `ap:`, windows self-expire), otherwise in-process. Without
  Redis, each replica limits independently.
