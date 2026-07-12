//! Protocol-family classification of every ModelType.
//!
//! Rather than one engine per vendor across 65 vendors, this merges them into
//! a set of family engines by **protocol family**: vendors on the same protocol
//! share one engine implementation, and vendor differences live in params/URL.
//! Every ModelType must map to exactly one family — match exhaustiveness is
//! enforced by the compiler, so an unclassified new ModelType fails to compile.

use crate::model_type::ModelType;

/// The wire-protocol family an engine speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelFamily {
    /// OpenAI chat/completions shape (includes openai-compatible vendors)
    ChatOpenAi,
    /// Anthropic messages shape (the whole claude family)
    ChatMessages,
    /// Google Vertex/Gemini generateContent shape
    ChatVertex,
    /// vector embeddings
    Embeddings,
    /// image generation
    Image,
    /// text-to-speech (TTS)
    AudioTts,
    /// speech-to-text (STT)
    AudioStt,
    /// other audio (sound effects/voice cloning/isolation/alignment/podcast)
    AudioOther,
    /// video generation (async task type)
    Video,
    /// web search
    Search,
    /// realtime bidirectional streaming (websocket upstream bridging is future work)
    Realtime,
    /// dedicated misc (sandbox/document parsing/GCS/interactions)
    Misc,
}

impl ModelType {
    /// Which family engine serves this model type.
    pub const fn family(self) -> ModelFamily {
        use ModelFamily as F;
        use ModelType as M;
        match self {
            // --- OpenAI chat protocol family (native openai + compatible vendors) ---
            M::OpenaiChat
            | M::OpenaiCompletions
            | M::OpenaiChatLike
            | M::OpenaiVision
            | M::Chat
            | M::Completion
            | M::ChatGpt
            | M::GptV
            | M::Responses
            | M::Playground
            | M::AliOpenai
            | M::AliQwen
            | M::AliQwenVL
            | M::AwsDeepseek
            | M::DeepSeek
            | M::Mistral
            | M::Moonshot
            | M::Hunyuan
            | M::Spark
            | M::Tencent
            | M::Ksyun
            | M::YiYan
            | M::BaiduErnie
            | M::BingChat
            | M::NewBing
            | M::BaichuanTurbo
            | M::BaichuanNpc
            | M::ZhipuChat
            | M::ZhipuVision
            | M::StepFunChat
            | M::PerplexityChat
            | M::Openrouter
            | M::OpenrouterCommon
            | M::OpenrouterGemini
            | M::MiniMax
            | M::MiniMaxPro
            | M::MiniMaxV2
            | M::SenseNovaChat
            | M::SenseNovaVision
            | M::OciGenericChat
            | M::OciCohereChat
            | M::AwsCohereCommand
            | M::AwsLlama => F::ChatOpenAi,

            // --- Anthropic messages protocol family ---
            M::Claude
            | M::AwsClaude
            | M::AwsClaude3
            | M::AwsClaude3Chat
            | M::AwsClaude3Sdk
            | M::AwsClaude3Vision
            | M::GcpClaude3
            | M::NativeGcpClaude
            | M::AzureClaude
            | M::OpenrouterClaude => F::ChatMessages,

            // --- Vertex/Gemini generateContent ---
            M::ChatBison | M::TextBison | M::Gemini | M::VertexGemma | M::VertexImageText => {
                F::ChatVertex
            }

            // --- Embeddings ---
            M::OpenaiEmbeddings
            | M::Embedding
            | M::AliEmbedding
            | M::GeminiEmbedding
            | M::VertexEmbedding
            | M::VertexEmbeddingMulti
            | M::VertexEmbedContent
            | M::OciEmbedding
            | M::SenseNovaEmbed => F::Embeddings,

            // --- Image ---
            M::OpenaiPicture
            | M::Picture
            | M::ImageGenerations
            | M::AzureGptImage
            | M::AliWanx
            | M::AliMidjourney
            | M::AwsStabilityAI
            | M::AwsStabilityAIV3
            | M::Flux
            | M::Ideogram
            | M::GrokImagine
            | M::ZhipuCog
            | M::VertexImage => F::Image,

            // --- Audio: TTS ---
            M::OpenaiTts
            | M::Tts
            | M::AzureTTS
            | M::AzureTTSBatch
            | M::GoogleTts
            | M::GeminiTts
            | M::AliCosyvoice
            | M::ElevenLabTTS
            | M::ElevenLabTTSMulti
            | M::ElevenLabTTD
            | M::MinimaxT2APro
            | M::MinimaxT2AV2 => F::AudioTts,

            // --- Audio: STT ---
            M::Whisper
            | M::AzureSTT
            | M::AzureSTTBatch
            | M::GoogleStt
            | M::GeminiStt
            | M::ElevenLabSTT => F::AudioStt,

            // --- Audio: other ---
            M::ElevenLabSTS
            | M::ElevenLabSoundEffect
            | M::ElevenLabMusic
            | M::ElevenLabMusicVideoToMsic
            | M::ElevenLabAudioIsolation
            | M::ElevenLabForcedAlignment
            | M::MinimaxVoiceClone
            | M::MinimaxVoiceDesign
            | M::AliPodcast => F::AudioOther,

            // --- Video (async task type) ---
            M::Sora | M::VertexVeo | M::Kling | M::Runway | M::Vidu | M::MiniMaxVideo => F::Video,

            // --- Search ---
            M::BingSearch
            | M::Brave
            | M::GoogleCustomSearch
            | M::SerpApi
            | M::SerpSearch
            | M::You => F::Search,

            // --- Realtime (websocket upstream) ---
            M::AzureRealtime
            | M::GeminiRealtime
            | M::GlmRealtime
            | M::ElevenLabSTTRealtime
            | M::ElevenLabTTSRealtime
            | M::AliQwenTTSRealtime => F::Realtime,

            // --- Misc ---
            M::E2BSandbox | M::TextinMarkdown | M::GoogleGcs | M::GoogleInteractions => F::Misc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_type_has_a_family() {
        // exhaustiveness is compile-time; this asserts the family split adds up
        let mut counts = std::collections::HashMap::new();
        for &m in ModelType::ALL {
            *counts.entry(m.family()).or_insert(0usize) += 1;
        }
        let total: usize = counts.values().sum();
        assert_eq!(total, ModelType::ALL.len());
        assert_eq!(counts[&ModelFamily::ChatMessages], 10);
        assert_eq!(counts[&ModelFamily::Realtime], 6);
    }

    #[test]
    fn spot_checks() {
        assert_eq!(ModelType::OpenaiChat.family(), ModelFamily::ChatOpenAi);
        assert_eq!(ModelType::Gemini.family(), ModelFamily::ChatVertex);
        assert_eq!(ModelType::Kling.family(), ModelFamily::Video);
        assert_eq!(ModelType::Brave.family(), ModelFamily::Search);
        assert_eq!(ModelType::GlmRealtime.family(), ModelFamily::Realtime);
    }
}
