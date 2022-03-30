use actix_web::*;
use serde_derive::*;
use sled::Db;
mod err;
use err::ARes;
mod user;
//use err_tools::*;
//use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct ORP {
    //owner: String,
    resource: String,
    player: String,
}

#[get("/")]
async fn index() -> String {
    "Food".to_string()
}

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    username: String,
    password: String,
}

#[get("/new_user")]
async fn new_user(form: web::Query<NewUser>, db: web::Data<Db>) -> ARes<HttpResponse> {
    println!("New user called");
    let nu = form.into_inner();

    let hash_user = user::User::new(nu.username, &nu.password)?;
    let dbi = db.into_inner();
    //    let ulock = dbi.lock();
    //   let ures = ulock.ok().e_str("Poisoned Mutex")?;
    let users = dbi.open_tree("users")?;
    let name = hash_user.name.clone();
    users.insert(
        &name.as_bytes(),
        serde_json::to_string(&hash_user)?.as_bytes(),
    )?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"todo":"Send Auth keys"}"#))
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
    let _orp = path.into_inner();
    HttpResponse::Ok()
        .content_type("application-json")
        .body("{}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(sled::open("test/maindb.db"));

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db.clone())
            .service(new_user)
            .service(room)
            .service(index)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
