use crate::core::types::{AppState, FieldId};
use crate::repositories::SqliteGameRepository;
use crate::services::GamePlayService;

use actix_web::http::StatusCode;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use qstring::QString;

fn create_image_response(image_content: Vec<u8>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .append_header((
            "Cache-Control",
            "private, max-age=0, no-cache, no-store, must-revalidate",
        ))
        .append_header(("Pragma", "no-cache"))
        .body(image_content)
}

#[get("/{room}/get_current_player")]
async fn get_current_user(dep: AppState, room: web::Path<String>) -> impl Responder {
    let repo = Box::new(SqliteGameRepository::new(dep.conn.clone()));
    let service: GamePlayService = GamePlayService::new(repo);
    let image_content = service.get_current_player_image(room.to_string());

    create_image_response(image_content)
}

#[get("/{room}/get_field/{field_id}")]
async fn get_field(dep: AppState, param: web::Path<(String, FieldId)>) -> impl Responder {
    let room = param.0.to_string();
    let field_id = param.1;

    let repo = Box::new(SqliteGameRepository::new(dep.conn.clone()));

    let service: GamePlayService = GamePlayService::new(repo);
    let image_content = service.get_field_image(room, field_id);

    create_image_response(image_content)
}

#[get("/{room}/update_field/{field_id}")]
async fn update_field(
    dep: AppState,
    param: web::Path<(String, FieldId)>,
    req: HttpRequest,
) -> impl Responder {
    let room = param.0.to_string();
    let field_id = param.1;
    let query_str = req.query_string();
    let qs = QString::from(query_str);
    let redirect = qs.get("r").map(String::from);

    let repo = Box::new(SqliteGameRepository::new(dep.conn.clone()));
    let service: GamePlayService = GamePlayService::new(repo);
    let res = service.make_step(room, field_id);

    match redirect {
        Some(r) => HttpResponse::PermanentRedirect()
            .append_header(("Location", r))
            .append_header((
                "Cache-Control",
                "private, max-age=0, no-cache, no-store, must-revalidate",
            ))
            .append_header(("Pragma", "no-cache"))
            .finish(),
        None => match res {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::build(StatusCode::BAD_REQUEST).finish(),
        },
    }
}
