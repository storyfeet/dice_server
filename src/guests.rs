use serde_derive::*;

#[derive(Serialize, Deserialize)]
pub struct Guest {
    name: String,
    pass: String,
    rooms: Vec<String>,
    read: Vec<String>,
    write: Vec<String>,
}
