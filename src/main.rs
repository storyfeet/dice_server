use serde_derive::*;
use sled::Db;
mod err;
mod uri_reader;
mod user;
use err_tools::*;
//use std::sync::{Arc, Mutex};
use hyper::{service::*, *};
use std::convert::Infallible;
use std::str::FromStr;

const CONTENT_TYPE: &str = "Content-Type";
type HRes<T> = anyhow::Result<Response<T>>;

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

async fn new_user(req: Request<Body>, db: Db) -> HRes<Body> {
    println!("New user called");

    let user = user::User::from_query(req.uri().query().e_str("No Params")?)?;
    let users = db.open_tree("users")?;
    users.insert(
        &user.name.as_bytes(),
        serde_json::to_string(&user)?.as_bytes(),
    )?;
    Ok(Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .body(format!(r#"{{"new_user":"{}"}}"#, user.name).into())?)
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

async fn muxer(req: Request<Body>, db: Db) -> std::result::Result<Response<Body>, Infallible> {
    let res = match req.uri().path() {
        "/new_user" => new_user(req, db).await,
        p => e_string(format!("Not a valid path: {}", p)),
    };
    match res {
        Ok(v) => Ok(v),
        Err(e) => Ok(Response::new(format!("Error:{}", e).into())),
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
