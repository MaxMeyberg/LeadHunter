mod apify_call;
mod llama;
mod parse_json;
use anyhow::{Result};
use parse_json::from_value;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

   
    // TODO: Change this url to a user input
    let linkedin_url = "https://www.linkedin.com/in/nevingeorge4/".to_string();
    // TODO: Check to see if url is valid


    /*❓ Need help understanding API call? 👉 Click me! 🖱️ 

    The "?" are simple shorthand to be:

    match apify_call::run_actor(&linkedin_url).await {
        Ok(data) => {
            println!("JSON: {}", serde_json::to_string_pretty(&data)?);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
    */
    let apify_data = apify_call::run_actor(&linkedin_url).await?;
    
    println!("JSON: {}", serde_json::to_string_pretty(&apify_data)?);
    let info = from_value(&apify_data)?;
    println!("\n→ ProfileInfo: {:#?}", info);

    //TODO: Add in the way to hook it up to llama
    /* 
    
    */
    let system = "You are a helpful assistant.";
    let user = format!(
        "Here is the profile JSON:\n{}\n\nWrite a concise cold email.",
        info.to_prompt()
    );
    let email = llama::generate_from_llama(system, &user).await?;
    println!("\n=== GENERATED EMAIL ===\n{}", email);

    //let test = "mailto:contact@company.com?subject=Job%20Application&body=Hello%2C%0A%0AI%20saw%20your%20job%20posting%20and%20would%20like%20to%20apply.".to_string();
    Ok(())


    //TODO: Check emails if none, then show it cant find an email and then tailor a linkedin message



}