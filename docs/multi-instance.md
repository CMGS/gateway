# Running a fleet

Multiple `gw` instances behind a load balancer (e.g. nginx). What must be
shared, what stays local, and what the LB needs to do.

## What each instance holds

| State | Backend | Shared across the fleet? |
|-------|---------|--------------------------|
| Rate limits / quotas (`Governance`) | Redis (`storage.redis_url`) | ✅ yes, when Redis is set |
| Billing ledger / files / batches (`Store`) | SQLite (`storage.sqlite_path`) | ❌ SQLite is local; needs a networked backend |
| Account health / cooldown | in-process | ❌ per-instance |
| Request cache | in-process (moka) | ❌ per-instance (a miss just recomputes) |
| Config (keys, models, providers) | local YAML | ❌ loaded at boot per instance |

**For a correct fleet you need:**

1. **Redis** for `Governance` — so per-key QPS/quota/TPM hold across all
   instances instead of each counting on its own.
2. A **shared config** so every instance authenticates the same keys and
   routes the same models (see [Dynamic config](#dynamic-config) below).
3. A **shared `Store`** if you rely on the ledger/files/batches being visible
   from any instance — SQLite is per-node. A networked Store (Postgres) is planned; until then, either pin ledger reads to one node or accept
   per-node ledgers aggregated at scrape time.

## Load balancer

Use the sample [`deploy/nginx.conf`](../deploy/nginx.conf). The essentials:

- **SSE**: `proxy_buffering off` and a long `proxy_read_timeout` — otherwise
  nginx buffers the whole stream or cuts long generations.
- **WebSocket** (`/v1/realtime`): the `Upgrade`/`Connection: upgrade` headers
  and a long read timeout. A WS connection pins to one instance for its life,
  so no session store is needed.
- **Health**: point the upstream health check at `/health`.
- **Metrics**: scrape `/metrics` on each instance directly (Prometheus service
  discovery), not through the LB — the LB would spread scrapes across
  instances and blur per-instance data.

## Session affinity

- **Chat/completions/embeddings/etc.** are stateless — any instance, no
  affinity needed.
- **Realtime WebSocket** pins naturally (the connection lives on one instance).
- **Batch**: submit runs the job on the receiving instance's background task.
  With a shared `Store`, polling `GET /v1/batches/{id}` works from any
  instance; without it, poll must return to the submitting instance (use
  `ip_hash` or a sticky cookie on `/v1/batches`).

## Dynamic config

Config is currently a YAML file loaded at boot; changing keys or models needs
a restart. For a fleet you want to change keys without redeploying. The
roadmap direction:

1. Hot reload: `SIGHUP` / an admin endpoint re-reads the config source and
   swaps it atomically (no dropped connections). Combined with a shared source
   (a mounted ConfigMap, an object in Redis), all instances pick up changes.
2. A `KeyStore` seam backed by Redis/SQL with an admin API to create, revoke,
   and re-quota keys at runtime — every instance reads the live source, so a
   key issued once is valid fleet-wide immediately.

See the [issue tracker](https://github.com/cocoonstack/gateway/issues) for status.
