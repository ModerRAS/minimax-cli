use std::path::PathBuf;
use crate::config::Config;
use crate::core::api_client::MinimaxClient;
use crate::core::models::{AudioSetting, TextToAudioRequest, VoiceSetting};

pub async fn run(
    config: &Config,
    text: &str,
    voice_id: &str,
    speed: f32,
    output_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    let client = MinimaxClient::new(config.api_key.clone(), config.api_host.clone());
    
    let req = TextToAudioRequest {
        model: "speech-2.6-hd".to_string(),
        text: text.to_string(),
        voice_setting: VoiceSetting {
            voice_id: voice_id.to_string(),
            speed,
            vol: 1.0,
            pitch: 0,
            emotion: "happy".to_string(),
        },
        audio_setting: AudioSetting {
            sample_rate: 32000,
            bitrate: 128000,
            format: "mp3".to_string(),
            channel: 1,
        },
        language_boost: Some("auto".to_string()),
        output_format: None,
    };
    
    let audio_hex = client.text_to_audio(&req)?;
    
    let output_path = output_dir.unwrap_or_else(|| config.output_dir.clone());
    std::fs::create_dir_all(&output_path)?;
    
    let audio_bytes = hex::decode(&audio_hex)
        .map_err(|e| anyhow::anyhow!("Failed to decode audio hex: {}", e))?;
    
    let filename = format!("t2a_{}_{}.mp3", 
        &text.chars().take(10).collect::<String>().replace(' ', "_"),
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );
    
    let file_path = output_path.join(&filename);
    std::fs::write(&file_path, audio_bytes)?;
    
    println!("Audio saved to: {}", file_path.display());
    println!("Voice used: {}", voice_id);
    
    Ok(())
}
