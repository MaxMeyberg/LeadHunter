mod apify_call;

#[tokio::main]
async fn main(){

    // TODO: Change this url to a user input
    let linkedin_url = "https://www.linkedin.com/in/williamhgates/".to_string();
    // TODO: Check to see if url is valid

    //TODO: Check emails if none, then show it cant find an email and then tailor a linkedin message


    
    let _json = apify_call::run_actor(&linkedin_url).await;

    match _json{
        Ok(data) => println!("JSON: {}", serde_json::to_string_pretty(&data).unwrap()),
        Err(e) => eprintln!("Uh Oh, error: {}", e),
    }


}