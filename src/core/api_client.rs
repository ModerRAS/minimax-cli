use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::errors::MinimaxError;
use super::models::*;

pub struct MinimaxClient {
    client: Client,
    api_key: String,
    api_host: String,
}

impl MinimaxClient {
    pub fn new(api_key: String, api_host: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key,
            api_host,
        }
    }

    fn make_request<R: DeserializeOwned, B: Serialize>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<&B>,
    ) -> Result<R, MinimaxError> {
        let url = format!("{}{}", self.api_host, endpoint);

        let mut request = self.client.request(method, &url);
        request = request.header("Authorization", format!("Bearer {}", self.api_key));
        request = request.header("MM-API-Source", "minimax-cli");

        if let Some(body) = body {
            request = request.header("Content-Type", "application/json");
            request = request.json(body);
        }

        let response = request.send()?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().unwrap_or_default();
            return Err(MinimaxError::RequestError(format!(
                "HTTP {}: {}",
                status.as_u16(),
                text
            )));
        }

        let data: serde_json::Value = response.json()?;

        if let Some(base_resp) = data.get("base_resp").and_then(|v| v.as_object()) {
            let status_code = base_resp
                .get("status_code")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;

            if status_code != 0 {
                let message = base_resp
                    .get("status_msg")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();

                match status_code {
                    1004 => return Err(MinimaxError::AuthError(message)),
                    2038 => return Err(MinimaxError::RequestError(format!(
                        "{} (need real-name verification at https://platform.minimaxi.com/user-center/basic-information)", 
                        message
                    ))),
                    _ => return Err(MinimaxError::ApiError { code: status_code, message }),
                }
            }
        }

        serde_json::from_value(data)
            .map_err(|e| MinimaxError::RequestError(format!("Failed to parse response: {}", e)))
    }

    // Text to Audio
    pub fn text_to_audio(&self, req: &TextToAudioRequest) -> Result<String, MinimaxError> {
        #[derive(Deserialize)]
        struct Resp {
            data: Option<AudioData>,
        }
        #[derive(Deserialize)]
        struct AudioData {
            audio: Option<String>,
        }
        let data: Resp = self.make_request(reqwest::Method::POST, "/v1/t2a_v2", Some(req))?;
        data.data
            .and_then(|d| d.audio)
            .ok_or_else(|| MinimaxError::RequestError("No audio data in response".to_string()))
    }

    // List Voices
    pub fn list_voices(&self, voice_type: &str) -> Result<ListVoicesResponse, MinimaxError> {
        #[derive(Serialize)]
        struct Req {
            voice_type: String,
        }
        self.make_request(
            reqwest::Method::POST,
            "/v1/get_voice",
            Some(&Req {
                voice_type: voice_type.to_string(),
            }),
        )
    }

    // Upload File
    pub fn upload_file(
        &self,
        file_data: &[u8],
        filename: &str,
        purpose: &str,
    ) -> Result<FileInfo, MinimaxError> {
        let form = reqwest::blocking::multipart::Form::new()
            .part(
                "file",
                reqwest::blocking::multipart::Part::bytes(file_data.to_vec())
                    .file_name(filename.to_string())
                    .mime_str("audio/mpeg")
                    .map_err(|e| MinimaxError::RequestError(e.to_string()))?,
            )
            .text("purpose", purpose.to_string());

        let url = format!("{}{}", self.api_host, "/v1/files/upload");

        let mut request = self.client.request(reqwest::Method::POST, &url);
        request = request.header("Authorization", format!("Bearer {}", self.api_key));
        request = request.header("MM-API-Source", "minimax-cli");
        request = request.multipart(form);

        let response = request.send()?;

        #[derive(Deserialize)]
        struct Resp {
            file: Option<FileInfo>,
        }
        let data: Resp = response.json()?;
        data.file
            .ok_or_else(|| MinimaxError::RequestError("No file info in response".to_string()))
    }

    // Voice Clone
    pub fn voice_clone(&self, req: &VoiceCloneRequest) -> Result<VoiceCloneResponse, MinimaxError> {
        self.make_request(reqwest::Method::POST, "/v1/voice_clone", Some(req))
    }

    // Generate Video (returns task_id immediately)
    pub fn generate_video(&self, req: &VideoGenerationRequest) -> Result<String, MinimaxError> {
        #[derive(Deserialize)]
        struct Resp {
            task_id: Option<String>,
        }
        let data: Resp =
            self.make_request(reqwest::Method::POST, "/v1/video_generation", Some(req))?;
        data.task_id
            .ok_or_else(|| MinimaxError::RequestError("No task_id in response".to_string()))
    }

    // Query Video Status
    pub fn query_video(&self, task_id: &str) -> Result<QueryVideoResponse, MinimaxError> {
        let endpoint = format!("/v1/query/video_generation?task_id={}", task_id);
        #[derive(Serialize)]
        struct Empty;
        self.make_request(reqwest::Method::GET, &endpoint, None::<&Empty>)
    }

    // Text to Image
    pub fn text_to_image(&self, req: &ImageGenerationRequest) -> Result<Vec<String>, MinimaxError> {
        let data: ImageGenerationResponse =
            self.make_request(reqwest::Method::POST, "/v1/image_generation", Some(req))?;
        data.data
            .and_then(|d| d.image_urls)
            .ok_or_else(|| MinimaxError::RequestError("No image URLs in response".to_string()))
    }

    // Music Generation
    pub fn music_generation(&self, req: &MusicGenerationRequest) -> Result<String, MinimaxError> {
        let data: MusicGenerationResponse =
            self.make_request(reqwest::Method::POST, "/v1/music_generation", Some(req))?;
        data.data
            .and_then(|d| d.audio)
            .ok_or_else(|| MinimaxError::RequestError("No audio data in response".to_string()))
    }

    // Voice Design
    pub fn voice_design(
        &self,
        req: &VoiceDesignRequest,
    ) -> Result<(String, Option<String>), MinimaxError> {
        let data: VoiceDesignResponse =
            self.make_request(reqwest::Method::POST, "/v1/voice_design", Some(req))?;
        let voice_id = data
            .voice_id
            .ok_or_else(|| MinimaxError::RequestError("No voice_id in response".to_string()))?;
        Ok((voice_id, data.trial_audio))
    }

    // Retrieve File (get download URL)
    pub fn get_file(&self, file_id: &str) -> Result<String, MinimaxError> {
        #[derive(Deserialize)]
        struct Resp {
            file: Option<FileDetail>,
        }
        #[derive(Serialize)]
        struct Empty;
        let data: Resp = self.make_request(
            reqwest::Method::GET,
            &format!("/v1/files/retrieve?file_id={}", file_id),
            None::<&Empty>,
        )?;
        data.file
            .and_then(|f| f.download_url)
            .ok_or_else(|| MinimaxError::RequestError("No download URL in response".to_string()))
    }

    // Download file bytes
    pub fn download_file(&self, url: &str) -> Result<Vec<u8>, MinimaxError> {
        let response = self.client.get(url).send()?;
        Ok(response.bytes()?.to_vec())
    }
}
