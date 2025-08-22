mod controllers;
mod handlers;
mod middleware;
mod models;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web, middleware::Logger};
use controllers::resume_controller::handle_resume;
use handlers::user_handlers::{login, register};
use middleware::auth::AuthMiddleware;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info) 
        .init();

    let mongo_url = std::env::var("MONGO_URL")?;
    let origin = std::env::var("ORIGIN")?;
    let port = std::env::var("PORT").unwrap_or_else(|_| "2222".to_string());

    let client = mongodb::Client::with_uri_str(&mongo_url).await?;

    println!("🚀 Server starting on port {}", port); 

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(origin.as_str())
            .allowed_origin("https://cevvy.vercel.app")
            // .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .wrap(cors)
            .app_data(web::Data::new(client.clone()))
            .route("/health", web::get().to(|| async { 
                log::info!("Health check endpoint hit");
                "ok" 
            }))
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(
                web::resource("/resume")
                    .wrap(AuthMiddleware)
                    .route(web::post().to(handle_resume)),
            )
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}