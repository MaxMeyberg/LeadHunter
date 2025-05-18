// src/pinecone_service.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;
use std::collections::HashMap; // For generic metadata

#[derive(Serialize, Debug)]
pub struct PineconeVector {
    pub id: String,
    pub values: Vec<f32>,
    pub metadata: Option<HashMap<String, Value>>, // Using HashMap for flexible metadata
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")] // To match Pinecone's expected field names like "vectors"
struct UpsertRequestPayload {
    vectors: Vec<PineconeVector>,
    namespace: Option<String>,
}

#[derive(Deserialize, Debug)]
struct UpsertResponse {
    #[serde(rename = "upsertedCount")]
    upserted_count: u32,
    // Add other fields if needed, e.g., error details
}

#[derive(Debug)]
pub enum PineconeError {
    HttpRequestError(reqwest::Error),
    ApiError { status: u16, message: String },
    SerdeError(serde_json::Error),
}

impl From<reqwest::Error> for PineconeError {
    fn from(err: reqwest::Error) -> Self {
        PineconeError::HttpRequestError(err)
    }
}

impl From<serde_json::Error> for PineconeError {
    fn from(err: serde_json::Error) -> Self {
        PineconeError::SerdeError(err)
    }
}

/// Upserts vectors to a Pinecone index.
///
/// # Arguments
/// * `client` - An instance of `reqwest::Client`.
/// * `index_endpoint` - The full Pinecone index endpoint URL for upserting (e.g., "https://your-index-name-gcp-starter.svc.gcp-starter.pinecone.io/vectors/upsert").
/// * `api_key` - Your Pinecone API key.
/// * `vectors` - A vector of `PineconeVector` to upsert.
/// * `namespace` - Optional namespace to upsert vectors into.
///
/// # Returns
/// The number of vectors successfully upserted, or a `PineconeError`.
pub async fn upsert_vectors(
    client: &Client,
    index_endpoint: &str,
    api_key: &str,
    vectors_to_upsert: Vec<PineconeVector>,
    namespace: Option<String>,
) -> Result<u32, PineconeError> {
    if vectors_to_upsert.is_empty() {
        return Ok(0); // Nothing to upsert
    }

    let payload = UpsertRequestPayload {
        vectors: vectors_to_upsert,
        namespace,
    };

    let response = client
        .post(index_endpoint)
        .header("Api-Key", api_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        let upsert_response = response.json::<UpsertResponse>().await?;
        Ok(upsert_response.upserted_count)
    } else {
        let status = response.status().as_u16();
        let error_message = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
        Err(PineconeError::ApiError {
            status,
            message: format!("Pinecone API Error ({}): {}", status, error_message),
        })
    }
}