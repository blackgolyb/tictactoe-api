use crate::core::types::{AppDependency, FieldId};
use crate::repositories::SqliteGameRepository;
use crate::services::GamePlayService;

use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/{room}/get_field/{field_id}")]
async fn get_field(dep: AppDependency, param: web::Path<(String, FieldId)>) -> impl Responder {
    let room = param.0.to_string();
    let field_id = param.1;

    let repo = Box::new(SqliteGameRepository::new(dep.conn.clone()));

    let service: GamePlayService = GamePlayService::new(repo);
    let image_content = service.get_field_image(room, field_id);

    // let image_content = web::block(move || image_content).await.unwrap();

    HttpResponse::Ok()
        .content_type("image/png")
        .append_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
        .append_header(("Pragma", "no-cache"))
        .body(image_content)
}

#[get("/{room}/update_field/{field_id}")]
async fn update_field(dep: AppDependency, param: web::Path<(String, FieldId)>) -> impl Responder {
    let room = param.0.to_string();
    let field_id = param.1;

    let repo = Box::new(SqliteGameRepository::new(dep.conn.clone()));

    let service: GamePlayService = GamePlayService::new(repo);
    let res = service.make_step(room, field_id);

    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::build(StatusCode::BAD_REQUEST),
    }
}
