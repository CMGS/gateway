//! Real HTTP transport (reqwest + rustls) and the default scheme-routing dispatch.
//!
//! Engines address accounts without a configured endpoint via `mock://` sentinel
//! URLs. [`DispatchTransport`] — the server default — keeps those in-process
//! ([`MockTransport`]) and sends real URLs over HTTP, so going live is purely an
//! account-config change. Buffered SSE for now — incremental forwarding is
//! future work.

use std::collections::HashMap;
use std::time::Duration;

use ap_models::{GResult, GatewayError};

use crate::transport::{MockTransport, Transport, UpstreamBody, UpstreamRequest, UpstreamResponse};

const RETRY_BACKOFF: Duration = Duration::from_millis(100);
// A hung connect (black-holed SYN) must surface as a connect error — which the
// retry predicate covers — instead of burning the whole request timeout.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Per-account upstream policy: request timeout and how many times a
/// connect-phase failure is retried (a request that reached the vendor is
/// never replayed — LLM calls are not idempotent).
#[derive(Debug, Clone, Copy)]
pub struct UpstreamPolicy {
    pub timeout: Duration,
    pub connect_retries: u32,
}

impl Default for UpstreamPolicy {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(60),
            connect_retries: 1,
        }
    }
}

/// Real HTTP transport (reqwest + rustls).
#[derive(Debug)]
pub struct HttpTransport {
    client: reqwest::Client,
    default_policy: UpstreamPolicy,
    per_account: HashMap<String, UpstreamPolicy>,
}

impl HttpTransport {
    pub fn new(timeout: Duration) -> GResult<Self> {
        Self::with_policies(
            UpstreamPolicy {
                timeout,
                ..UpstreamPolicy::default()
            },
            HashMap::new(),
        )
    }

    pub fn with_policies(
        default_policy: UpstreamPolicy,
        per_account: HashMap<String, UpstreamPolicy>,
    ) -> GResult<Self> {
        let client = reqwest::Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .build()
            .map_err(|e| GatewayError::internal("build http client").with_source(e))?;
        Ok(Self {
            client,
            default_policy,
            per_account,
        })
    }

    pub fn policy_for(&self, account: &str) -> UpstreamPolicy {
        self.per_account
            .get(account)
            .copied()
            .unwrap_or(self.default_policy)
    }
}

#[async_trait::async_trait]
impl Transport for HttpTransport {
    async fn send(&self, req: UpstreamRequest) -> GResult<UpstreamResponse> {
        let method = reqwest::Method::from_bytes(req.method.as_bytes())
            .map_err(|e| GatewayError::bad_request(format!("bad method: {e}")))?;
        let policy = self.policy_for(&req.account);
        let body = bytes::Bytes::from(req.body);
        let mut attempt = 0u32;
        let resp = loop {
            let mut builder = self
                .client
                .request(method.clone(), &req.url)
                .timeout(policy.timeout);
            for (k, v) in &req.headers {
                builder = builder.header(k, v);
            }
            match builder.body(body.clone()).send().await {
                Ok(resp) => break resp,
                Err(e) if e.is_connect() && attempt < policy.connect_retries => {
                    attempt += 1;
                    metrics::counter!(
                        "gateway_upstream_connect_retries_total",
                        "account" => req.account.clone(),
                    )
                    .increment(1);
                    tokio::time::sleep(RETRY_BACKOFF * attempt).await;
                }
                Err(e) => {
                    return Err(GatewayError::new(
                        ap_consts::ErrCode::FED_RESP_RPC_FAILED,
                        502,
                        format!("upstream request failed: {e}"),
                    ));
                }
            }
        };
        let status = resp.status().as_u16();
        let is_sse = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.starts_with("text/event-stream"))
            .unwrap_or(false);
        if is_sse {
            use futures::TryStreamExt;
            return Ok(UpstreamResponse {
                status,
                body: UpstreamBody::SseStream(Box::pin(
                    resp.bytes_stream().map_err(|e| e.to_string()),
                )),
            });
        }
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| GatewayError::internal("read upstream body").with_source(e))?
            .to_vec();
        Ok(UpstreamResponse {
            status,
            body: UpstreamBody::Json(bytes),
        })
    }
}

/// Default transport: `mock://` sentinel URLs (accounts with no configured
/// endpoint) stay in-process, everything else goes over real HTTP.
#[derive(Debug)]
pub struct DispatchTransport {
    mock: MockTransport,
    http: HttpTransport,
}

impl DispatchTransport {
    pub fn new(timeout: Duration) -> GResult<Self> {
        Ok(Self {
            mock: MockTransport,
            http: HttpTransport::new(timeout)?,
        })
    }

    pub fn with_policies(
        default_policy: UpstreamPolicy,
        per_account: HashMap<String, UpstreamPolicy>,
    ) -> GResult<Self> {
        Ok(Self {
            mock: MockTransport,
            http: HttpTransport::with_policies(default_policy, per_account)?,
        })
    }
}

#[async_trait::async_trait]
impl Transport for DispatchTransport {
    async fn send(&self, req: UpstreamRequest) -> GResult<UpstreamResponse> {
        if req.url.starts_with("mock://") {
            self.mock.send(req).await
        } else {
            self.http.send(req).await
        }
    }
}
