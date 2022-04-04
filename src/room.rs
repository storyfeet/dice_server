use serde_derive::*;

pub struct Event {
    date: u64,
    etype: EventType,
    is_public: bool,
}

pub enum EventType {
    Chat(String),
    Create { name: String, kind: String },
    Roll(String), //Consider making a Roll
}

pub enum Item {}

pub enum Template {}

pub struct Room {
    events: Vec<Event>,
    items: Vec<Item>,
    templates: Vec<Template>,
}
