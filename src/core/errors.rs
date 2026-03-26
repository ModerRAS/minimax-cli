use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinimaxError {
    #[error("API authentication failed: {0}")]
    AuthError(String),

    #[error("API request failed: {0}")]
    RequestError(String),

    #[error("API error {code}: {message}")]
    ApiError { code: i32, message: String },

    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Task {task_id} failed: {reason}")]
    TaskFailed { task_id: String, reason: String },

    #[error("File operation failed: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Timeout waiting for task: {0}")]
    TimeoutError(String),
}
