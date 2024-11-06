use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::context::ProcessContext;
use crate::factory::ProcessorFactory;
use crate::models::InputData;

mod context;
mod factory;
mod image;
mod models;

#[derive(Serialize)]
struct CommonErrorResponse {
    error: String,
}

#[derive(Deserialize)]
struct GenerateImageRequest {
    prompt: String,
    model: String,
    steps: Option<u32>,
}

async fn handle_generate_image(req: web::Json<GenerateImageRequest>) -> impl Responder {
    let input = InputData::new(req.prompt.clone(), req.steps);
    match input.validate() {
        Ok(_) => {
            let strategy = ProcessorFactory::create(&req.model);
            match strategy {
                Some(strategy) => {
                    let context = ProcessContext::new(strategy);
                    match context.execute(input).await {
                        Ok(output) => HttpResponse::Ok()
                            .content_type("image/png")
                            .body(output.result),
                        Err(e) => HttpResponse::InternalServerError().json(CommonErrorResponse {
                            error: e.error_message,
                        }),
                    }
                }
                None => HttpResponse::BadRequest().json(CommonErrorResponse {
                    error: "Invalid model name".to_string(),
                }),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(CommonErrorResponse {
            error: e.to_string(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let bind_address = std::env::var("SERVER_BIND_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());

    println!("Starting server at: {}", bind_address);

    HttpServer::new(|| {
        App::new()
            .route("/generate_image", web::post().to(handle_generate_image))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Ok") }),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
