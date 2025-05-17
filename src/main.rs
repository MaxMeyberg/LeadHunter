mod apify_call;
mod lmstudio;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde::{Deserialize, Serialize};
use apify_call::run_actor;
use lmstudio::{generate_email, improve_email};

#[derive(Deserialize)]
struct ScrapeLinkedInRequest {
    url: String,
    prompt: String,
    #[serde(default = "default_skip_apify")]
    skip_apify: bool,
}

fn default_skip_apify() -> bool {
    true  // Set default to true
}

#[derive(Serialize)]
struct ScrapeLinkedInResponse {
    email_address: String,
    email_body: String,
    analysis_rationale: Vec<String>,
}

async fn scrape_linkedin(
    payload: web::Json<ScrapeLinkedInRequest>
) -> impl Responder {
    if payload.url.trim().is_empty() {
        return HttpResponse::BadRequest().body("Missing URL in request");
    }

    // Use dummy data if skip_apify is true, otherwise call the Apify API
    let ws_info = if payload.skip_apify {
        // Create a minimal mock response that matches what the email generator expects
        serde_json::json!({ 
            "data": { 
                "profile": { 
                    "fullName": "Test User",
                    "headline": "Test Headline",
                    "summary": "Test Summary"
                },
                "experiences": [],
                "education": []
            }
        })
    } else {
        match run_actor(&payload.url).await {
            Ok(data) => data,
            Err(err) => {
                log::error!("LinkedIn scrape failed: {}", err);
                return HttpResponse::InternalServerError().body("Error scraping LinkedIn profile");
            }
        }
    };

    match generate_email(ws_info, &payload.prompt).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => {
            log::error!("Email generation failed: {}", err);
            HttpResponse::InternalServerError().body("Error generating email")
        }
    }
}

#[derive(Deserialize)]
struct ImproveEmailRequest {
    email: String,
    prompt: String,
    recipient_name: Option<String>,
}

#[derive(Serialize)]
struct ImproveEmailResponse {
    improved_email: String,
    improvement_rationale: Vec<String>,
}

async fn improve_email_endpoint(
    payload: web::Json<ImproveEmailRequest>
) -> impl Responder {
    if payload.email.trim().is_empty() || payload.prompt.trim().is_empty() {
        return HttpResponse::BadRequest().body("Missing email or prompt in request");
    }

    match improve_email(&payload.email, &payload.prompt, payload.recipient_name.clone()).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(err) => {
            log::error!("Email improvement failed: {}", err);
            HttpResponse::InternalServerError().body("Error improving email")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/scrape-linkedin", web::post().to(scrape_linkedin))
            .route("/improve-email", web::post().to(improve_email_endpoint))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
