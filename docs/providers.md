# Providers

A provider is an upstream vendor the gateway calls. Two ways to configure one:
a **preset** (recommended) or a raw **account**.

## Presets

A `providers:` entry expands into an account with the kind's base URL, served
protocols, and auth style — going live is `kind` + `api_key_env`:

```yaml
providers:
  - name: openai
    kind: openai
    api_key_env: OPENAI_API_KEY
models:
  - name: gpt-4o
    provider: openai      # fills the protocol and pins the model to openai's accounts
```

### Kinds

| kind | base URL | protocols | auth |
|------|----------|-----------|------|
| `openai` | `https://api.openai.com` | chat, embeddings, image, tts, stt, responses, completions | `Bearer` |
| `anthropic` | `https://api.anthropic.com` | anthropic-messages | `x-api-key` + `anthropic-version` |
| `gemini` | `https://generativelanguage.googleapis.com` | gemini | API key |
| `deepseek` | `https://api.deepseek.com` | openai-chat | `Bearer` |
| `openrouter` | `https://openrouter.ai/api` | openai-chat | `Bearer` |

Any other OpenAI-compatible vendor (Qwen, Ollama, vLLM, a relay) uses
`kind: openai` with an `endpoint:` override:

```yaml
providers:
  - name: myvendor
    kind: openai
    endpoint: "https://my-relay.example.com"
    api_key_env: MYVENDOR_KEY
```

A preset also accepts `endpoint`, `timeout_seconds`, and `connect_retries`,
inherited by the synthesized account. An explicit `accounts:` entry with the
same name wins over the preset.

## Going live

1. Put the key in the process environment: `export OPENAI_API_KEY=sk-...`
   (keys never live in the config file — the account names an env var).
2. Configure the provider/account with a real `endpoint` and `api_key_env`.
3. Start the gateway. Requests egress to the real vendor and the ledger records
   real usage.

`GW_TRANSPORT` overrides transport routing: unset routes `mock://` sentinel
URLs in-process and real URLs over HTTP; `mock` forces zero egress; `http`
disables the mock so a misconfigured account fails loudly.

## Accounts, failover, and health

Multiple accounts can serve the same protocol. Selection is by `priority`
(lower first), round-robin within a tie, with PTU-tier accounts preferred over
paygo. On an upstream 5xx the failed account is excluded and another is tried
once (a PTU→paygo switch is flagged `ptu_spillover`). Consecutive failures put
an account into cooldown (`stability.failure_threshold` / `cooldown_seconds`),
and it auto-recovers on expiry. A streaming response that already sent bytes to
the client is never failed over.

## AWS SigV4

AWS Bedrock accounts sign requests with SigV4. Set `api_key_env` to the access
key id's env var and `secret_key_env` to the secret key's; both must resolve or
the account falls back to inert mock credentials.
