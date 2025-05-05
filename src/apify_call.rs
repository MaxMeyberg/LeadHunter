use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn run_actor(api_token: &str, actor_id: &str, profile_url: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    // Initialize the HTTP client
    let client = Client::new();

    // Check if the API token is provided
    if api_token.is_empty() {
        return Err("No API token found".into());
    }

    // Prepare the Actor input with a single URL
    let run_input = json!({
        "profileUrls": [profile_url]
    });

    // Make the API call to run the Actor
    let run_url = format!("https://api.apify.com/v2/acts/{}/runs", actor_id);
    let response = client
        .post(&run_url)
        .bearer_auth(api_token)
        .json(&run_input)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err("Failed to run the Actor".into());
    }

    let run: serde_json::Value = response.json().await?;

    // Retry mechanism to wait for the actor to finish
    let mut retries = 10;
    let mut run_status = run.clone();

    while retries > 0 {
        if run_status["data"]["finishedAt"].as_str().is_some() {
            break;
        } else {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let run_status_url = format!("https://api.apify.com/v2/acts/{}/runs/{}", actor_id, run["data"]["id"].as_str().unwrap());
            let run_status_response = client
                .get(&run_status_url)
                .bearer_auth(api_token)
                .send()
                .await?;

            if run_status_response.status().is_success() {
                run_status = run_status_response.json().await?;
            } else {
                return Err("Failed to fetch run status".into());
            }
        }

        retries -= 1;
    }

    if retries == 0 {
        return Err("Actor did not finish in time".into());
    }

    // Use match instead of if-else for handling dataset_id
    match run["data"].get("defaultDatasetId").and_then(|v| v.as_str()) {
        Some(dataset_id) => {
            let dataset_url = format!("https://api.apify.com/v2/datasets/{}/items", dataset_id);
            let dataset_response = client
                .get(&dataset_url)
                .bearer_auth(api_token)
                .send()
                .await?;

            if dataset_response.status().is_success() {
                let items: Vec<serde_json::Value> = dataset_response.json().await?;
                if let Some(first_item) = items.into_iter().next() {
                    let full_name = first_item.get("fullName").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let headline = first_item.get("headline").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let email = first_item.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let about = first_item.get("about").and_then(|v| v.as_str()).unwrap_or("").to_string();

                    let result = json!({
                        "fullName": full_name,
                        "headline": headline,
                        "email": email,
                        "about": about
                    });

                    return Ok(result);
                } else {
                    return Err("Dataset is empty".into());
                }
            } else {
                return Err("Failed to fetch dataset".into());
            }
        }
        None => return Err("Dataset ID not found".into()),
    }
}
