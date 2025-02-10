use std::fs;
use std::path::Path;
use std::env;
use crate::errors::AppError;

/// Load environment variables from .env file
pub fn load_env() -> Result<(), AppError> {
    dotenvy::dotenv().map_err(AppError::EnvError)?;
    Ok(())
}

/// Get user ID and API token from environment variables
pub fn get_credentials() -> Result<(String, String), AppError> {
    let user_id = env::var("HABITICA_USER_ID")
        .map_err(|_| AppError::CredentialsError("User ID not found in .env".to_string()))?;
    let api_token = env::var("HABITICA_API_TOKEN")
        .map_err(|_| AppError::CredentialsError("API token not found in .env".to_string()))?;
    Ok((user_id, api_token))
}

/// Save credentials to the .env file
pub fn save_credentials(user_id: &str, api_token: &str) -> Result<(), AppError> {
    let env_file = Path::new(".env");

    // Create the .env file if it doesn't exist
    if !env_file.exists() {
        fs::write(env_file, "").map_err(AppError::IoError)?;
    }

    // Write credentials to the .env file
    let content = format!(
        "HABITICA_USER_ID={}\nHABITICA_API_TOKEN={}\n",
        user_id, api_token
    );
    fs::write(env_file, content).map_err(AppError::IoError)?;

    Ok(())
}

/// Delete the .env file
pub fn clean_credentials() -> Result<(), AppError> {
    let env_file = Path::new(".env");

    if env_file.exists() {
        fs::remove_file(env_file).map_err(AppError::IoError)?;
        println!("✅ .env file deleted.");
    } else {
        println!("ℹ️ No .env file found.");
    }

    Ok(())
}
