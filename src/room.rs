use serde_derive::*;

#[derive(Serialize, Deserialize)]
pub struct Event {
    date: u64,
    etype: EventType,
    is_public: bool,
}

#[derive(Serialize, Deserialize)]
pub enum EventType {
    Chat(String),
    Create { name: String, kind: String },
    Roll(String), //Consider making a Roll
}

pub enum Template {}

#[derive(Serialize, Deserialize)]
pub struct Room {
    logs: Vec<String>,
    permissions: Vec<Permission>,
    data: Vec<String>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            permissions: Vec::new(),
            data: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Permission {
    name: Option<String>,
    rooms: Vec<String>,
    read: Vec<String>,
    write: Vec<String>,
    create: Vec<String>,
}
