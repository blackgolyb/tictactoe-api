use actix_web::{get, web, App, HttpServer, Responder};

trait GameRepositoryInterface {
    fn get_board(&self, room: &str) -> u32;
    fn update_board(&mut self, room: &str, board: u32) -> Result<(), String>;
}

type FieldId = u16;

enum FieldStatus {
    Empty,
    X,
    O,
}
enum GameStatus {
    Going,
    WinnerX,
    WinnerO,
}

type WinnerSequence = [FieldId; 3];

trait GameServiceInterface {
    fn get_field(&self, room: &str, field_id: FieldId) -> FieldStatus;
    fn make_step(&self, room: &str, field_id: FieldId);
    fn check_game(&self, room: &str) -> (GameStatus, Option<WinnerSequence>);
}

struct GameService {
    room_repository: Box<dyn GameRepositoryInterface>,
}

impl GameServiceInterface for GameService {
    fn get_field(&self, room: &str, field_id: FieldId) -> FieldStatus {
        unimplemented!()
    }
    fn make_step(&self, room: &str, field_id: FieldId) {
        unimplemented!()
    }
    fn check_game(&self, room: &str) -> (GameStatus, Option<WinnerSequence>) {
        unimplemented!()
    }
}

#[get("/{room}/get_field/{field_id}")]
async fn get_field(param: web::Path<(String, FieldId)>) -> impl Responder {
    let room = &param.0;
    let field_id = param.1;

    format!("{} {}", room, field_id)
}

#[get("/{room}/update_field/{field_id}")]
async fn update_field(param: web::Path<(String, FieldId)>) -> impl Responder {
    let room = &param.0;
    let field_id = param.1;

    format!("{} {}", room, field_id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;

    println!("Starting server on {host}:{port}");

    HttpServer::new(|| {
        App::new().service(
            web::scope("/app").service(web::scope("/v1").service(get_field).service(update_field)),
        )
    })
    .bind((host, port))?
    .run()
    .await
}
