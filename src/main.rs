mod core;
mod handlers;
mod services;
mod repositories;

use core::config::load_config;
use handlers::{get_field, update_field};

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let host = config.host;
    let port = config.port;
    let version_url = format!("/v{}", config.api_version);

    println!("Starting server on {host}:{port}");

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api").service(
                web::scope(&version_url)
                    .service(get_field)
                    .service(update_field),
            ),
        )
    })
    .bind((host, port))?
    .run()
    .await
}
