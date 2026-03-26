use serde::{Deserialize, Serialize};

/// Base response wrapper for all API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    pub base_resp: Option<BaseResp>,
    #[serde(flatten)]
    pub data: Option<T>,
}

/// Base error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResp {
    pub status_code: i32,
    pub status_msg: Option<String>,
}

/// Task status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Processing,
    Success,
    Fail,
}

// ============== Text-to-Audio ==============

/// Text-to-audio request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextToAudioRequest {
    pub model: String,
    pub text: String,
    pub voice_setting: VoiceSetting,
    pub audio_setting: AudioSetting,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_boost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

/// Voice settings for TTS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSetting {
    pub voice_id: String,
    pub speed: f32,
    pub vol: f32,
    pub pitch: i32,
    pub emotion: String,
}

/// Audio output settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSetting {
    pub sample_rate: i32,
    pub bitrate: i32,
    pub format: String,
    pub channel: i32,
}

/// Text-to-audio response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextToAudioResponse {
    pub audio: Option<String>, // hex encoded or URL
}

/// Text-to-audio response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextToAudioData {
    pub audio: String,
}

// ============== List Voices ==============

/// List voices request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVoicesRequest {
    pub voice_type: String,
}

/// List voices response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVoicesResponse {
    pub system_voice: Option<Vec<Voice>>,
    pub voice_cloning: Option<Vec<Voice>>,
}

/// Voice information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub voice_id: String,
    pub voice_name: String,
}

// ============== Voice Clone ==============

/// Voice clone request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCloneRequest {
    pub file_id: String,
    pub voice_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Voice clone response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCloneResponse {
    pub demo_audio: Option<String>,
}

/// File upload response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub file: Option<FileInfo>,
}

/// Uploaded file information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_id: String,
}

// ============== Video Generation ==============

/// Video generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoGenerationRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_frame_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

/// Video generation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoGenerationResponse {
    pub task_id: Option<String>,
}

/// Video generation response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoGenerationData {
    pub task_id: String,
}

/// Query video status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryVideoResponse {
    pub status: String,
    pub file_id: Option<String>,
    pub download_url: Option<String>,
}

// ============== Image Generation ==============

/// Image generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    pub model: String,
    pub prompt: String,
    pub aspect_ratio: String,
    pub n: i32,
    pub prompt_optimizer: bool,
}

/// Image generation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationResponse {
    pub data: Option<ImageGenerationData>,
}

/// Image generation response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationData {
    pub image_urls: Option<Vec<String>>,
}

// ============== Music Generation ==============

/// Music generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicGenerationRequest {
    pub model: String,
    pub prompt: String,
    pub lyrics: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_setting: Option<MusicAudioSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

/// Music audio settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicAudioSetting {
    pub sample_rate: i32,
    pub bitrate: i32,
    pub format: String,
}

/// Music generation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicGenerationResponse {
    pub data: Option<MusicData>,
}

/// Music generation response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicData {
    pub audio: Option<String>,
}

// ============== Voice Design ==============

/// Voice design request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceDesignRequest {
    pub prompt: String,
    pub preview_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

/// Voice design response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceDesignResponse {
    pub voice_id: Option<String>,
    pub trial_audio: Option<String>,
}

// ============== File Retrieve ==============

/// File retrieve response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRetrieveResponse {
    pub file: Option<FileDetail>,
}

/// File detail information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDetail {
    pub file_id: String,
    pub download_url: Option<String>,
}

// ============== Task Storage Model (SQLite) ==============

/// Stored task for SQLite persistence
#[derive(Debug, Clone)]
pub struct StoredTask {
    pub id: i64,
    pub task_id: String,
    pub task_type: String, // "video", "image", "music"
    pub status: String,    // "pending", "processing", "success", "fail"
    pub prompt: Option<String>,
    pub model: Option<String>,
    pub file_id: Option<String>,
    pub download_url: Option<String>,
    pub local_path: Option<String>,
    pub error_msg: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}
