use serde_derive::*;
mod auth;
mod guests;
mod room;
mod state;
mod uri_reader;
mod user;
use err_tools::*;
use state::State;
//use std::sync::{Arc, Mutex};
use auth::Auth;
use hyper::{service::*, *};
use serde::{de::DeserializeOwned, Serialize};
use std::convert::Infallible;
use std::ops::Deref;
use std::str::FromStr;

const CONTENT_TYPE: &str = "Content-Type";
const CT_HTML: &str = "text/html";
const CT_JS: &str = "application/javascript";
const CT_JSON: &str = "application/json";
const CT_CSS: &str = "text/css";
type HRes<T> = anyhow::Result<Response<T>>;

#[derive(Serialize)]
pub struct AuthResponse<T: Clone + Serialize, R: Serialize> {
    auth: Auth<T>,
    data: R,
}

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    username: String,
    password: String,
}

pub fn ok_json<T: Serialize + Clone, D: Serialize>(auth: Auth<T>, data: D) -> HRes<Body> {
    let au = AuthResponse { auth, data };
    Ok(Response::builder()
        .header(CONTENT_TYPE, CT_JSON)
        .body(serde_json::to_string(&au)?.into())?)
}

pub fn get_data<'a, T: DeserializeOwned>(
    t: &'a sled::Tree,
    name: &str,
) -> anyhow::Result<Option<T>> {
    match t.get(name)? {
        Some(v) => Ok(serde_json::from_str(std::str::from_utf8(v.deref())?)?),
        None => Ok(None),
    }
}

async fn page(req: Request<Body>) -> HRes<Body> {
    let (ct, s) = match req.uri().path() {
        "/" => (CT_HTML, include_str!("static/index.html")),
        "/static/jquery.min.js" => (CT_JS, include_str!("static/jquery.min.js")),
        "/static/main.css" => (CT_CSS, include_str!("static/main.css")),
        _ => e_str("Path did not reach anything")?,
    };

    Ok(Response::builder()
        .header(CONTENT_TYPE, ct)
        .body(s.into())?)
}

async fn new_user(req: Request<Body>, st: State) -> HRes<Body> {
    println!("New user called");

    let user = user::User::from_query(req.uri().query().e_str("No Params")?)?.hash()?;
    let users = st.db.open_tree("users")?;
    match get_data::<user::HashUser>(&users, &user.name) {
        Ok(None) => {}
        Ok(Some(_)) => return e_str("User already exists"),
        Err(_) => return e_str("Could not access Users"),
    }

    users.insert(
        &user.name.as_bytes(),
        serde_json::to_string(&user)?.as_bytes(),
    )?;

    let auth = st.auth.new_auth(user.name.clone());
    ok_json(auth, user.name)
}

async fn login(req: Request<Body>, st: State) -> HRes<Body> {
    println!("Login Called");
    let user = user::User::from_query(req.uri().query().e_str("No Params")?)?;
    let users = st.db.open_tree("users")?;
    let hu: user::HashUser = get_data(&users, &user.name)?.e_str("User does not exist")?;
    if !hu.verify(&user) {
        return e_str("Could not verify user");
    }
    let auth = st.auth.new_auth(user.name.clone());
    ok_json(auth, user.name)
}

pub async fn renew_login(req: Request<Body>, st: State) -> HRes<Body> {
    println!("Check Login called");
    let auth = st
        .auth
        .check_query(req.uri().query().e_str("Params for Auth")?)?;

    let users = st.db.open_tree("users")?;
    let hu: user::HashUser = get_data(&users, &auth.data)?.e_str("Login for non existent user")?;
    ok_json(auth, hu.name)
}

pub async fn create_guest(req: Request<Body>, st: State) -> HRes<Body> {
    println!("new_guest");
    let auth = st
        .auth
        .check_query(req.uri().query().e_str("Parames for Auth")?)?;
    let guests = st.db.open_tree("guests")?;
    let newguest = serde_urlencoded::from_str(req.uri().query().e_str("no guest data")?)?;
    let mut glist: Vec<guests::Guest> = get_data(&guests, &auth.data)?.unwrap_or(Vec::new());
    glist.push(newguest);
    guests.insert(&auth.data, serde_json::to_string(&glist)?.as_bytes())?;
    ok_json(auth, glist)
}

async fn muxer(req: Request<Body>, st: State) -> std::result::Result<Response<Body>, Infallible> {
    let res = match req.uri().path() {
        "/new_user" => new_user(req, st).await,
        "/login" => login(req, st).await,
        "/renew_login" => renew_login(req, st).await,
        "/create_guest" => create_guest(req, st).await,
        _ => page(req).await,
    };
    match res {
        Ok(v) => Ok(v),
        Err(e) => Ok(Response::new(format!("Error:{}", e).into())),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sstate = State::new("test/maindb.db")?;

    let make_svc = make_service_fn(move |_conn| {
        let ss = sstate.clone();
        async { Ok::<_, Infallible>(service_fn(move |req| muxer(req, ss.clone()))) }
    });

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let addr = std::net::SocketAddr::from_str("127.0.0.1:8080")?;
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
