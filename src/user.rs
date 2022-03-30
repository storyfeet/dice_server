use crate::err::ARes;
use rand::Rng;
use serde_derive::*;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    salt: Vec<u8>,
    hash: String,
    difficulty: u32,
}

impl User {
    pub fn new(name: String, pass: &str) -> ARes<Self> {
        let mut salt = [0; 20];
        rand::thread_rng().fill(&mut salt);
        let difficulty = bcrypt::DEFAULT_COST;
        let mut psalt: Vec<u8> = Vec::new();
        psalt.extend(&salt);
        psalt.extend(pass.as_bytes());
        let hash = bcrypt::hash(psalt, difficulty)?;
        Ok(User {
            name,
            salt: salt.to_vec(),
            hash: hash,
            difficulty,
        })
    }
}
