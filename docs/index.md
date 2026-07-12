# Gateway

Single-binary LLM access point in Rust (binary: `ap`): OpenAI- and
Anthropic-compatible APIs in front of pluggable model providers, with
key-based auth, quotas, rate limits, failover, and a billing ledger.

```
client ──► /v1/* (OpenAI + Anthropic surfaces, streaming SSE, realtime WS)
       ──► pipeline: resolve/quota/cache → account select (PTU, failover)
                     → rate limits → engine → usage → billing ledger
       ──► providers: real endpoints over HTTP · in-process mock for the rest
```

## Guides

- [Architecture](architecture.md) — crate layout, request pipeline, trait
  seams, testing
- [Configuration](configuration.md) — the `gateway.yaml` reference, env
  switches, going live against real upstreams

## Repository

Source and issue tracker:
[github.com/CMGS/gateway](https://github.com/CMGS/gateway).
