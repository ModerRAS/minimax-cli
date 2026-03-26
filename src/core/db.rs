use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::Mutex;

use super::errors::MinimaxError;
use super::models::StoredTask;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, MinimaxError> {
        // Ensure parent directory exists
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                MinimaxError::DatabaseError(format!("Failed to create directory: {}", e))
            })?;
        }

        let conn = Connection::open(path)
            .map_err(|e| MinimaxError::DatabaseError(format!("Failed to open database: {}", e)))?;

        // Enable WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .map_err(|e| MinimaxError::DatabaseError(format!("Failed to set PRAGMA: {}", e)))?;

        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), MinimaxError> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                task_id         TEXT NOT NULL UNIQUE,
                task_type       TEXT NOT NULL,
                status          TEXT NOT NULL DEFAULT 'pending',
                prompt          TEXT,
                model           TEXT,
                file_id         TEXT,
                download_url    TEXT,
                local_path      TEXT,
                error_msg       TEXT,
                created_at      TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at      TEXT NOT NULL DEFAULT (datetime('now')),
                completed_at    TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_tasks_task_id ON tasks(task_id);
            CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
            CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);
            "#,
        )
        .map_err(|e| MinimaxError::DatabaseError(format!("Failed to create schema: {}", e)))?;
        Ok(())
    }

    pub fn insert_task(
        &self,
        task_id: &str,
        task_type: &str,
        prompt: Option<&str>,
        model: Option<&str>,
    ) -> Result<i64, MinimaxError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO tasks (task_id, task_type, status, prompt, model) VALUES (?1, ?2, 'pending', ?3, ?4)",
            params![task_id, task_type, prompt, model],
        )
        .map_err(|e| MinimaxError::DatabaseError(format!("Failed to insert task: {}", e)))?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_task(&self, task_id: &str) -> Result<Option<StoredTask>, MinimaxError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, task_id, task_type, status, prompt, model, file_id, download_url, local_path, error_msg, created_at, updated_at, completed_at FROM tasks WHERE task_id = ?1")
            .map_err(|e| MinimaxError::DatabaseError(format!("Failed to prepare statement: {}", e)))?;

        let task = stmt
            .query_row(params![task_id], |row| {
                Ok(StoredTask {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    task_type: row.get(2)?,
                    status: row.get(3)?,
                    prompt: row.get(4)?,
                    model: row.get(5)?,
                    file_id: row.get(6)?,
                    download_url: row.get(7)?,
                    local_path: row.get(8)?,
                    error_msg: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                    completed_at: row.get(12)?,
                })
            })
            .optional()
            .map_err(|e| MinimaxError::DatabaseError(format!("Failed to query task: {}", e)))?;

        Ok(task)
    }

    pub fn update_task_status(&self, task_id: &str, status: &str) -> Result<(), MinimaxError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE tasks SET status = ?1, updated_at = datetime('now') WHERE task_id = ?2",
            params![status, task_id],
        )
        .map_err(|e| MinimaxError::DatabaseError(format!("Failed to update task: {}", e)))?;
        Ok(())
    }

    pub fn update_task_success(
        &self,
        task_id: &str,
        file_id: &str,
        download_url: &str,
    ) -> Result<(), MinimaxError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE tasks SET status = 'success', file_id = ?1, download_url = ?2, updated_at = datetime('now'), completed_at = datetime('now') WHERE task_id = ?3",
            params![file_id, download_url, task_id],
        )
        .map_err(|e| MinimaxError::DatabaseError(format!("Failed to update task: {}", e)))?;
        Ok(())
    }

    pub fn update_task_failed(&self, task_id: &str, error_msg: &str) -> Result<(), MinimaxError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE tasks SET status = 'fail', error_msg = ?1, updated_at = datetime('now'), completed_at = datetime('now') WHERE task_id = ?2",
            params![error_msg, task_id],
        )
        .map_err(|e| MinimaxError::DatabaseError(format!("Failed to update task: {}", e)))?;
        Ok(())
    }

    pub fn update_task_local_path(
        &self,
        task_id: &str,
        local_path: &str,
    ) -> Result<(), MinimaxError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE tasks SET local_path = ?1, updated_at = datetime('now') WHERE task_id = ?2",
            params![local_path, task_id],
        )
        .map_err(|e| MinimaxError::DatabaseError(format!("Failed to update task: {}", e)))?;
        Ok(())
    }

    pub fn list_tasks(
        &self,
        status: Option<&str>,
        limit: i32,
    ) -> Result<Vec<StoredTask>, MinimaxError> {
        let conn = self.conn.lock().unwrap();

        let map_row = |row: &rusqlite::Row| -> Result<StoredTask, rusqlite::Error> {
            Ok(StoredTask {
                id: row.get(0)?,
                task_id: row.get(1)?,
                task_type: row.get(2)?,
                status: row.get(3)?,
                prompt: row.get(4)?,
                model: row.get(5)?,
                file_id: row.get(6)?,
                download_url: row.get(7)?,
                local_path: row.get(8)?,
                error_msg: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
                completed_at: row.get(12)?,
            })
        };

        let tasks: Vec<StoredTask> = if let Some(status) = status {
            let mut stmt = conn.prepare(
                "SELECT id, task_id, task_type, status, prompt, model, file_id, download_url, local_path, error_msg, created_at, updated_at, completed_at FROM tasks WHERE status = ?1 ORDER BY created_at DESC LIMIT ?2"
            ).map_err(|e| MinimaxError::DatabaseError(format!("Failed to prepare statement: {}", e)))?;
            let result = stmt
                .query_map(params![status, limit], map_row)
                .map_err(|e| MinimaxError::DatabaseError(format!("Failed to query tasks: {}", e)))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| {
                    MinimaxError::DatabaseError(format!("Failed to collect tasks: {}", e))
                })?;
            result
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, task_id, task_type, status, prompt, model, file_id, download_url, local_path, error_msg, created_at, updated_at, completed_at FROM tasks ORDER BY created_at DESC LIMIT ?1"
            ).map_err(|e| MinimaxError::DatabaseError(format!("Failed to prepare statement: {}", e)))?;
            let result = stmt
                .query_map(params![limit], map_row)
                .map_err(|e| MinimaxError::DatabaseError(format!("Failed to query tasks: {}", e)))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| {
                    MinimaxError::DatabaseError(format!("Failed to collect tasks: {}", e))
                })?;
            result
        };

        Ok(tasks)
    }

    pub fn delete_task(&self, task_id: &str) -> Result<(), MinimaxError> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM tasks WHERE task_id = ?1", params![task_id])
            .map_err(|e| MinimaxError::DatabaseError(format!("Failed to delete task: {}", e)))?;
        Ok(())
    }
}
