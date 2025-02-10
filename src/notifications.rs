use std::process::Command;
use crate::errors::AppError;

/// Send a desktop notification using `notify-send`
pub fn notify(title: &str, message: &str) -> Result<(), AppError> {
    let status = Command::new("notify-send")
        .arg(title)
        .arg(message)
        .status()
        .map_err(|e| AppError::NotificationError(format!("Failed to send notification: {}", e)))?;

    if !status.success() {
        return Err(AppError::NotificationError("Failed to send notification".to_string()));
    }

    Ok(())
}
