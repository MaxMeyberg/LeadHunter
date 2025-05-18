use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Helper struct for deserializing skill entries
#[derive(Debug, Serialize, Deserialize)]
struct SkillEntry {
    title: Option<String>,
}

/// A struct holding only the key fields for LinkedIn outreach.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileInfo {
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "jobTitle")]
    pub job_title: Option<String>,
    #[serde(rename = "companyName")] // Changed from "company"
    pub company_name: Option<String>,
    #[serde(rename = "addressWithoutCountry")] // Changed from "location"
    pub location: Option<String>,
    #[serde(rename = "headline")]
    pub headline: Option<String>,
    #[serde(rename = "about")]
    pub about: Option<String>,
    #[serde(rename = "skills")] // Changed from "topSkills" and type changed
    pub skills: Option<Vec<SkillEntry>>,
    #[serde(rename = "recentNews")]
    pub recent_news: Option<String>, // This will be None if not present in JSON
    // Changed type to Value to capture raw JSON for interests
    pub interests: Option<Value>,
    // Add the email field. Assumes the Apify JSON might have an "email" field.
    // If the field name in the JSON is different, use #[serde(rename = "actualFieldName")]
    pub email: Option<String>,
}

/// Intermediate struct for formatting the prompt
#[derive(Debug, Serialize)]
struct ProfileForPrompt<'a> {
    first_name: &'a Option<String>,
    last_name: &'a Option<String>,
    job_title: &'a Option<String>,
    company_name: &'a Option<String>,
    location: &'a Option<String>,
    headline: &'a Option<String>,
    about: &'a Option<String>,
    skills: Option<Vec<String>>, // Flattened skills
    recent_news: &'a Option<String>,
    interests: &'a Option<Value>, // Keep as Value for the prompt for now
    email: &'a Option<String>,
}

impl ProfileInfo {
    /// Format the profile info into a JSON snippet for LLM prompts.
    pub fn to_prompt(&self) -> String {
        let skills_for_prompt: Option<Vec<String>> = self.skills.as_ref().map(|skill_entries| {
            skill_entries.iter().filter_map(|se| se.title.clone()).collect()
        });

        let prompt_data = ProfileForPrompt {
            first_name: &self.first_name,
            last_name: &self.last_name,
            job_title: &self.job_title,
            company_name: &self.company_name,
            location: &self.location,
            headline: &self.headline,
            about: &self.about,
            skills: skills_for_prompt,
            recent_news: &self.recent_news,
            interests: &self.interests, // Still Value, can be refined later
            email: &self.email,
        };

        // Serialize to pretty JSON, or fallback to empty object on error
        serde_json::to_string_pretty(&prompt_data).unwrap_or_else(|_| "{\"error\": \"Failed to serialize prompt data\"}".to_string())
    }
}

/// Parse a JSON `Value` (e.g. your LinkedIn scraper output) into `ProfileInfo`
pub fn from_value(v: &Value) -> Result<ProfileInfo> {
    serde_json::from_value(v.clone())
        .context("Failed to deserialize ProfileInfo from Value")
}
