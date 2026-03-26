use std::path::PathBuf;
use crate::config::Config;
use crate::core::api_client::MinimaxClient;
use crate::core::models::{MusicAudioSetting, MusicGenerationRequest};

pub async fn run(
    config: &Config,
    prompt: &str,
    lyrics: &str,
    format: &str,
    output_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    let client = MinimaxClient::new(config.api_key.clone(), config.api_host.clone());
    
    // Parse lyrics with \n handling
    let lyrics_parsed = lyrics.replace("\\n", "\n");
    
    let req = MusicGenerationRequest {
        model: "music-2.0".to_string(),
        prompt: prompt.to_string(),
        lyrics: lyrics_parsed,
        audio_setting: Some(MusicAudioSetting {
            sample_rate: 32000,
            bitrate: 128000,
            format: format.to_string(),
        }),
        output_format: None,
    };
    
    let audio_hex = client.music_generation(&req)?;
    
    let output_path = output_dir.unwrap_or_else(|| config.output_dir.clone());
    std::fs::create_dir_all(&output_path)?;
    
    let audio_bytes = hex::decode(&audio_hex)
        .map_err(|e| anyhow::anyhow!("Failed to decode audio hex: {}", e))?;
    
    let filename = format!("music_{}_{}.{}", 
        &prompt.chars().take(10).collect::<String>().replace(' ', "_"),
        chrono::Utc::now().format("%Y%m%d_%H%M%S"),
        format
    );
    
    let file_path = output_path.join(&filename);
    std::fs::write(&file_path, audio_bytes)?;
    
    println!("Music saved to: {}", file_path.display());
    
    Ok(())
}
