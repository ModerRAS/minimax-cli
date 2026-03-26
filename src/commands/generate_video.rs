use crate::config::Config;
use crate::core::api_client::MinimaxClient;
use crate::core::db::Database;
use crate::core::models::VideoGenerationRequest;

pub async fn run(
    config: &Config,
    prompt: &str,
    model: &str,
    first_frame_image: Option<String>,
    duration: Option<i32>,
    resolution: Option<String>,
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
    
    // Store task in database
    db.insert_task(&task_id, "video", Some(prompt), Some(model))?;
    
    println!("Video generation task submitted successfully!");
    println!("Task ID: {}", task_id);
    println!("Model: {}", model);
    println!("\nUse the following commands to check status:");
    println!("  minimax query-task --task-id {}", task_id);
    println!("  minimax download-task --task-id {} --output-dir ./downloads", task_id);
    println!("  minimax list-tasks");
    
    Ok(())
}
