use crate::config::Config;
use crate::core::api_client::MinimaxClient;
use crate::core::db::Database;
use crate::core::models::VideoGenerationRequest;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub async fn run(
    config: &Config,
    prompt: &str,
    model: &str,
    first_frame_image: Option<String>,
    duration: Option<i32>,
    resolution: Option<String>,
    async_mode: bool,
    output_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    let client = MinimaxClient::new(config.api_key.clone(), config.api_host.clone());
    let db = Database::new(&config.db_path)?;

    let req = VideoGenerationRequest {
        model: model.to_string(),
        prompt: prompt.to_string(),
        first_frame_image,
        duration,
        resolution,
    };

    let task_id = client.generate_video(&req)?;

    if async_mode {
        // Async mode: just return task_id immediately
        db.insert_task(&task_id, "video", Some(prompt), Some(model))?;

        println!("Video generation task submitted successfully!");
        println!("Task ID: {}", task_id);
        println!("Model: {}", model);
        println!("\nUse the following commands to check status:");
        println!("  minimax query-task --task-id {}", task_id);
        println!(
            "  minimax download-task --task-id {} --output-dir ./downloads",
            task_id
        );
        println!("  minimax list-tasks");
    } else {
        // Sync mode: wait for completion like MCP
        println!("Video generation task submitted. Waiting for completion...");
        println!("Task ID: {}", task_id);

        let _max_retries = if model == "MiniMax-Hailuo-02" { 60 } else { 30 };
        let retry_interval = 20;

        let file_id = loop {
            thread::sleep(Duration::from_secs(retry_interval));

            let status_response = client.query_video(&task_id)?;
            let status = status_response.status;

            if status == "fail" {
                anyhow::bail!("Video generation failed for task_id: {}", task_id);
            } else if status == "success" {
                if let Some(fid) = status_response.file_id {
                    break fid;
                }
                anyhow::bail!(
                    "Missing file_id in success response for task_id: {}",
                    task_id
                );
            }

            println!("Still processing... (task_id: {})", task_id);
        };

        let download_url = client.get_file(&file_id)?;

        let output_path = output_dir.unwrap_or_else(|| config.output_dir.clone());
        std::fs::create_dir_all(&output_path)?;

        let filename = format!(
            "video_{}_{}.mp4",
            &prompt
                .chars()
                .take(10)
                .collect::<String>()
                .replace(' ', "_"),
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        );
        let file_path = output_path.join(&filename);

        let video_bytes = client.download_file(&download_url)?;
        std::fs::write(&file_path, video_bytes)?;

        println!("Video saved to: {}", file_path.display());
    }

    Ok(())
}
