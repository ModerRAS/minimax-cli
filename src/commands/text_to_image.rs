use crate::config::Config;
use crate::core::api_client::MinimaxClient;
use crate::core::models::ImageGenerationRequest;
use std::path::PathBuf;

pub async fn run(
    config: &Config,
    prompt: &str,
    model: &str,
    aspect_ratio: &str,
    n: i32,
    prompt_optimizer: bool,
    output_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    let client = MinimaxClient::new(config.api_key.clone(), config.api_host.clone());

    let req = ImageGenerationRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        aspect_ratio: aspect_ratio.to_string(),
        n,
        prompt_optimizer,
    };

    let image_urls = client.text_to_image(&req)?;

    let output_path = output_dir.unwrap_or_else(|| config.output_dir.clone());
    std::fs::create_dir_all(&output_path)?;

    let mut saved_files = Vec::new();

    for (i, url) in image_urls.iter().enumerate() {
        let filename = format!(
            "image_{}_{}_{}.jpg",
            &prompt
                .chars()
                .take(10)
                .collect::<String>()
                .replace(' ', "_"),
            i,
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        );
        let file_path = output_path.join(&filename);

        let bytes = client.download_file(url)?;
        std::fs::write(&file_path, bytes)?;
        saved_files.push(file_path);
    }

    println!("Generated {} image(s):", saved_files.len());
    for path in &saved_files {
        println!("  - {}", path.display());
    }

    Ok(())
}
