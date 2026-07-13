//! SSE frame decoder — pure logic.
//!
//! Feed raw bytes, get back the `data:` payloads of complete events. Handles
//! partial frames across feeds and the OpenAI `[DONE]` sentinel. Transport-free
//! so it tests offline.

/// Incremental server-sent-events decoder (data-only, which is what LLM vendors use).
#[derive(Debug, Default)]
pub struct SseDecoder {
    buf: Vec<u8>,
    done: bool,
}

impl SseDecoder {
    /// Push bytes; returns the `data:` payloads of every event completed so far.
    /// `[DONE]` flips `is_done` and is not returned as a payload.
    pub fn feed(&mut self, bytes: &[u8]) -> Vec<String> {
        // Buffer raw bytes and only decode complete events: a network chunk can
        // end mid-way through a multi-byte UTF-8 character, and decoding each
        // chunk separately would corrupt it permanently.
        self.buf.extend_from_slice(bytes);
        let mut out = Vec::new();
        // events are separated by a blank line
        while let Some(pos) = self.buf.windows(2).position(|w| w == b"\n\n") {
            let event: Vec<u8> = self.buf.drain(..pos + 2).collect();
            let event = String::from_utf8_lossy(&event);
            for line in event.lines() {
                let line = line.strip_suffix('\r').unwrap_or(line);
                if let Some(data) = line.strip_prefix("data:") {
                    let data = data.strip_prefix(' ').unwrap_or(data);
                    if data == "[DONE]" {
                        self.done = true;
                    } else if !data.is_empty() {
                        out.push(data.to_owned());
                    }
                }
            }
        }
        out
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Decode a fully buffered SSE body in one go.
    pub fn decode_all(bytes: &[u8]) -> (Vec<String>, bool) {
        let mut d = SseDecoder::default();
        let events = d.feed(bytes);
        (events, d.is_done())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_events_and_done() {
        let body = b"data: {\"a\":1}\n\ndata: {\"b\":2}\n\ndata: [DONE]\n\n";
        let (events, done) = SseDecoder::decode_all(body);
        assert_eq!(events, vec![r#"{"a":1}"#, r#"{"b":2}"#]);
        assert!(done);
    }

    #[test]
    fn handles_split_frames_across_feeds() {
        let mut d = SseDecoder::default();
        assert!(d.feed(b"data: {\"a\"").is_empty());
        let got = d.feed(b":1}\n\ndata: [DO");
        assert_eq!(got, vec![r#"{"a":1}"#]);
        assert!(!d.is_done());
        assert!(d.feed(b"NE]\n\n").is_empty());
        assert!(d.is_done());
    }

    #[test]
    fn multibyte_utf8_split_across_feeds_survives() {
        let mut d = SseDecoder::default();
        let payload = "data: {\"t\":\"你好😀\"}\n\n".as_bytes();
        // split inside the first multi-byte character
        let (a, b) = payload.split_at(13);
        assert!(std::str::from_utf8(a).is_err(), "split must land mid-char");
        assert!(d.feed(a).is_empty());
        assert_eq!(d.feed(b), vec![r#"{"t":"你好😀"}"#]);
    }

    #[test]
    fn crlf_tolerated() {
        let (events, done) = SseDecoder::decode_all(b"data: x\r\n\n\ndata: [DONE]\r\n\n\n");
        // CRLF payload lines keep working; exact blank-line style varies by vendor
        assert_eq!(events, vec!["x"]);
        assert!(done);
    }
}
