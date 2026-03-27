use crate::config::Config;
use crate::core::api_client::MinimaxClient;
use crate::core::models::VoiceCloneRequest;
use std::path::PathBuf;

pub async fn run(
    config: &Config,
    voice_id: &str,
    file: &str,
    text: Option<&str>,
    is_url: bool,
    output_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    let client = MinimaxClient::new(config.api_key.clone(), config.api_host.clone());

    // Step 1: Upload file
    let file_id = if is_url {
        // For URL, we need to download first then upload (using blocking client)
        let response = reqwest::blocking::get(file)
            .map_err(|e| anyhow::anyhow!("Failed to download file from URL: {}", e))?;
        let bytes = response
            .bytes()
            .map_err(|e| anyhow::anyhow!("Failed to read file bytes: {}", e))?;
        let file_info = client.upload_file(&bytes, "audio.mp3", "voice_clone")?;
        file_info.file_id
    } else {
        let bytes =
            std::fs::read(file).map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))?;
        let file_info = client.upload_file(&bytes, "audio.mp3", "voice_clone")?;
        file_info.file_id
    };

    // Step 2: Clone voice
    // Only set model when text is provided (matching MCP behavior)
    let req = VoiceCloneRequest {
        file_id: file_id.clone(),
        voice_id: voice_id.to_string(),
        text: text.map(|s| s.to_string()),
        model: text.map(|_| "speech-2.6-hd".to_string()),
    };

    let result = client.voice_clone(&req)?;

    // Step 3: Save demo audio if available
    if let Some(demo_url) = result.demo_audio {
        let output_path = output_dir.unwrap_or_else(|| config.output_dir.clone());
        std::fs::create_dir_all(&output_path)?;

        let audio_bytes = client.download_file(&demo_url)?;
        let filename = format!(
            "voice_clone_demo_{}.wav",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        );
        let file_path = output_path.join(&filename);
        std::fs::write(&file_path, audio_bytes)?;

        println!("Voice cloned successfully!");
        println!("Voice ID: {}", voice_id);
        println!("Demo audio saved to: {}", file_path.display());
    } else {
        println!("Voice cloned successfully!");
        println!("Voice ID: {}", voice_id);
    }

    Ok(())
}
