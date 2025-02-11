use std::fs;
use std::path::Path;
use std::env;
use crate::errors::AppError;

/// Get the path to the API keys file
fn get_api_keys_path() -> Result<String, AppError> {
    let home_dir = env::var("HOME").map_err(|_| AppError::CredentialsError("Could not determine home directory".to_string()))?;
    let api_keys_dir = format!("{}/Habix-API-Keys", home_dir);
    let api_keys_file = format!("{}/habix-api-keys.txt", api_keys_dir);
    Ok(api_keys_file)
}

/// Save credentials to the API keys file
pub fn save_credentials(user_id: &str, api_token: &str) -> Result<(), AppError> {
    let api_keys_file = get_api_keys_path()?;
    let api_keys_dir = Path::new(&api_keys_file).parent().ok_or_else(|| AppError::CredentialsError("Invalid API keys directory".to_string()))?;

    // Create the directory if it doesn't exist
    if !api_keys_dir.exists() {
        fs::create_dir_all(api_keys_dir).map_err(AppError::IoError)?;
    }

    // Write credentials to the file
    let content = format!("HABITICA_USER_ID={}\nHABITICA_API_TOKEN={}\n", user_id, api_token);
    fs::write(&api_keys_file, content).map_err(AppError::IoError)?;

    println!("✅ Setup complete! Credentials saved at: {}", api_keys_file);
    Ok(())
}

/// Load credentials from the API keys file
pub fn load_credentials() -> Result<(String, String), AppError> {
    let api_keys_file = get_api_keys_path()?;

    // Read the file
    let content = fs::read_to_string(&api_keys_file).map_err(|_| AppError::CredentialsError("API keys file not found".to_string()))?;

    // Parse credentials
    let mut user_id = None;
    let mut api_token = None;

    for line in content.lines() {
        if line.starts_with("HABITICA_USER_ID=") {
            user_id = Some(line.trim_start_matches("HABITICA_USER_ID=").to_string());
        } else if line.starts_with("HABITICA_API_TOKEN=") {
            api_token = Some(line.trim_start_matches("HABITICA_API_TOKEN=").to_string());
        }
    }

    match (user_id, api_token) {
        (Some(user_id), Some(api_token)) => Ok((user_id, api_token)),
        _ => Err(AppError::CredentialsError("Invalid API keys file format".to_string())),
    }
}

/// Delete the API keys file
pub fn clean_credentials() -> Result<(), AppError> {
    let api_keys_file = get_api_keys_path()?;

    if Path::new(&api_keys_file).exists() {
        fs::remove_file(&api_keys_file).map_err(AppError::IoError)?;
        println!("✅ API keys file deleted: {}", api_keys_file);
    } else {
        println!("ℹ️ No API keys file found.");
    }

    Ok(())
}
