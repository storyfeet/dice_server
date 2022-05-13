use serde_derive::*;
mod auth;
mod guests;
mod room;
mod state;
mod uri_reader;
mod user;
use err_tools::*;
use state::State;
use uri_reader::QueryMap;
//use std::sync::{Arc, Mutex};
use auth::Auth;
use hyper::{service::*, *};
use serde::{de::DeserializeOwned, Serialize};
//use std::convert::Infallible;
use room::{Permission, Room};
use std::ops::Deref;
use std::str::FromStr;

const CONTENT_TYPE: &str = "Content-Type";
const CT_HTML: &str = "text/html";
const CT_JS: &str = "application/javascript";
const CT_JSON: &str = "application/json";
const CT_CSS: &str = "text/css";

const TBL_USERS: &str = "users";
//const TBL_GUESTS: &str = "guests";
const TBL_ROOM_LIST: &str = "room_list";
const TBL_ROOMS: &str = "rooms";
//const TBL_DATA: &str = "data"; //Scenes,templates,characters,

type HRes<T> = anyhow::Result<Response<T>>;

pub async fn split_param_data(
    req: Request<Body>,
) -> anyhow::Result<(http::request::Parts, QueryMap)> {
    let (p, body) = req.into_parts();
    if let Some(s) = p.uri.query() {
        let mp = QueryMap::new(s);
        return Ok((p, mp));
    }
    let data = hyper::body::to_bytes(body).await?;
    let s: &str = std::str::from_utf8(&data)?;
    Ok((p, QueryMap::new(s)))
}

#[derive(Serialize)]
pub struct AuthResponse<T: Clone + Serialize, R: Serialize> {
    auth: Option<Auth<T>>,
    data: R,
}

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    username: String,
    password: String,
}

pub fn ok_json<T: Serialize + Clone, D: Serialize>(auth: Option<Auth<T>>, data: D) -> HRes<Body> {
    let au = AuthResponse { auth, data };
    Ok(Response::builder()
        .header(CONTENT_TYPE, CT_JSON)
        .body(serde_json::to_string(&au)?.into())?)
}

pub fn put_data<V: Serialize>(t: &sled::Tree, k: &str, v: &V) -> anyhow::Result<()> {
    t.insert(k.as_bytes(), serde_json::to_string(v)?.as_bytes())?;
    Ok(())
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
        "/" => {
            return Ok(Response::builder()
                .header(CONTENT_TYPE, CT_HTML)
                .body(format!(include_str!("static/index.html"), "undefined").into())?);
        }
        "/static/jquery.min.js" => (CT_JS, include_str!("static/jquery.min.js")),
        "/static/requests.js" => (CT_JS, include_str!("static/requests.js")),
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
    let users = st.db.open_tree(TBL_USERS)?;
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
    ok_json(Some(auth), user.name)
}

async fn process_login(req: Request<Body>, st: State) -> anyhow::Result<Auth<String>> {
    let (_, qmap) = split_param_data(req).await?;
    let user = user::User::from_qmap(&qmap)?;
    let users = st.db.open_tree(TBL_USERS)?;
    let hu: user::HashUser = get_data(&users, &user.name)?.e_str("User does not exist")?;
    if !hu.verify(&user) {
        return e_str("Could not verify user");
    }
    Ok(st.auth.new_auth(user.name.clone()))
}

async fn login(req: Request<Body>, st: State) -> HRes<Body> {
    let auth = process_login(req, st).await?;
    let dt = auth.data.clone();
    ok_json(Some(auth), dt)
}

pub async fn renew_login(req: Request<Body>, st: State) -> HRes<Body> {
    println!("Check Login called");
    let (_, qmap) = split_param_data(req).await?;
    let auth = st.auth.check_qdata(&qmap)?;

    let users = st.db.open_tree(TBL_USERS)?;
    let hu: user::HashUser = get_data(&users, &auth.data)?.e_str("Login for non existent user")?;
    ok_json(Some(auth), hu.name)
}

pub async fn create_room(req: Request<Body>, st: State) -> HRes<Body> {
    let (_p, qmap) = split_param_data(req).await?;
    let auth = st.auth.check_qdata(&qmap)?;
    let rname = qmap
        .map
        .get("room_name")
        .e_str("no room name provided to create room")?;

    //Get users room list
    let room_list = st.db.open_tree(TBL_ROOM_LIST)?;
    let mut list: Vec<String> = match get_data(&room_list, &auth.data) {
        Ok(Some(d)) => d,
        Ok(None) => Vec::new(),
        Err(e) => return Err(e),
    };
    if !list.contains(rname) {
        list.push(rname.to_string());
        room_list.insert(
            &auth.data.as_bytes(),
            serde_json::to_string(&list)?.as_bytes(),
        )?;
    }

    ok_json(Some(auth), rname.to_string())
}

pub async fn list_rooms(req: Request<Body>, st: State) -> HRes<Body> {
    let (_p, qmap) = split_param_data(req).await?;
    let rname = match qmap.map.get("name") {
        Some(s) => s.to_string(),
        None => st.auth.check_qdata(&qmap)?.data,
    };
    let room_list = st.db.open_tree(TBL_ROOM_LIST)?;
    let list: Vec<String> = match get_data(&room_list, &rname) {
        Ok(Some(d)) => {
            println!("GOT LIST:{:?}", d);
            d
        }
        Ok(None) => Vec::new(),
        Err(e) => return Err(e),
    };
    ok_json::<(), _>(None, &list)
}

async fn qlogin(req: Request<Body>, st: State) -> HRes<Body> {
    let rp = Response::builder().header(CONTENT_TYPE, CT_HTML);
    match process_login(req, st).await {
        Ok(au) => {
            let aj = serde_json::to_string(&au)?;
            Ok(rp.body(format!(include_str!("static/index.html"), aj).into())?)
        }
        Err(_) => Ok(rp.body(format!(include_str!("static/index.html"), "undefined").into())?),
    }
}

async fn set_permissions(req: Request<Body>, st: State) -> HRes<Body> {
    let (_p, qmap) = split_param_data(req).await?;
    let auth = st.auth.check_qdata(&qmap)?;
    let rname = qmap.get("room_name").e_str("No room_name provided")?;
    let rpath = format!("{}/{}", auth.data, rname);
    let names = qmap
        .get("names")
        .e_str("No guest_name provided")?
        .to_string();
    let read = qmap.get("read").unwrap_or("").to_string();
    let write = qmap.get("write").unwrap_or("").to_string();
    let create = qmap.get("create").unwrap_or("").to_string();
    let rooms = st.db.open_tree(TBL_ROOMS)?;
    let mut r: Room = match get_data(&rooms, &rpath) {
        Ok(Some(r)) => r,
        Err(e) => return e_string(format!("Could not read room:{} - {}", rpath, e)),
        Ok(None) => Room::new(),
    };
    r.permissions.push(Permission {
        names,
        read,
        write,
        create,
    });
    put_data(&rooms, &rpath, &r)?;
    ok_json(Some(auth), r)
}

async fn view_permissions(req: Request<Body>, st: State) -> HRes<Body> {
    let (_p, qmap) = split_param_data(req).await?;
    let auth = st.auth.check_qdata(&qmap)?;
    let rpath = qmap.get("room_path").e_str("No room_path provided")?;
    let owner = rpath.split("/").next().unwrap();

    let rooms = st.db.open_tree(TBL_ROOMS)?;
    let r: Room = match get_data(&rooms, &rpath) {
        Ok(Some(r)) => r,
        Err(e) => return e_string(format!("Could not read room:{} - {}", rpath, e)),
        Ok(None) => Room::new(),
    };

    if owner == auth.data {
        ok_json(Some(auth), &r.permissions)
    } else {
        let p = r.guest_permissions(&auth.data);
        ok_json(Some(auth), &p)
    }
}

async fn muxer(req: Request<Body>, st: State) -> anyhow::Result<Response<Body>> {
    let res = match req.uri().path() {
        "/new_user" => new_user(req, st).await,
        "/login" => login(req, st).await,
        "/renew_login" => renew_login(req, st).await,
        "/set_permissions" => set_permissions(req, st).await,
        "/view_permissions" => view_permissions(req, st).await,
        "/create_room" => create_room(req, st).await,
        "/list_rooms" => list_rooms(req, st).await,
        "/qlogin" => qlogin(req, st).await,
        _ => page(req).await,
    };
    match res {
        Ok(v) => Ok(v),
        Err(e) => Ok(Response::builder()
            .header(CONTENT_TYPE, CT_JSON)
            .body(format!(r#"{{"err":"{}" }}"#, e).into())?),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sstate = State::new("test/maindb.db")?;

    let make_svc = make_service_fn(move |_conn| {
        let ss = sstate.clone();
        async { Ok::<_, anyhow::Error>(service_fn(move |req| muxer(req, ss.clone()))) }
    });

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let addr = std::net::SocketAddr::from_str("127.0.0.1:8080")?;
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}
