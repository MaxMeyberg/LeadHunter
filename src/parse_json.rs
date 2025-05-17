use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;

/// A struct holding just the “crucial” fields.
#[derive(Debug, Deserialize)]
pub struct ProfileInfo {
    pub full_name: Option<String>,
    pub headline: Option<String>,
    pub location: Option<String>,
    pub about: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

impl ProfileInfo {
    /// Format the profile info into a text snippet for LLM prompts.
    pub fn to_prompt(&self) -> String {
        let mut out = String::new();
        if let Some(name) = &self.full_name {
            out.push_str(&format!("Name: {}\n", name));
        }
        if let Some(headline) = &self.headline {
            out.push_str(&format!("Headline: {}\n", headline));
        }
        if let Some(loc) = &self.location {
            out.push_str(&format!("Location: {}\n", loc));
        }
        if let Some(about) = &self.about {
            out.push_str(&format!("About: {}\n", about));
        }
        if let Some(email) = &self.email {
            out.push_str(&format!("Email: {}\n", email));
        }
        if let Some(phone) = &self.phone_number {
            out.push_str(&format!("Phone: {}\n", phone));
        }
        out
    }
}

/// Parse a JSON `Value` (e.g. your `apify_data`) into `ProfileInfo`
pub fn from_value(v: &Value) -> Result<ProfileInfo> {
    serde_json::from_value(v.clone())
        .context("Failed to deserialize ProfileInfo from Value")
}
