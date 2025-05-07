/* ❓ Need help understanding API call? 👉 Click me! 🖱️ 
Before you suffer, This is the Python Apify API documentaion, we wanna mimick this in Rust:

from apify_client import ApifyClient

# Initialize the ApifyClient with your API token
client = ApifyClient("<YOUR_API_TOKEN>")

# Prepare the Actor input
run_input = { "profileUrls": [
        "https://www.linkedin.com/in/williamhgates",
        "http://www.linkedin.com/in/jeannie-wyrick-b4760710a",
    ] }

# Run the Actor and wait for it to finish
run = client.actor("2SyF0bVxmgGr8IVCZ").call(run_input=run_input)

# Fetch and print Actor results from the run's dataset (if there are any)
item = next(client.dataset(run["defaultDatasetId"]).iterate_items())


*/

use reqwest::Client; //Rust library needed to have rust send HTTP requests
use serde_json::{json, Value}; // Allows "let url = json!({"profileUrls": [linkedin_url]});" to work
use dotenv::dotenv;
use std::{env, error};


struct ApifyAPI {
    api_key: String, 
    actor_id: String,
    api_url: String, // URL for apify API
}

impl ApifyAPI {
    // This is a contructor to neatly pack all the data from apify into an easier API call
    fn new() -> Self{
        
        dotenv().ok(); // load up .env file, same as "load_dotenv()" in python
        let api_key: String = std::env::var("APIFY_API_KEY").expect("Missing Apify API key, is it gone?"); // get api key from /.env, panic if no API key found
        let actor_id: String = "2SyF0bVxmgGr8IVCZ".to_string(); // change actor_id to change web scraper we use
        let api_url: String = format!("https://api.apify.com/v2/acts/{}/runs", actor_id);
        
        ApifyAPI { api_key, actor_id, api_url,} // return the default
    }

    /* ❓ Need help understanding? 👉 Click me! 🖱️
    Let's break down:
    Result<serde_json::Value, Box<dyn Error>>

    Result<> is an enum that is Value or Err

    Rust requires all types to have a known size at compile time, so for every error type, we need to say its dyn or the borrow checker is after our cheeks

    AKA: the "Err" is Box<dyn error::Error>

    the std::error is this super fat struct that hurts my head, but there is stuff like std::error::Report, std::error::Error and stuff like that

    Box<> just mean its heap data, ngl I am still having a hard time fullly understnding this concept I am explaining
     */
    async fn post_request(&self, linkedin_url: &str) -> Result<serde_json::Value, Box<dyn error::Error>>{
        // Initialize the HTTP client
        let client = Client::new();
        
        //DONT modify "profileUrls": or else the Apify API wont work
        let json_url: Value = json!({"profileUrls": [linkedin_url]});

        /* ❓ Need help understanding? 👉 Click me! 🖱️

            .post(&self.url) -> RequestBuilder 

                create the HTTP POST request and make it target the Apify URL we want

            .bearer_auth(&self.api_key) -> RequestBuilder 

                Adds a Bearer token (using the API key) to the Authorization header of HTTP request for authentication.
                (AKA: Ass in the holder of the API key)

            .json(&json_url) -> RequestBuilder 

                Add in the JSON payload (AKA: just the linkedin URL)

            .send().await?

                Simply sends and waits for the request, 
                in the meantime, lets keep running on other things in rust while the http request is being sent
        */
        let response = client
            .post(&self.api_url)
            .bearer_auth(&self.api_key)
            .json(&json_url)
            .send()
            .await?;

        if !response.status().is_success() {
            /* ❓ Need help understanding? 👉 Click me! 🖱️
            .into() converts "Failed to fetch run status" (which is an &str) it into a String, 
    
            The String is then automatically boxed into a Box<dyn error::Error>
            
            
            Recall that Result is an enum that returns either:
            - Ok(T): A successful result containing a value of type T (AKA: the JSON data from Apify)
            - Err(E): An error result containing a value of type E (AKA: Box<dyn Error>).
            */
            return Err("Failed to run the Actor".into());
        }
        // Confirms we can move on to run rest of program


        // Parse the JSON Response from API call, Apify should confirm that they got our stuff
        let confirmed: serde_json::Value = response.json().await?;

        Ok(confirmed)

    }

    
}

pub async fn run_actor(profile_url: &str) -> Result<serde_json::Value, Box<dyn error::Error>> {


    // --TODO: Remove the fix test
    // dotenv().ok();
    // let api_key = std::env::var("APIFY_API_KEY").expect("Missing Apify API key");
    // let actor_id = "2SyF0bVxmgGr8IVCZ".to_string();
    // let apify_api = ApifyAPI::new(api_key.clone(), actor_id.clone(), profile_url);
    let apify = ApifyAPI::new();
    // Make the API call to run the Actor (POST)
    let run = apify.post_request(profile_url).await?;

    // Retry mechanism to wait for the actor to finish
    let client = Client::new();

    
    // Retry mechanism to wait for the actor to finish
    let mut retries = 10;
    let mut run_status = run.clone();

    while retries > 0 {
        if run_status["data"]["finishedAt"].as_str().is_some() {
            break;
        } else {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            //(GET)
            
            let run_status_url = format!("https://api.apify.com/v2/acts/{}/runs/{}", apify.actor_id, run["data"]["id"].as_str().unwrap());
            let run_status_response = client
                .get(&run_status_url)
                .bearer_auth(apify.api_key.clone())
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
                .bearer_auth(apify.api_key)
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
