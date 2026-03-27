use crate::config::Config;
use crate::core::api_client::MinimaxClient;
use crate::core::db::Database;
use std::path::PathBuf;

pub async fn run(
    config: &Config,
    task_id: &str,
    output_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    let client = MinimaxClient::new(config.api_key.clone(), config.api_host.clone());
    let db = Database::new(&config.db_path)?;

    // Get task from database
    let task = db
        .get_task(task_id)?
        .ok_or_else(|| anyhow::anyhow!("Task not found: {}", task_id))?;

    if task.status != "success" {
        return Err(anyhow::anyhow!(
            "Task is not completed yet. Status: {}",
            task.status
        ));
    }

    let download_url = task
        .download_url
        .ok_or_else(|| anyhow::anyhow!("No download URL available"))?;

    let output_path = output_dir.unwrap_or_else(|| config.output_dir.clone());
    std::fs::create_dir_all(&output_path)?;

    let bytes = client.download_file(&download_url)?;

    let extension = match task.task_type.as_str() {
        "video" => "mp4",
        "image" => "jpg",
        "music" => "mp3",
        _ => "bin",
    };

    let filename = format!("{}_{}.{}", task.task_type, task_id, extension);
    let file_path = output_path.join(&filename);

    std::fs::write(&file_path, bytes)?;

    // Update database with local path
    db.update_task_local_path(task_id, file_path.to_str().unwrap_or(""))?;

    println!("Downloaded to: {}", file_path.display());

    Ok(())
}
