use crate::config::Config;
use crate::core::api_client::MinimaxClient;

pub async fn run(config: &Config, voice_type: &str) -> anyhow::Result<()> {
    let client = MinimaxClient::new(config.api_key.clone(), config.api_host.clone());
    
    let result = client.list_voices(voice_type)?;
    
    println!("System Voices:");
    if let Some(voices) = result.system_voice {
        for voice in voices {
            println!("  - {} (ID: {})", voice.voice_name, voice.voice_id);
        }
    }
    
    println!("\nVoice Cloning Voices:");
    if let Some(voices) = result.voice_cloning {
        for voice in voices {
            println!("  - {} (ID: {})", voice.voice_name, voice.voice_id);
        }
    }
    
    Ok(())
}
