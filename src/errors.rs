use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to save credentials: {0}")]
    CredentialsError(String),

    #[error("Failed to fetch tasks: {0}")]
    FetchTasksError(String),

    #[error("Notification error: {0}")]
    NotificationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Environment variable error: {0}")]
    EnvError(#[from] dotenvy::Error),
}
