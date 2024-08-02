mod core;
mod db;
mod handlers;
mod repositories;
mod services;

use core::{config::load_config, types::AppDependency};
use db::{get_connection, init_db};
use env_logger::Env;
use std::sync::{Arc, Mutex};

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let host = config.host;
    let port = config.port;
    let version_url = format!("/v{}", config.api_version);

    let conn = get_connection(config.db_path);
    init_db(&conn);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let state = web::Data::new(AppDependency {
        conn: Arc::new(Mutex::new(conn)),
    });

    println!("Starting server on {host}:{port}");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(
                web::scope("/api").service(
                    web::scope(&version_url)
                        .service(handlers::get_field)
                        .service(handlers::get_current_user)
                        .service(handlers::update_field)
                ),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
