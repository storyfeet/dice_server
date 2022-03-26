use actix_web::*;
use serde_derive::*;

#[derive(Deserialize)]
pub struct ORP {
    owner: String,
    resource: String,
    player: String,
}

#[get("/")]
async fn index() -> String {
    "Food".to_string()
}

#[get("/room/{owner}/{resource}/{player}")]
async fn room(path: web::Path<ORP>) -> HttpResponse {
    let orp = path.into_inner();
    HttpResponse::Ok().content_type("text-html").body(format!(
        include_str!("static/room.html"),
        room = orp.resource,
        owner = orp.player,
    ))
}

#[get("/events/{owner}/{resource}/{player}")]
async fn events(path: web::Path<ORP>) -> HttpResponse {
    let orp = path.into_inner();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(room).service(index))
        .bind("localhost:8080")?
        .run()
        .await
}
