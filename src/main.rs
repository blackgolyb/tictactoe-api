mod core;
mod db;
mod handlers;
mod repositories;
mod services;

use core::{config::load_config, types::AppDep};
use db::{get_connection, init_db};
use handlers::{get_field, update_field};
use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let host = config.host;
    let port = config.port;
    let version_url = format!("/v{}", config.api_version);

    let conn = get_connection(config.db_path);
    init_db(&conn);

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let dep_conn = web::Data::new(AppDep {
        conn: Arc::new(Mutex::new(conn)),
    });

    println!("Starting server on {host}:{port}");

    HttpServer::new(move || {
        App::new().app_data(dep_conn.clone()).service(
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
