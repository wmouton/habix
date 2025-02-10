use reqwest::blocking::Client;
use serde::Deserialize;
use crate::errors::AppError;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Vec<Task>,
}

#[derive(Deserialize, Debug)]
struct Task {
    text: String,          // Task description
    r#type: String,        // Task type: "daily", "todo", "habit", "reward"
    completed: Option<bool>, // Only present for "todos" and "dailies"
}

/// Fetch incomplete dailies and todos from Habitica API
pub fn fetch_tasks(user_id: &str, api_token: &str) -> Result<Vec<String>, AppError> {
    let client = Client::new();
    let response = client
        .get("https://habitica.com/api/v3/tasks/user")
        .header("x-api-user", user_id)
        .header("x-api-key", api_token)
        .send()?;

    if !response.status().is_success() {
        return Err(AppError::FetchTasksError(format!(
            "API returned status: {}",
            response.status()
        )));
    }

    let api_response: ApiResponse = response.json()?;

    // Filter tasks to include only incomplete dailies and todos
    let incomplete_tasks: Vec<String> = api_response
        .data
        .into_iter()
        .filter(|task| {
            // Include only "todos" and "dailies" that are not completed
            (task.r#type == "daily" || task.r#type == "todo") && task.completed != Some(true)
        })
        .map(|task| task.text)
        .collect();

    Ok(incomplete_tasks)
}

/// Filter pending tasks (no changes needed here)
pub fn get_pending_tasks(tasks: Vec<String>) -> Vec<String> {
    tasks.into_iter().filter(|task| !task.is_empty()).collect()
}
