use err_tools::*;
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
    pub fn new(name: String, pass: &str) -> anyhow::Result<Self> {
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

    pub fn from_query(q: &str) -> anyhow::Result<Self> {
        let mp = crate::uri_reader::QueryMap::new(q).map;
        let name = mp.get("name").e_str("User needs a Name")?.to_string();
        let pass = mp.get("pass").e_str("User needs a Password")?;
        Self::new(name, pass)
    }
}
