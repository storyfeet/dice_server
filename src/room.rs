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
    /// _ for everyone, comma separated list of names
    pub names: String,
    ///Comma separated list of paths
    pub read: String,
    ///Comma separated list of paths
    pub write: String,
    ///Comma separated list of paths
    pub create: String,
}

impl Permission {
    fn permit_path(ss: &str, needle: &str) -> bool {
        for s in ss.split(",") {
            let t = s.trim();
            if t == "" {
                continue;
            }
            if needle.starts_with(t) {
                return true;
            }
        }
        false
    }
    pub fn permit_name(&self, name: &str) -> bool {
        for s in self.names.split(",") {
            let t = s.trim();
            if t == "" {
                continue;
            }
            if t == "_" {
                return true;
            }
            if t == name {
                return true;
            }
        }
        return false;
    }

    pub fn permit_read(&self, name: &str, path: &str) -> bool {
        self.permit_name(name) && Self::permit_path(&self.read, path)
    }
    pub fn permit_write(&self, name: &str, path: &str) -> bool {
        self.permit_name(name) && Self::permit_path(&self.write, path)
    }
    pub fn permit_create(&self, name: &str, path: &str) -> bool {
        self.permit_name(name) && Self::permit_path(&self.create, path)
    }
}
