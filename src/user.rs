use crate::uri_reader::QueryMap;
use err_tools::*;
use rand::Rng;
use serde_derive::*;

#[derive(Serialize, Deserialize)]
pub struct HashUser {
    pub name: String,
    salt: Vec<u8>,
    hash: String,
    difficulty: u32,
}

pub struct User {
    pub name: String,
    pass: String,
}
impl User {
    pub fn from_query(q: &str) -> anyhow::Result<Self> {
        let mp = QueryMap::new(q);
        Self::from_qmap(&mp)
    }

    pub fn from_qmap(mp: &QueryMap) -> anyhow::Result<Self> {
        let name = mp.map.get("name").e_str("User needs a Name")?.to_string();
        let pass = mp
            .map
            .get("pass")
            .e_str("User needs a Password")?
            .to_string();
        Ok(Self { name, pass })
    }

    pub fn hash(self) -> anyhow::Result<HashUser> {
        HashUser::new(self.name, &self.pass)
    }
}

impl HashUser {
    pub fn new(name: String, pass: &str) -> anyhow::Result<Self> {
        let mut salt = [0; 20];
        rand::thread_rng().fill(&mut salt);
        let difficulty = bcrypt::DEFAULT_COST;
        let mut psalt: Vec<u8> = Vec::new();
        psalt.extend(&salt);
        psalt.extend(pass.as_bytes());
        let hash = bcrypt::hash(psalt, difficulty)?;
        Ok(Self {
            name,
            salt: salt.to_vec(),
            hash: hash,
            difficulty,
        })
    }

    pub fn verify(&self, user: &User) -> bool {
        let mut ps = self.salt.clone();
        ps.extend(user.pass.as_bytes());
        bcrypt::verify(ps, &self.hash).unwrap_or(false)
    }
}
