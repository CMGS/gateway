//! Core domain models for the gateway.
//!
//! Layer L1: depends only on `ap-consts`. Holds the request/response types the
//! whole pipeline threads through, the unified error model, and the usage view.

pub mod cost;
pub mod error;
pub mod params;
pub mod recorder;
pub mod request;
pub mod response;
pub mod token_estimate;
pub mod usage;

pub use cost::{TokenInput, TokenRate, platform_input, platform_total};
pub use error::{GResult, GatewayError};
pub use params::{
    ChatParams, EmbeddingParams, ImageParams, SearchParams, SttParams, TtsParams, TypedParams,
    VideoParams,
};
pub use recorder::{Block, Recorder, SimpleRecorder, TimingRecorder};
pub use request::domain::{Account, ChatMsg, ProductConf, ReqExtraParam, UserConf};
pub use request::{GatewayRequest, ModelParamV2};
pub use response::GatewayResponse;
pub use token_estimate::{HeuristicEncoder, TokenEncoder, estimate_prompt_tokens};
pub use usage::CommonUsage;
