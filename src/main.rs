mod apify_call;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    // TODO: Change this url to a user input
    let linkedin_url = "https://www.linkedin.com/in/nevingeorge4/".to_string();
    // TODO: Check to see if url is valid
    //TODO: Check emails if none, then show it cant find an email and then tailor a linkedin message


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
    let _json = apify_call::run_actor(&linkedin_url).await?;

    println!("JSON: {}", serde_json::to_string_pretty(&_json)?);
    Ok(())


}