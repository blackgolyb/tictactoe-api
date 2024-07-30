use crate::core::types::{FieldId, FieldStatus};
use crate::services::traits::VisualizeGame;
use crate::services::GameVisualizeService;

use actix_web::{get, web, HttpResponse, Responder};

#[get("/{room}/get_field/{field_id}")]
async fn get_field(param: web::Path<(String, FieldId)>) -> impl Responder {
    let room = param.0.to_string();
    let field_id = param.1;

    //
    let assets = "./assets";
    let visualizer = GameVisualizeService::new(assets);
    let winner_seq = Some(vec![1, 4, 7]);

    let a = visualizer.get_field_image(field_id, FieldStatus::X, winner_seq);

    let image_content = web::block(move || a).await.unwrap();

    HttpResponse::Ok()
        .content_type("image/png")
        .body(image_content)
}

#[get("/{room}/update_field/{field_id}")]
async fn update_field(param: web::Path<(String, FieldId)>) -> impl Responder {
    let room = &param.0;
    let field_id = param.1;

    format!("{} {}", room, field_id)
}
