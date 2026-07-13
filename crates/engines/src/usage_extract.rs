//! Usage normalization.
//!
//! Engines stash the vendor's raw `usage` subtree bytes on `GatewayResponse
//! .raw_usage_json`; this pure function maps them into the normalized
//! [`CommonUsage`] view. The DAG post-process node calls it.

use gw_models::CommonUsage;
use serde_json::Value;

/// Extract a normalized usage view from raw vendor usage JSON.
/// `messages_protocol` selects the Anthropic field map; otherwise OpenAI's.
/// Returns `None` when the bytes are empty/unparseable — callers fall back to
/// the top-level token fields.
pub fn extract_common_usage(raw: &[u8], messages_protocol: bool) -> Option<CommonUsage> {
    if raw.is_empty() {
        return None;
    }
    let v: Value = serde_json::from_slice(raw).ok()?;
    fn get(v: &Value, path: &[&str]) -> i64 {
        let mut cur = v;
        for p in path {
            match cur.get(p) {
                Some(n) => cur = n,
                None => return 0,
            }
        }
        cur.as_i64().unwrap_or(0)
    }

    Some(if messages_protocol {
        // Anthropic: input/output (+ cache fields)
        let input = get(&v, &["input_tokens"]);
        let output = get(&v, &["output_tokens"]);
        let read_cache = get(&v, &["cache_read_input_tokens"]);
        let write_cache = get(&v, &["cache_creation_input_tokens"]);
        CommonUsage {
            platform_input: input,
            read_cache,
            write_cache,
            completion: output,
            reason: 0,
            platform_total: input + output + read_cache + write_cache,
        }
    } else {
        // OpenAI: prompt/completion/total (+ details)
        let prompt = get(&v, &["prompt_tokens"]);
        let completion = get(&v, &["completion_tokens"]);
        let total = get(&v, &["total_tokens"]);
        let read_cache = get(&v, &["prompt_tokens_details", "cached_tokens"]);
        let reason = get(&v, &["completion_tokens_details", "reasoning_tokens"]);
        CommonUsage {
            // cached ⊆ prompt and reasoning ⊆ completion by the vendor contract,
            // but never trust upstream: clamp so malformed usage can't emit
            // negative token counts (→ negative billing).
            platform_input: (prompt - read_cache).max(0),
            read_cache,
            write_cache: 0,
            completion: (completion - reason).max(0),
            reason,
            platform_total: if total > 0 {
                total
            } else {
                prompt + completion
            },
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openai_map() {
        let raw = br#"{"prompt_tokens":10,"completion_tokens":5,"total_tokens":15,
            "prompt_tokens_details":{"cached_tokens":4},
            "completion_tokens_details":{"reasoning_tokens":2}}"#;
        let u = extract_common_usage(raw, false).unwrap();
        assert_eq!(u.platform_input, 6);
        assert_eq!(u.read_cache, 4);
        assert_eq!(u.completion, 3);
        assert_eq!(u.reason, 2);
        assert_eq!(u.platform_total, 15);
    }

    #[test]
    fn malformed_usage_never_bills_negative() {
        // vendor contract violation: more cached than prompt, more reasoning than
        // completion. Must clamp to 0, not emit negative token counts.
        let raw = br#"{"prompt_tokens":3,"completion_tokens":2,"total_tokens":5,
            "prompt_tokens_details":{"cached_tokens":9},
            "completion_tokens_details":{"reasoning_tokens":9}}"#;
        let u = extract_common_usage(raw, false).unwrap();
        assert_eq!(u.platform_input, 0, "clamped, not negative");
        assert_eq!(u.completion, 0, "clamped, not negative");
        assert_eq!(u.read_cache, 9);
        assert_eq!(u.reason, 9);
    }

    #[test]
    fn anthropic_map() {
        let raw = br#"{"input_tokens":8,"output_tokens":6,"cache_read_input_tokens":2}"#;
        let u = extract_common_usage(raw, true).unwrap();
        assert_eq!(u.platform_input, 8);
        assert_eq!(u.completion, 6);
        assert_eq!(u.read_cache, 2);
        assert_eq!(u.platform_total, 16);
    }

    #[test]
    fn empty_or_garbage_is_none() {
        assert!(extract_common_usage(b"", false).is_none());
        assert!(extract_common_usage(b"not-json", false).is_none());
    }
}
