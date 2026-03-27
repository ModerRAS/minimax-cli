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

    let task = db
        .get_task(task_id)?
        .ok_or_else(|| anyhow::anyhow!("Task not found: {}", task_id))?;

    println!("Task ID: {}", task.task_id);
    println!("Type: {}", task.task_type);
    println!("Status: {}", task.status);
    println!("Created: {}", task.created_at);

    if task.status == "pending" || task.status == "processing" {
        let response = client.query_video(task_id)?;

        match response.status.as_str() {
            "Pending" | "pending" | "Processing" | "Preparing" | "Queueing" | "InQueue" => {
                println!("\nTask is still processing (status: {})...", response.status);
                println!(
                    "Check again later with: minimax query-task --task-id {}",
                    task_id
                );
            }
            "Success" => {
                let file_id = response.get_file_id().unwrap_or_default();
                
                let download_url = if let Some(url) = &response.download_url {
                    url.clone()
                } else if !file_id.is_empty() {
                    match client.get_file(&file_id) {
                        Ok(url) => url,
                        Err(e) => {
                            eprintln!("Warning: Could not get download URL: {}", e);
                            String::new()
                        }
                    }
                } else {
                    String::new()
                };

                db.update_task_success(task_id, &file_id, &download_url)?;

                println!("\n✅ Task completed!");
                println!("File ID: {}", file_id);
                if !download_url.is_empty() {
                    println!("Download URL: {}", download_url);
                } else {
                    println!("Download URL: (not available)");
                }

                if let Some(ref dir) = output_dir {
                    println!("\nAuto-downloading to: {}", dir.display());
                    let bytes = client.download_file(&download_url)?;
                    std::fs::create_dir_all(dir)?;
                    let filename = format!("{}.mp4", task_id);
                    let file_path = dir.join(&filename);
                    std::fs::write(&file_path, bytes)?;
                    println!("Saved to: {}", file_path.display());
                } else {
                    println!("\nDownload with: minimax download-task --task-id {} --output-dir ./downloads", task_id);
                }
            }
            "Fail" => {
                db.update_task_failed(task_id, "Video generation failed")?;
                println!("\n❌ Task failed!");
            }
            _ => {
                println!("\nUnknown status: {}", response.status);
            }
        }
    } else if task.status == "success" {
        println!("\n✅ Task already completed!");
        if let Some(url) = &task.download_url {
            println!("Download URL: {}", url);

            if let Some(ref dir) = output_dir {
                println!("\nAuto-downloading to: {}", dir.display());
                let bytes = client.download_file(url)?;
                std::fs::create_dir_all(dir)?;
                let filename = format!("{}.mp4", task_id);
                let file_path = dir.join(&filename);
                std::fs::write(&file_path, bytes)?;
                println!("Saved to: {}", file_path.display());
            } else {
                println!(
                    "\nDownload with: minimax download-task --task-id {} --output-dir ./downloads",
                    task_id
                );
            }
        }
    } else if task.status == "fail" {
        println!("\n❌ Task failed!");
        if let Some(err) = &task.error_msg {
            println!("Error: {}", err);
        }
    }

    Ok(())
}
