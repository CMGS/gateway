//! Real HTTP transport (reqwest + rustls) and the default scheme-routing dispatch.
//!
//! Engines address accounts without a configured endpoint via `mock://` sentinel
//! URLs. [`DispatchTransport`] — the server default — keeps those in-process
//! ([`MockTransport`]) and sends real URLs over HTTP, so going live is purely an
//! account-config change. Buffered SSE for now — incremental forwarding is
//! future work.

use ap_models::{GResult, GatewayError};

use crate::transport::{MockTransport, Transport, UpstreamBody, UpstreamRequest, UpstreamResponse};

/// Real HTTP transport (reqwest + rustls).
#[derive(Debug)]
pub struct HttpTransport {
    client: reqwest::Client,
}

impl HttpTransport {
    pub fn new(timeout: std::time::Duration) -> GResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| GatewayError::internal("build http client").with_source(e))?;
        Ok(Self { client })
    }
}

#[async_trait::async_trait]
impl Transport for HttpTransport {
    async fn send(&self, req: UpstreamRequest) -> GResult<UpstreamResponse> {
        let method = reqwest::Method::from_bytes(req.method.as_bytes())
            .map_err(|e| GatewayError::bad_request(format!("bad method: {e}")))?;
        let mut builder = self.client.request(method, &req.url);
        for (k, v) in &req.headers {
            builder = builder.header(k, v);
        }
        let resp = builder.body(req.body).send().await.map_err(|e| {
            GatewayError::new(
                ap_consts::ErrCode::FED_RESP_RPC_FAILED,
                502,
                format!("upstream request failed: {e}"),
            )
        })?;
        let status = resp.status().as_u16();
        let is_sse = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.starts_with("text/event-stream"))
            .unwrap_or(false);
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| GatewayError::internal("read upstream body").with_source(e))?
            .to_vec();
        Ok(UpstreamResponse {
            status,
            body: if is_sse {
                UpstreamBody::Sse(bytes)
            } else {
                UpstreamBody::Json(bytes)
            },
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
    pub fn new(timeout: std::time::Duration) -> GResult<Self> {
        Ok(Self {
            mock: MockTransport,
            http: HttpTransport::new(timeout)?,
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
