//! `ModelType` — the model dispatch key.
//!
//! Enumerates every upstream model method (135 of them) as the switch key
//! used by the engine factory. Wire strings are preserved exactly so requests
//! and configs referring to a model by string stay stable across releases.
//!
//! Regenerate with scratchpad/gen_model_type.py after adding new model types.

use std::fmt;

/// One variant per upstream model method. Serializes to/from the wire string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ModelType {
    #[serde(rename = "ali_cosyvoice")]
    AliCosyvoice,
    #[serde(rename = "ali_embedding")]
    AliEmbedding,
    #[serde(rename = "ali_midjourney")]
    AliMidjourney,
    #[serde(rename = "ali_openai")]
    AliOpenai,
    #[serde(rename = "ali_podcast")]
    AliPodcast,
    #[serde(rename = "ali_qwen")]
    AliQwen,
    #[serde(rename = "ali_qwen_tts_realtime")]
    AliQwenTTSRealtime,
    #[serde(rename = "ali_qwen_vl")]
    AliQwenVL,
    #[serde(rename = "ali_wanx")]
    AliWanx,
    #[serde(rename = "aws_claude")]
    AwsClaude,
    #[serde(rename = "aws_claude3")]
    AwsClaude3,
    #[serde(rename = "aws_claude3_chat")]
    AwsClaude3Chat,
    #[serde(rename = "aws_claude3_sdk")]
    AwsClaude3Sdk,
    #[serde(rename = "aws_claude3_vision")]
    AwsClaude3Vision,
    #[serde(rename = "aws_cohere_command")]
    AwsCohereCommand,
    #[serde(rename = "aws_deepseek")]
    AwsDeepseek,
    #[serde(rename = "aws_llama")]
    AwsLlama,
    #[serde(rename = "aws_stability_ai")]
    AwsStabilityAI,
    #[serde(rename = "aws_stability_ai_v3")]
    AwsStabilityAIV3,
    #[serde(rename = "azure_claude")]
    AzureClaude,
    #[serde(rename = "azure_gpt_image")]
    AzureGptImage,
    #[serde(rename = "realtime")]
    AzureRealtime,
    #[serde(rename = "azure_stt")]
    AzureSTT,
    #[serde(rename = "azure_stt_batch")]
    AzureSTTBatch,
    #[serde(rename = "azure_tts")]
    AzureTTS,
    #[serde(rename = "azure_tts_batch")]
    AzureTTSBatch,
    #[serde(rename = "baichuan_npc")]
    BaichuanNpc,
    #[serde(rename = "baichuan_turbo")]
    BaichuanTurbo,
    #[serde(rename = "ernie")]
    BaiduErnie,
    #[serde(rename = "bing_chat")]
    BingChat,
    #[serde(rename = "bingsearch")]
    BingSearch,
    #[serde(rename = "brave")]
    Brave,
    #[serde(rename = "chat/completions")]
    Chat,
    #[serde(rename = "chat-bison")]
    ChatBison,
    #[serde(rename = "chatgpt")]
    ChatGpt,
    #[serde(rename = "claude")]
    Claude,
    #[serde(rename = "completions")]
    Completion,
    #[serde(rename = "deepseek")]
    DeepSeek,
    #[serde(rename = "e2b_sandbox")]
    E2BSandbox,
    #[serde(rename = "elevenlab_audio_isolation")]
    ElevenLabAudioIsolation,
    #[serde(rename = "elevenlab_forced_alignment")]
    ElevenLabForcedAlignment,
    #[serde(rename = "elevenlab_music")]
    ElevenLabMusic,
    #[serde(rename = "elevenlab_music_video_to_music")]
    ElevenLabMusicVideoToMsic,
    #[serde(rename = "elevenlab_sts")]
    ElevenLabSTS,
    #[serde(rename = "elevenlab_stt")]
    ElevenLabSTT,
    #[serde(rename = "elevenlab_stt_realtime")]
    ElevenLabSTTRealtime,
    #[serde(rename = "elevenlab_sound_effect")]
    ElevenLabSoundEffect,
    #[serde(rename = "elevenlab_ttd")]
    ElevenLabTTD,
    #[serde(rename = "elevenlab_tts")]
    ElevenLabTTS,
    #[serde(rename = "elevenlab_tts_multi")]
    ElevenLabTTSMulti,
    #[serde(rename = "elevenlab_tts_realtime")]
    ElevenLabTTSRealtime,
    #[serde(rename = "embeddings")]
    Embedding,
    #[serde(rename = "flux")]
    Flux,
    #[serde(rename = "gcp_claude3")]
    GcpClaude3,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "gemini-embedding")]
    GeminiEmbedding,
    #[serde(rename = "gemini_realtime")]
    GeminiRealtime,
    #[serde(rename = "gemini_stt")]
    GeminiStt,
    #[serde(rename = "gemini_tts")]
    GeminiTts,
    #[serde(rename = "glm-realtime")]
    GlmRealtime,
    #[serde(rename = "google_custom_search")]
    GoogleCustomSearch,
    #[serde(rename = "google_gcs")]
    GoogleGcs,
    #[serde(rename = "google-interactions")]
    GoogleInteractions,
    #[serde(rename = "google_stt")]
    GoogleStt,
    #[serde(rename = "google_tts")]
    GoogleTts,
    #[serde(rename = "gptv")]
    GptV,
    #[serde(rename = "oci-grok-imagine")]
    GrokImagine,
    #[serde(rename = "hunyuan")]
    Hunyuan,
    #[serde(rename = "ideogram")]
    Ideogram,
    #[serde(rename = "images/generations")]
    ImageGenerations,
    #[serde(rename = "kling")]
    Kling,
    #[serde(rename = "ksyun")]
    Ksyun,
    #[serde(rename = "minimax")]
    MiniMax,
    #[serde(rename = "minimax_pro")]
    MiniMaxPro,
    #[serde(rename = "minimax_v2")]
    MiniMaxV2,
    #[serde(rename = "minimax_video")]
    MiniMaxVideo,
    #[serde(rename = "minimax_t2a_pro")]
    MinimaxT2APro,
    #[serde(rename = "minimax_t2a_v2")]
    MinimaxT2AV2,
    #[serde(rename = "minimax_voice_clone")]
    MinimaxVoiceClone,
    #[serde(rename = "minimax_voice_design")]
    MinimaxVoiceDesign,
    #[serde(rename = "mistral")]
    Mistral,
    #[serde(rename = "moonshot")]
    Moonshot,
    #[serde(rename = "native_gcp_claude")]
    NativeGcpClaude,
    #[serde(rename = "bing")]
    NewBing,
    #[serde(rename = "oci_cohere_chat")]
    OciCohereChat,
    #[serde(rename = "oci_embedding")]
    OciEmbedding,
    #[serde(rename = "oci_generic_chat")]
    OciGenericChat,
    #[serde(rename = "openai-chat")]
    OpenaiChat,
    #[serde(rename = "chat-like")]
    OpenaiChatLike,
    #[serde(rename = "openai-completions")]
    OpenaiCompletions,
    #[serde(rename = "openai-embeddings")]
    OpenaiEmbeddings,
    #[serde(rename = "openai-picture")]
    OpenaiPicture,
    #[serde(rename = "openai-tts")]
    OpenaiTts,
    #[serde(rename = "openai-vision")]
    OpenaiVision,
    #[serde(rename = "openrouter")]
    Openrouter,
    #[serde(rename = "openrouter_claude")]
    OpenrouterClaude,
    #[serde(rename = "openrouter_common")]
    OpenrouterCommon,
    #[serde(rename = "openrouter_gemini")]
    OpenrouterGemini,
    #[serde(rename = "perplexity-chat")]
    PerplexityChat,
    #[serde(rename = "picture")]
    Picture,
    #[serde(rename = "playground")]
    Playground,
    #[serde(rename = "responses")]
    Responses,
    #[serde(rename = "runway")]
    Runway,
    #[serde(rename = "sense_nova_chat")]
    SenseNovaChat,
    #[serde(rename = "sense_nova_embed")]
    SenseNovaEmbed,
    #[serde(rename = "sense_nova_vision")]
    SenseNovaVision,
    #[serde(rename = "serp_api")]
    SerpApi,
    #[serde(rename = "serp_search")]
    SerpSearch,
    #[serde(rename = "sora")]
    Sora,
    #[serde(rename = "spark")]
    Spark,
    #[serde(rename = "stepfun_chat")]
    StepFunChat,
    #[serde(rename = "tencent")]
    Tencent,
    #[serde(rename = "text-bison")]
    TextBison,
    #[serde(rename = "textin_to_markdown")]
    TextinMarkdown,
    #[serde(rename = "tts")]
    Tts,
    #[serde(rename = "vertex-embed-content")]
    VertexEmbedContent,
    #[serde(rename = "vertex-embedding")]
    VertexEmbedding,
    #[serde(rename = "vertex-embedding-multi")]
    VertexEmbeddingMulti,
    #[serde(rename = "vertex-gemma")]
    VertexGemma,
    #[serde(rename = "vertex-image")]
    VertexImage,
    #[serde(rename = "vertex-image-text")]
    VertexImageText,
    #[serde(rename = "vertex-veo")]
    VertexVeo,
    #[serde(rename = "vidu")]
    Vidu,
    #[serde(rename = "audio/transcriptions")]
    Whisper,
    #[serde(rename = "yiyan")]
    YiYan,
    #[serde(rename = "you")]
    You,
    #[serde(rename = "zhipu-chat")]
    ZhipuChat,
    #[serde(rename = "zhipu-cog")]
    ZhipuCog,
    #[serde(rename = "zhipu-vision")]
    ZhipuVision,
}

impl ModelType {
    /// The exact wire string for this model type (e.g. `"openai-chat"`).
    pub const fn as_str(self) -> &'static str {
        match self {
            ModelType::AliCosyvoice => "ali_cosyvoice",
            ModelType::AliEmbedding => "ali_embedding",
            ModelType::AliMidjourney => "ali_midjourney",
            ModelType::AliOpenai => "ali_openai",
            ModelType::AliPodcast => "ali_podcast",
            ModelType::AliQwen => "ali_qwen",
            ModelType::AliQwenTTSRealtime => "ali_qwen_tts_realtime",
            ModelType::AliQwenVL => "ali_qwen_vl",
            ModelType::AliWanx => "ali_wanx",
            ModelType::AwsClaude => "aws_claude",
            ModelType::AwsClaude3 => "aws_claude3",
            ModelType::AwsClaude3Chat => "aws_claude3_chat",
            ModelType::AwsClaude3Sdk => "aws_claude3_sdk",
            ModelType::AwsClaude3Vision => "aws_claude3_vision",
            ModelType::AwsCohereCommand => "aws_cohere_command",
            ModelType::AwsDeepseek => "aws_deepseek",
            ModelType::AwsLlama => "aws_llama",
            ModelType::AwsStabilityAI => "aws_stability_ai",
            ModelType::AwsStabilityAIV3 => "aws_stability_ai_v3",
            ModelType::AzureClaude => "azure_claude",
            ModelType::AzureGptImage => "azure_gpt_image",
            ModelType::AzureRealtime => "realtime",
            ModelType::AzureSTT => "azure_stt",
            ModelType::AzureSTTBatch => "azure_stt_batch",
            ModelType::AzureTTS => "azure_tts",
            ModelType::AzureTTSBatch => "azure_tts_batch",
            ModelType::BaichuanNpc => "baichuan_npc",
            ModelType::BaichuanTurbo => "baichuan_turbo",
            ModelType::BaiduErnie => "ernie",
            ModelType::BingChat => "bing_chat",
            ModelType::BingSearch => "bingsearch",
            ModelType::Brave => "brave",
            ModelType::Chat => "chat/completions",
            ModelType::ChatBison => "chat-bison",
            ModelType::ChatGpt => "chatgpt",
            ModelType::Claude => "claude",
            ModelType::Completion => "completions",
            ModelType::DeepSeek => "deepseek",
            ModelType::E2BSandbox => "e2b_sandbox",
            ModelType::ElevenLabAudioIsolation => "elevenlab_audio_isolation",
            ModelType::ElevenLabForcedAlignment => "elevenlab_forced_alignment",
            ModelType::ElevenLabMusic => "elevenlab_music",
            ModelType::ElevenLabMusicVideoToMsic => "elevenlab_music_video_to_music",
            ModelType::ElevenLabSTS => "elevenlab_sts",
            ModelType::ElevenLabSTT => "elevenlab_stt",
            ModelType::ElevenLabSTTRealtime => "elevenlab_stt_realtime",
            ModelType::ElevenLabSoundEffect => "elevenlab_sound_effect",
            ModelType::ElevenLabTTD => "elevenlab_ttd",
            ModelType::ElevenLabTTS => "elevenlab_tts",
            ModelType::ElevenLabTTSMulti => "elevenlab_tts_multi",
            ModelType::ElevenLabTTSRealtime => "elevenlab_tts_realtime",
            ModelType::Embedding => "embeddings",
            ModelType::Flux => "flux",
            ModelType::GcpClaude3 => "gcp_claude3",
            ModelType::Gemini => "gemini",
            ModelType::GeminiEmbedding => "gemini-embedding",
            ModelType::GeminiRealtime => "gemini_realtime",
            ModelType::GeminiStt => "gemini_stt",
            ModelType::GeminiTts => "gemini_tts",
            ModelType::GlmRealtime => "glm-realtime",
            ModelType::GoogleCustomSearch => "google_custom_search",
            ModelType::GoogleGcs => "google_gcs",
            ModelType::GoogleInteractions => "google-interactions",
            ModelType::GoogleStt => "google_stt",
            ModelType::GoogleTts => "google_tts",
            ModelType::GptV => "gptv",
            ModelType::GrokImagine => "oci-grok-imagine",
            ModelType::Hunyuan => "hunyuan",
            ModelType::Ideogram => "ideogram",
            ModelType::ImageGenerations => "images/generations",
            ModelType::Kling => "kling",
            ModelType::Ksyun => "ksyun",
            ModelType::MiniMax => "minimax",
            ModelType::MiniMaxPro => "minimax_pro",
            ModelType::MiniMaxV2 => "minimax_v2",
            ModelType::MiniMaxVideo => "minimax_video",
            ModelType::MinimaxT2APro => "minimax_t2a_pro",
            ModelType::MinimaxT2AV2 => "minimax_t2a_v2",
            ModelType::MinimaxVoiceClone => "minimax_voice_clone",
            ModelType::MinimaxVoiceDesign => "minimax_voice_design",
            ModelType::Mistral => "mistral",
            ModelType::Moonshot => "moonshot",
            ModelType::NativeGcpClaude => "native_gcp_claude",
            ModelType::NewBing => "bing",
            ModelType::OciCohereChat => "oci_cohere_chat",
            ModelType::OciEmbedding => "oci_embedding",
            ModelType::OciGenericChat => "oci_generic_chat",
            ModelType::OpenaiChat => "openai-chat",
            ModelType::OpenaiChatLike => "chat-like",
            ModelType::OpenaiCompletions => "openai-completions",
            ModelType::OpenaiEmbeddings => "openai-embeddings",
            ModelType::OpenaiPicture => "openai-picture",
            ModelType::OpenaiTts => "openai-tts",
            ModelType::OpenaiVision => "openai-vision",
            ModelType::Openrouter => "openrouter",
            ModelType::OpenrouterClaude => "openrouter_claude",
            ModelType::OpenrouterCommon => "openrouter_common",
            ModelType::OpenrouterGemini => "openrouter_gemini",
            ModelType::PerplexityChat => "perplexity-chat",
            ModelType::Picture => "picture",
            ModelType::Playground => "playground",
            ModelType::Responses => "responses",
            ModelType::Runway => "runway",
            ModelType::SenseNovaChat => "sense_nova_chat",
            ModelType::SenseNovaEmbed => "sense_nova_embed",
            ModelType::SenseNovaVision => "sense_nova_vision",
            ModelType::SerpApi => "serp_api",
            ModelType::SerpSearch => "serp_search",
            ModelType::Sora => "sora",
            ModelType::Spark => "spark",
            ModelType::StepFunChat => "stepfun_chat",
            ModelType::Tencent => "tencent",
            ModelType::TextBison => "text-bison",
            ModelType::TextinMarkdown => "textin_to_markdown",
            ModelType::Tts => "tts",
            ModelType::VertexEmbedContent => "vertex-embed-content",
            ModelType::VertexEmbedding => "vertex-embedding",
            ModelType::VertexEmbeddingMulti => "vertex-embedding-multi",
            ModelType::VertexGemma => "vertex-gemma",
            ModelType::VertexImage => "vertex-image",
            ModelType::VertexImageText => "vertex-image-text",
            ModelType::VertexVeo => "vertex-veo",
            ModelType::Vidu => "vidu",
            ModelType::Whisper => "audio/transcriptions",
            ModelType::YiYan => "yiyan",
            ModelType::You => "you",
            ModelType::ZhipuChat => "zhipu-chat",
            ModelType::ZhipuCog => "zhipu-cog",
            ModelType::ZhipuVision => "zhipu-vision",
        }
    }

    /// Parse a wire string back into a `ModelType`. Inverse of `as_str`.
    pub fn from_wire(s: &str) -> Option<ModelType> {
        Some(match s {
            "ali_cosyvoice" => ModelType::AliCosyvoice,
            "ali_embedding" => ModelType::AliEmbedding,
            "ali_midjourney" => ModelType::AliMidjourney,
            "ali_openai" => ModelType::AliOpenai,
            "ali_podcast" => ModelType::AliPodcast,
            "ali_qwen" => ModelType::AliQwen,
            "ali_qwen_tts_realtime" => ModelType::AliQwenTTSRealtime,
            "ali_qwen_vl" => ModelType::AliQwenVL,
            "ali_wanx" => ModelType::AliWanx,
            "aws_claude" => ModelType::AwsClaude,
            "aws_claude3" => ModelType::AwsClaude3,
            "aws_claude3_chat" => ModelType::AwsClaude3Chat,
            "aws_claude3_sdk" => ModelType::AwsClaude3Sdk,
            "aws_claude3_vision" => ModelType::AwsClaude3Vision,
            "aws_cohere_command" => ModelType::AwsCohereCommand,
            "aws_deepseek" => ModelType::AwsDeepseek,
            "aws_llama" => ModelType::AwsLlama,
            "aws_stability_ai" => ModelType::AwsStabilityAI,
            "aws_stability_ai_v3" => ModelType::AwsStabilityAIV3,
            "azure_claude" => ModelType::AzureClaude,
            "azure_gpt_image" => ModelType::AzureGptImage,
            "realtime" => ModelType::AzureRealtime,
            "azure_stt" => ModelType::AzureSTT,
            "azure_stt_batch" => ModelType::AzureSTTBatch,
            "azure_tts" => ModelType::AzureTTS,
            "azure_tts_batch" => ModelType::AzureTTSBatch,
            "baichuan_npc" => ModelType::BaichuanNpc,
            "baichuan_turbo" => ModelType::BaichuanTurbo,
            "ernie" => ModelType::BaiduErnie,
            "bing_chat" => ModelType::BingChat,
            "bingsearch" => ModelType::BingSearch,
            "brave" => ModelType::Brave,
            "chat/completions" => ModelType::Chat,
            "chat-bison" => ModelType::ChatBison,
            "chatgpt" => ModelType::ChatGpt,
            "claude" => ModelType::Claude,
            "completions" => ModelType::Completion,
            "deepseek" => ModelType::DeepSeek,
            "e2b_sandbox" => ModelType::E2BSandbox,
            "elevenlab_audio_isolation" => ModelType::ElevenLabAudioIsolation,
            "elevenlab_forced_alignment" => ModelType::ElevenLabForcedAlignment,
            "elevenlab_music" => ModelType::ElevenLabMusic,
            "elevenlab_music_video_to_music" => ModelType::ElevenLabMusicVideoToMsic,
            "elevenlab_sts" => ModelType::ElevenLabSTS,
            "elevenlab_stt" => ModelType::ElevenLabSTT,
            "elevenlab_stt_realtime" => ModelType::ElevenLabSTTRealtime,
            "elevenlab_sound_effect" => ModelType::ElevenLabSoundEffect,
            "elevenlab_ttd" => ModelType::ElevenLabTTD,
            "elevenlab_tts" => ModelType::ElevenLabTTS,
            "elevenlab_tts_multi" => ModelType::ElevenLabTTSMulti,
            "elevenlab_tts_realtime" => ModelType::ElevenLabTTSRealtime,
            "embeddings" => ModelType::Embedding,
            "flux" => ModelType::Flux,
            "gcp_claude3" => ModelType::GcpClaude3,
            "gemini" => ModelType::Gemini,
            "gemini-embedding" => ModelType::GeminiEmbedding,
            "gemini_realtime" => ModelType::GeminiRealtime,
            "gemini_stt" => ModelType::GeminiStt,
            "gemini_tts" => ModelType::GeminiTts,
            "glm-realtime" => ModelType::GlmRealtime,
            "google_custom_search" => ModelType::GoogleCustomSearch,
            "google_gcs" => ModelType::GoogleGcs,
            "google-interactions" => ModelType::GoogleInteractions,
            "google_stt" => ModelType::GoogleStt,
            "google_tts" => ModelType::GoogleTts,
            "gptv" => ModelType::GptV,
            "oci-grok-imagine" => ModelType::GrokImagine,
            "hunyuan" => ModelType::Hunyuan,
            "ideogram" => ModelType::Ideogram,
            "images/generations" => ModelType::ImageGenerations,
            "kling" => ModelType::Kling,
            "ksyun" => ModelType::Ksyun,
            "minimax" => ModelType::MiniMax,
            "minimax_pro" => ModelType::MiniMaxPro,
            "minimax_v2" => ModelType::MiniMaxV2,
            "minimax_video" => ModelType::MiniMaxVideo,
            "minimax_t2a_pro" => ModelType::MinimaxT2APro,
            "minimax_t2a_v2" => ModelType::MinimaxT2AV2,
            "minimax_voice_clone" => ModelType::MinimaxVoiceClone,
            "minimax_voice_design" => ModelType::MinimaxVoiceDesign,
            "mistral" => ModelType::Mistral,
            "moonshot" => ModelType::Moonshot,
            "native_gcp_claude" => ModelType::NativeGcpClaude,
            "bing" => ModelType::NewBing,
            "oci_cohere_chat" => ModelType::OciCohereChat,
            "oci_embedding" => ModelType::OciEmbedding,
            "oci_generic_chat" => ModelType::OciGenericChat,
            "openai-chat" => ModelType::OpenaiChat,
            "chat-like" => ModelType::OpenaiChatLike,
            "openai-completions" => ModelType::OpenaiCompletions,
            "openai-embeddings" => ModelType::OpenaiEmbeddings,
            "openai-picture" => ModelType::OpenaiPicture,
            "openai-tts" => ModelType::OpenaiTts,
            "openai-vision" => ModelType::OpenaiVision,
            "openrouter" => ModelType::Openrouter,
            "openrouter_claude" => ModelType::OpenrouterClaude,
            "openrouter_common" => ModelType::OpenrouterCommon,
            "openrouter_gemini" => ModelType::OpenrouterGemini,
            "perplexity-chat" => ModelType::PerplexityChat,
            "picture" => ModelType::Picture,
            "playground" => ModelType::Playground,
            "responses" => ModelType::Responses,
            "runway" => ModelType::Runway,
            "sense_nova_chat" => ModelType::SenseNovaChat,
            "sense_nova_embed" => ModelType::SenseNovaEmbed,
            "sense_nova_vision" => ModelType::SenseNovaVision,
            "serp_api" => ModelType::SerpApi,
            "serp_search" => ModelType::SerpSearch,
            "sora" => ModelType::Sora,
            "spark" => ModelType::Spark,
            "stepfun_chat" => ModelType::StepFunChat,
            "tencent" => ModelType::Tencent,
            "text-bison" => ModelType::TextBison,
            "textin_to_markdown" => ModelType::TextinMarkdown,
            "tts" => ModelType::Tts,
            "vertex-embed-content" => ModelType::VertexEmbedContent,
            "vertex-embedding" => ModelType::VertexEmbedding,
            "vertex-embedding-multi" => ModelType::VertexEmbeddingMulti,
            "vertex-gemma" => ModelType::VertexGemma,
            "vertex-image" => ModelType::VertexImage,
            "vertex-image-text" => ModelType::VertexImageText,
            "vertex-veo" => ModelType::VertexVeo,
            "vidu" => ModelType::Vidu,
            "audio/transcriptions" => ModelType::Whisper,
            "yiyan" => ModelType::YiYan,
            "you" => ModelType::You,
            "zhipu-chat" => ModelType::ZhipuChat,
            "zhipu-cog" => ModelType::ZhipuCog,
            "zhipu-vision" => ModelType::ZhipuVision,
            _ => return None,
        })
    }

    /// Every known model type, in stable order. Useful for the `/v1/models` route.
    pub const ALL: &'static [ModelType] = &[
        ModelType::AliCosyvoice,
        ModelType::AliEmbedding,
        ModelType::AliMidjourney,
        ModelType::AliOpenai,
        ModelType::AliPodcast,
        ModelType::AliQwen,
        ModelType::AliQwenTTSRealtime,
        ModelType::AliQwenVL,
        ModelType::AliWanx,
        ModelType::AwsClaude,
        ModelType::AwsClaude3,
        ModelType::AwsClaude3Chat,
        ModelType::AwsClaude3Sdk,
        ModelType::AwsClaude3Vision,
        ModelType::AwsCohereCommand,
        ModelType::AwsDeepseek,
        ModelType::AwsLlama,
        ModelType::AwsStabilityAI,
        ModelType::AwsStabilityAIV3,
        ModelType::AzureClaude,
        ModelType::AzureGptImage,
        ModelType::AzureRealtime,
        ModelType::AzureSTT,
        ModelType::AzureSTTBatch,
        ModelType::AzureTTS,
        ModelType::AzureTTSBatch,
        ModelType::BaichuanNpc,
        ModelType::BaichuanTurbo,
        ModelType::BaiduErnie,
        ModelType::BingChat,
        ModelType::BingSearch,
        ModelType::Brave,
        ModelType::Chat,
        ModelType::ChatBison,
        ModelType::ChatGpt,
        ModelType::Claude,
        ModelType::Completion,
        ModelType::DeepSeek,
        ModelType::E2BSandbox,
        ModelType::ElevenLabAudioIsolation,
        ModelType::ElevenLabForcedAlignment,
        ModelType::ElevenLabMusic,
        ModelType::ElevenLabMusicVideoToMsic,
        ModelType::ElevenLabSTS,
        ModelType::ElevenLabSTT,
        ModelType::ElevenLabSTTRealtime,
        ModelType::ElevenLabSoundEffect,
        ModelType::ElevenLabTTD,
        ModelType::ElevenLabTTS,
        ModelType::ElevenLabTTSMulti,
        ModelType::ElevenLabTTSRealtime,
        ModelType::Embedding,
        ModelType::Flux,
        ModelType::GcpClaude3,
        ModelType::Gemini,
        ModelType::GeminiEmbedding,
        ModelType::GeminiRealtime,
        ModelType::GeminiStt,
        ModelType::GeminiTts,
        ModelType::GlmRealtime,
        ModelType::GoogleCustomSearch,
        ModelType::GoogleGcs,
        ModelType::GoogleInteractions,
        ModelType::GoogleStt,
        ModelType::GoogleTts,
        ModelType::GptV,
        ModelType::GrokImagine,
        ModelType::Hunyuan,
        ModelType::Ideogram,
        ModelType::ImageGenerations,
        ModelType::Kling,
        ModelType::Ksyun,
        ModelType::MiniMax,
        ModelType::MiniMaxPro,
        ModelType::MiniMaxV2,
        ModelType::MiniMaxVideo,
        ModelType::MinimaxT2APro,
        ModelType::MinimaxT2AV2,
        ModelType::MinimaxVoiceClone,
        ModelType::MinimaxVoiceDesign,
        ModelType::Mistral,
        ModelType::Moonshot,
        ModelType::NativeGcpClaude,
        ModelType::NewBing,
        ModelType::OciCohereChat,
        ModelType::OciEmbedding,
        ModelType::OciGenericChat,
        ModelType::OpenaiChat,
        ModelType::OpenaiChatLike,
        ModelType::OpenaiCompletions,
        ModelType::OpenaiEmbeddings,
        ModelType::OpenaiPicture,
        ModelType::OpenaiTts,
        ModelType::OpenaiVision,
        ModelType::Openrouter,
        ModelType::OpenrouterClaude,
        ModelType::OpenrouterCommon,
        ModelType::OpenrouterGemini,
        ModelType::PerplexityChat,
        ModelType::Picture,
        ModelType::Playground,
        ModelType::Responses,
        ModelType::Runway,
        ModelType::SenseNovaChat,
        ModelType::SenseNovaEmbed,
        ModelType::SenseNovaVision,
        ModelType::SerpApi,
        ModelType::SerpSearch,
        ModelType::Sora,
        ModelType::Spark,
        ModelType::StepFunChat,
        ModelType::Tencent,
        ModelType::TextBison,
        ModelType::TextinMarkdown,
        ModelType::Tts,
        ModelType::VertexEmbedContent,
        ModelType::VertexEmbedding,
        ModelType::VertexEmbeddingMulti,
        ModelType::VertexGemma,
        ModelType::VertexImage,
        ModelType::VertexImageText,
        ModelType::VertexVeo,
        ModelType::Vidu,
        ModelType::Whisper,
        ModelType::YiYan,
        ModelType::You,
        ModelType::ZhipuChat,
        ModelType::ZhipuCog,
        ModelType::ZhipuVision,
    ];
}

impl fmt::Display for ModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for ModelType {
    type Err = UnknownModelType;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ModelType::from_wire(s).ok_or_else(|| UnknownModelType(s.to_owned()))
    }
}

/// Error for an unrecognized model wire string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownModelType(pub String);

impl fmt::Display for UnknownModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown model type: {}", self.0)
    }
}
impl std::error::Error for UnknownModelType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_all() {
        for &m in ModelType::ALL {
            assert_eq!(ModelType::from_wire(m.as_str()), Some(m));
        }
    }

    #[test]
    fn variant_count() {
        assert_eq!(ModelType::ALL.len(), 129);
    }

    #[test]
    fn known_wire_values() {
        assert_eq!(ModelType::OpenaiChat.as_str(), "openai-chat");
        assert_eq!(ModelType::from_wire("claude"), Some(ModelType::Claude));
        assert_eq!(ModelType::from_wire("nope"), None);
    }
}
