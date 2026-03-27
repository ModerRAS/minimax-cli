use crate::config::Config;
use crate::core::api_client::MinimaxClient;
use crate::core::models::VoiceDesignRequest;
use std::path::PathBuf;

pub async fn run(
    config: &Config,
    prompt: &str,
    preview_text: &str,
    voice_id: Option<&str>,
    output_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    let client = MinimaxClient::new(config.api_key.clone(), config.api_host.clone());

    let req = VoiceDesignRequest {
        prompt: prompt.to_string(),
        preview_text: preview_text.to_string(),
        voice_id: voice_id.map(|s| s.to_string()),
    };

    let (generated_voice_id, trial_audio) = client.voice_design(&req)?;

    println!("Voice designed successfully!");
    println!("Voice ID: {}", generated_voice_id);

    if let Some(audio_hex) = trial_audio {
        let output_path = output_dir.unwrap_or_else(|| config.output_dir.clone());
        std::fs::create_dir_all(&output_path)?;

        let audio_bytes = hex::decode(&audio_hex)
            .map_err(|e| anyhow::anyhow!("Failed to decode audio hex: {}", e))?;

        let filename = format!(
            "voice_design_{}_{}.mp3",
            &preview_text
                .chars()
                .take(10)
                .collect::<String>()
                .replace(' ', "_"),
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        );
        let file_path = output_path.join(&filename);
        std::fs::write(&file_path, audio_bytes)?;

        println!("Trial audio saved to: {}", file_path.display());
    }

    Ok(())
}
