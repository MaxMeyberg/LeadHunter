use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use anyhow::{Result, Context};

/// Extract text content from a .docx file
pub fn extract_docx_text(file_path: &str) -> Result<String> {
    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;
    
    let mut archive = ZipArchive::new(file)
        .context("Failed to read .docx file as zip archive")?;
    
    // Get the main document content from the .docx
    let mut doc_file = archive.by_name("word/document.xml")
        .context("Failed to find document.xml in .docx file")?;
    
    let mut xml_contents = String::new();
    doc_file.read_to_string(&mut xml_contents)
        .context("Failed to read document.xml contents")?;
    
    // Extract text from XML
    let text = extract_text_from_xml(&xml_contents);
    Ok(text)
}

/// Parse XML and extract text content from Word document XML
fn extract_text_from_xml(xml: &str) -> String {
    use regex::Regex;
    
    // Extract text between <w:t> tags (Word text elements)
    let text_regex = Regex::new(r"<w:t[^>]*>(.*?)</w:t>").unwrap();
    
    let mut extracted_text = Vec::new();
    
    for cap in text_regex.captures_iter(xml) {
        if let Some(text) = cap.get(1) {
            extracted_text.push(text.as_str());
        }
    }
    
    // Join all text with spaces and clean up
    extracted_text
        .join(" ")
        .trim()
        .to_string()
}

/// Combine base system prompt with .docx content
pub async fn build_enhanced_system_prompt(
    base_prompt_path: &str, 
    docx_path: &str
) -> Result<String> {
    // Read base system prompt
    let base_prompt = tokio::fs::read_to_string(base_prompt_path)
        .await
        .with_context(|| format!("Failed to read base prompt: {}", base_prompt_path))?;
    
    // Extract .docx content
    let docx_content = extract_docx_text(docx_path)
        .with_context(|| format!("Failed to extract content from: {}", docx_path))?;
    
    // Combine them
    let enhanced_prompt = format!(
        "{}\n\n--- Additional Guidance ---\n{}", 
        base_prompt, 
        docx_content
    );
    
    Ok(enhanced_prompt)
}

#[cfg(test)]
mod tests {


    #[tokio::test]
    async fn test_build_enhanced_system_prompt() {
        // This would need actual test files to run
        // You can create small test files for this
    }
}