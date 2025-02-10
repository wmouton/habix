use reqwest::blocking::Client;
use serde::Deserialize;
use crate::errors::AppError;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Vec<Task>,
}

#[derive(Deserialize, Debug)]
struct Task {
    text: String,
}

/// Fetch tasks from Habitica API
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
    let task_texts = api_response
        .data
        .into_iter()
        .map(|task| task.text)
        .collect();

    Ok(task_texts)
}

/// Filter pending tasks
pub fn get_pending_tasks(tasks: Vec<String>) -> Vec<String> {
    tasks.into_iter().filter(|task| !task.is_empty()).collect()
}
