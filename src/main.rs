use serde_derive::*;
use sled::Db;
mod err;
mod user;
//use err_tools::*;
//use std::sync::{Arc, Mutex};
use err::ARes;
use hyper::{service::*, *};
use std::convert::Infallible;
use std::str::FromStr;

const CONTENT_TYPE: &str = "content_type";
type HRes<T> = ARes<Response<T>>;

#[derive(Deserialize)]
pub struct ORP {
    //owner: String,
    resource: String,
    player: String,
}

async fn index() -> String {
    "Food".to_string()
}

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    username: String,
    password: String,
}

async fn new_user(nu: NewUser, db: Db) -> HRes<String> {
    println!("New user called");

    let hash_user = user::User::new(nu.username, &nu.password)?;
    //    let ulock = dbi.lock();
    //   let ures = ulock.ok().e_str("Poisoned Mutex")?;
    let users = db.open_tree("users")?;
    let name = hash_user.name.clone();
    users.insert(
        &name.as_bytes(),
        serde_json::to_string(&hash_user)?.as_bytes(),
    )?;
    Ok(Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .body(r#"{"todo":"Send Auth keys"}"#.to_string())?)
}

async fn room(orp: ORP) -> HRes<String> {
    Ok(http::Response::builder()
        .header(CONTENT_TYPE, "text-html")
        .body(format!(
            include_str!("static/room.html"),
            room = orp.resource,
            owner = orp.player,
        ))?)
}

async fn events(orp: ORP) -> HRes<String> {
    Ok(Response::builder()
        .header(CONTENT_TYPE, "application-json")
        .body("{}".to_string())?)
}

async fn muxer(req: Request<Body>, _db: Db) -> std::result::Result<Response<Body>, Infallible> {
    match (req.uri().path()) {
        "/new_user" => Ok(Response::new("New User Eh".into())),
        _ => Ok(Response::new("Hello, from Muxer".into())),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = sled::open("test/maindb.db")?;

    let make_svc = make_service_fn(move |_conn| {
        let dc = db.clone();
        async { Ok::<_, Infallible>(service_fn(move |req| muxer(req, dc.clone()))) }
    });

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let addr = std::net::SocketAddr::from_str("127.0.0.1:8080")?;
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
