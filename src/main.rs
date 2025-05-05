mod apify_call;
use std::env;
use dotenv::dotenv;
#[tokio::main]
async fn main(){
    // TODO: Change this url to a user input
    let linkedin_url = "https://www.linkedin.com/in/williamhgates/".to_string();
    dotenv().ok(); // check to make sure .env file is activated
    let api_key = env::var("APIFY_API_TOKEN").expect("APIFY_API_TOKEN not set");

    let actor_id = "2SyF0bVxmgGr8IVCZ".to_string();
    let _json = apify_call::run_actor(&api_key, &actor_id, &linkedin_url).await;

    match _json{
        Ok(data) => println!("JSON: {}", serde_json::to_string_pretty(&data).unwrap()),
        Err(e) => eprintln!("Uh Oh, error: {}", e),
    }


}