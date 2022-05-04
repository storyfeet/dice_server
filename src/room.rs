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

pub struct Room {
    logs: Vec<String>,
}
