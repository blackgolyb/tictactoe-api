use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use qstring::QString;
use std::fs;
use std::path::PathBuf;

use crate::core::config::load_config;
use crate::game::{
    application::use_cases::{
        create_game::Input as CreateGameInput, make_move::Input as MakeMoveInput,
        visualize_board_field::Input as VisualizeBoardFieldInput,
        visualize_current_field::Input as VisualizeCurrentFieldInput,
    },
    domain::{models::Player, value_objects::Point},
    infrastructure::di::{
        resolve_create_game_use_case, resolve_make_move_use_case,
        resolve_visualize_board_field_use_case, resolve_visualize_current_field_use_case,
    },
    presentation::api::{
        dtos::{FieldDto, RedirectQuery, RoomDto},
        responses::{
            create_error_response, create_html_response, create_image_response,
            create_redirect_response,
        },
        state::AppState,
    },
};

/// Parse field coordinates from string format "x,y"
fn parse_field_id(field_id: &str) -> Option<Point> {
    let parts: Vec<&str> = field_id.split(',').collect();
    if parts.len() != 2 {
        return None;
    }

    let x = parts[0].parse::<u64>().ok()?;
    let y = parts[1].parse::<u64>().ok()?;

    Some((x, y))
}

/// GET / - Serve the main HTML page
#[get("/")]
pub async fn main_page() -> impl Responder {
    let config = load_config();
    let main_page_path = PathBuf::from(config.assets).join("index.html");

    match fs::read_to_string(main_page_path) {
        Ok(content) => create_html_response(content),
        Err(err) => {
            eprintln!("Error reading index.html: {}", err);
            create_error_response(
                "Failed to load the main page.",
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

#[get("/{room}/get-current-player")]
pub async fn get_current_player(
    state: web::Data<AppState>,
    room: web::Path<String>,
) -> impl Responder {
    let game_name = room.into_inner();

    // Resolve use case from DI container
    let mut use_case = resolve_visualize_current_field_use_case(&state.container);

    // Create input and run use case
    let input = VisualizeCurrentFieldInput { name: game_name };

    match use_case.run(input) {
        Ok(_) => {
            let state = use_case.state();
            match &state.current_player_img {
                Some(image_data) => create_image_response(image_data.clone()),
                None => create_error_response(
                    "Failed to get current player image",
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        }
        Err(_) => create_error_response(
            "Failed to visualize current player",
            actix_web::http::StatusCode::NOT_FOUND,
        ),
    }
}

#[get("/{room}/get-field/{field_id}")]
pub async fn get_field(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (game_name, field_id_str) = path.into_inner();

    // Parse field_id to Point
    let field = match parse_field_id(&field_id_str) {
        Some(point) => point,
        None => {
            return create_error_response(
                "Invalid field identifier",
                actix_web::http::StatusCode::BAD_REQUEST,
            );
        }
    };

    // Resolve use case from DI container
    let mut use_case = resolve_visualize_board_field_use_case(&state.container);

    // Create input and run use case
    let input = VisualizeBoardFieldInput {
        name: game_name,
        field,
    };

    match use_case.run(input) {
        Ok(_) => {
            let state = use_case.state();
            match &state.field_img {
                Some(image_data) => create_image_response(image_data.clone()),
                None => create_error_response(
                    "Failed to get field image",
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        }
        Err(_) => create_error_response(
            "Failed to visualize field",
            actix_web::http::StatusCode::NOT_FOUND,
        ),
    }
}

#[get("/{room}/make-move/{field_id}")]
pub async fn update_field(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> impl Responder {
    let (game_name, field_id_str) = path.into_inner();

    // Parse query string for redirect parameter
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let redirect = qs.get("r").map(String::from);

    // Parse field_id to Point
    let field = match parse_field_id(&field_id_str) {
        Some(point) => point,
        None => {
            return create_error_response(
                "Invalid field identifier",
                actix_web::http::StatusCode::BAD_REQUEST,
            );
        }
    };

    // Resolve use case from DI container
    let mut use_case = resolve_make_move_use_case(&state.container);

    // Create input and run use case
    let input = MakeMoveInput {
        name: game_name,
        field,
    };

    let result = use_case.run(input);

    // Handle redirect or return status
    match redirect {
        Some(redirect_url) => create_redirect_response(redirect_url),
        None => match result {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => create_error_response(
                "Failed to make move",
                actix_web::http::StatusCode::BAD_REQUEST,
            ),
        },
    }
}

#[post("/create-game")]
pub async fn create_game(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    // Parse query parameters
    let query_str = req.query_string();
    let qs = QString::from(query_str);

    // Extract parameters
    let name = match qs.get("name") {
        Some(n) => n.to_string(),
        None => {
            return create_error_response(
                "Missing 'name' parameter",
                actix_web::http::StatusCode::BAD_REQUEST,
            );
        }
    };

    let width = qs
        .get("width")
        .and_then(|w| w.parse::<usize>().ok())
        .unwrap_or(3);

    let height = qs
        .get("height")
        .and_then(|h| h.parse::<usize>().ok())
        .unwrap_or(3);

    let winning_length = qs
        .get("winning_length")
        .and_then(|wl| wl.parse::<u32>().ok())
        .unwrap_or(3);

    let first_player = match qs.get("first_player") {
        Some("X") | Some("x") => Player::X,
        Some("O") | Some("o") => Player::O,
        _ => Player::O,
    };

    // Resolve use case from DI container
    let mut use_case = resolve_create_game_use_case(&state.container);

    // Create input and run use case
    let input = CreateGameInput {
        name: name.clone(),
        game_size: (width, height),
        winning_length,
        first_player,
    };

    match use_case.run(input) {
        Ok(_) => {
            let response_body = format!("{{\"success\": true, \"name\": \"{}\"}}", name);
            HttpResponse::Ok()
                .content_type("application/json")
                .body(response_body)
        }
        Err(error) => {
            let error_message = match error {
                crate::game::application::use_cases::create_game::Error::InvalidRules => {
                    "Invalid game rules"
                }
                crate::game::application::use_cases::create_game::Error::GameAlreadyExists => {
                    "Game with this name already exists"
                }
            };
            create_error_response(error_message, actix_web::http::StatusCode::BAD_REQUEST)
        }
    }
}
