use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A struct holding only the key fields for LinkedIn outreach.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInfo {
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "jobTitle")]
    pub job_title: Option<String>,
    #[serde(rename = "company")]
    pub company: Option<String>,
    #[serde(rename = "location")]
    pub location: Option<String>,
    #[serde(rename = "headline")]
    pub headline: Option<String>,
    #[serde(rename = "about")]
    pub about: Option<String>,
    #[serde(rename = "topSkills")]
    pub top_skills: Option<Vec<String>>,
    #[serde(rename = "recentNews")]
    pub recent_news: Option<String>,
    #[serde(rename = "interests")]
    pub interests: Option<Vec<String>>,
}

impl ProfileInfo {
    /// Format the profile info into a JSON snippet for LLM prompts.
    pub fn to_prompt(&self) -> String {
        // Serialize to pretty JSON, or fallback to empty object on error
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Parse a JSON `Value` (e.g. your LinkedIn scraper output) into `ProfileInfo`
pub fn from_value(v: &Value) -> Result<ProfileInfo> {
    serde_json::from_value(v.clone())
        .context("Failed to deserialize ProfileInfo from Value")
}
