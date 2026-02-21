use std::path::PathBuf;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use tic_tac_toe_api::{
    core::config::load_config,
    game::{
        infrastructure::{
            db::{establish_connection, migrator::run_migrations},
            di::DIContainer,
        },
        presentation::api::{
            handlers::{create_game, get_current_player, get_field, main_page, update_field},
            state::AppState,
        },
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration from environment
    let config = load_config();
    let host = config.host.clone();
    let port = config.port;
    let version_url = format!("/v{}", config.api_version);

    println!("Initializing application...");

    // Establish database connection
    let db_connection = establish_connection(config.db_path.clone());
    println!("Database connection established at: {}", config.db_path);

    // Run database migrations
    run_migrations(&db_connection);
    println!("Database migrations completed");

    // Set up assets directory
    let assets_dir = PathBuf::from(config.assets.clone());
    println!("Assets directory: {}", assets_dir.display());

    // Create DI container with all dependencies
    let container = DIContainer::new(db_connection, assets_dir);
    println!("Dependency injection container initialized");

    // Create application state
    let app_state = web::Data::new(AppState::new(container));

    println!(
        "API available at: http://{}:{}/api{}",
        host, port, version_url
    );

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            // Serve main page
            .service(main_page)
            // API routes
            .service(
                web::scope("/api").service(
                    web::scope(&version_url)
                        .service(create_game)
                        .service(get_current_player)
                        .service(get_field)
                        .service(update_field),
                ),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
