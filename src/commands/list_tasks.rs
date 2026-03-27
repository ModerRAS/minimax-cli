use crate::config::Config;
use crate::core::db::Database;

pub async fn run(config: &Config, status: Option<&str>, limit: i32) -> anyhow::Result<()> {
    let db = Database::new(&config.db_path)?;

    let tasks = db.list_tasks(status, limit)?;

    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    println!(
        "{:<40} {:<10} {:<12} Created",
        "Task ID", "Type", "Status"
    );
    println!("{}", "-".repeat(80));

    for task in tasks {
        let status_str = match task.status.as_str() {
            "pending" => "⏳ pending",
            "processing" => "🔄 processing",
            "success" => "✅ success",
            "fail" => "❌ fail",
            _ => &task.status,
        };

        println!(
            "{:<40} {:<10} {:<12} {}",
            &task.task_id[..std::cmp::min(40, task.task_id.len())],
            task.task_type,
            status_str,
            task.created_at
        );
    }

    Ok(())
}
