//! Engine factory.
//!
//! Dispatches on the model type via [`ModelFamily`] (consts::family — every one
//! of the 135 ModelTypes is classified, enforced at compile time). Engines
//! receive a [`SharedTransport`].
//! The Realtime family is not a chat-pipeline engine: it is
//! served on the /v1/realtime WebSocket surface (views layer), so this factory
//! answers 501-with-pointer for it on the chat path.

use ap_consts::{ErrCode, ModelFamily, ModelType};
use ap_models::{GResult, GatewayError, GatewayRequest};

use crate::bespoke::{CohereEngine, DashScopeEngine, ErnieEngine, LlamaEngine, MinimaxV1Engine};
use crate::claude_engine::ClaudeEngine;
use crate::engine::ModelEngine;
use crate::families::{
    AudioEngine, AudioKind, CompletionsEngine, EmbeddingsEngine, ImageEngine, PassthroughEngine,
    ResponsesEngine, SearchEngine, VertexEngine, VideoEngine,
};
use crate::openai_engine::OpenAiEngine;
use crate::transport::SharedTransport;

/// Whether the gateway serves `mt`. All 135 types: chat-pipeline families go
/// through `get_engine`; the Realtime family is served on the /v1/realtime
/// WebSocket surface (mock session locally; real upstream bridging is future work).
pub fn is_implemented(_mt: ModelType) -> bool {
    true
}

/// Build the engine for a request.
///
/// Bespoke vendors (whose real wire shape differs from their family's canonical
/// protocol) dispatch to their dedicated engines first; everything else goes by
/// family.
pub fn get_engine(
    request: GatewayRequest,
    transport: SharedTransport,
) -> GResult<Box<dyn ModelEngine>> {
    let mt = request
        .model_type()
        .ok_or_else(|| GatewayError::bad_request("request missing model_param_v2"))?;

    // bespoke wire shapes (first batch of 4 vendors)
    match mt {
        ModelType::BaiduErnie => return Ok(Box::new(ErnieEngine::new(request, transport))),
        ModelType::MiniMax | ModelType::MiniMaxPro => {
            return Ok(Box::new(MinimaxV1Engine::new(request, transport)));
        }
        ModelType::AwsCohereCommand => return Ok(Box::new(CohereEngine::new(request, transport))),
        ModelType::AwsLlama => return Ok(Box::new(LlamaEngine::new(request, transport))),
        ModelType::AliQwen | ModelType::AliQwenVL => {
            return Ok(Box::new(DashScopeEngine::new(request, transport)));
        }
        // OpenAI Responses API — distinct wire shape (output items + input/output
        // token usage), not chat/completions.
        ModelType::Responses => return Ok(Box::new(ResponsesEngine::new(request, transport))),
        // Legacy text completions — {prompt} in, {choices[].text} out; distinct
        // from chat (would otherwise mis-route to the chat engine's /chat/completions).
        ModelType::OpenaiCompletions | ModelType::Completion => {
            return Ok(Box::new(CompletionsEngine::new(request, transport)));
        }
        _ => {}
    }

    Ok(match mt.family() {
        ModelFamily::ChatOpenAi => Box::new(OpenAiEngine::new(request, transport)),
        ModelFamily::ChatMessages => Box::new(ClaudeEngine::new(request, transport)),
        ModelFamily::ChatVertex => Box::new(VertexEngine::new(request, transport)),
        ModelFamily::Embeddings => Box::new(EmbeddingsEngine::new(request, transport)),
        ModelFamily::Image => Box::new(ImageEngine::new(request, transport)),
        ModelFamily::AudioTts => Box::new(AudioEngine::new(request, transport, AudioKind::Tts)),
        ModelFamily::AudioStt => Box::new(AudioEngine::new(request, transport, AudioKind::Stt)),
        ModelFamily::AudioOther => Box::new(AudioEngine::new(request, transport, AudioKind::Other)),
        ModelFamily::Video => Box::new(VideoEngine::new(request, transport)),
        ModelFamily::Search => Box::new(SearchEngine::new(request, transport)),
        ModelFamily::Misc => Box::new(PassthroughEngine::new(request, transport)),
        ModelFamily::Realtime => {
            // realtime bypasses the chat pipeline: served on the /v1/realtime WebSocket surface
            return Err(GatewayError::new(
                ErrCode::INTERNAL_UNKNOWN,
                501,
                format!(
                    "realtime model `{}` is served on the /v1/realtime websocket surface, not the chat surface",
                    mt.as_str()
                ),
            ));
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;
    use ap_models::ModelParamV2;
    use std::sync::Arc;

    fn req(mt: ModelType) -> GatewayRequest {
        GatewayRequest {
            model_param_v2: Some(ModelParamV2::new(mt)),
            ..Default::default()
        }
    }

    #[test]
    fn every_non_realtime_type_dispatches() {
        let t: SharedTransport = Arc::new(MockTransport);
        let mut dispatched = 0;
        for &mt in ModelType::ALL {
            let got = get_engine(req(mt), t.clone());
            if mt.family() == ModelFamily::Realtime {
                assert_eq!(got.err().map(|e| e.http_status), Some(501), "{mt}");
            } else {
                assert!(got.is_ok(), "no engine for {mt}");
                dispatched += 1;
            }
        }
        assert_eq!(dispatched, ModelType::ALL.len() - 6); // 6 realtime types
    }

    #[test]
    fn rejects_missing_param() {
        let t: SharedTransport = Arc::new(MockTransport);
        let err = get_engine(GatewayRequest::default(), t).err().unwrap();
        assert_eq!(err.http_status, 400);
    }
}
