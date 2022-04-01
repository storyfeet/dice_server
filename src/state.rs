use crate::auth::AuthList;
use sled::Db;
#[derive(Clone)]
pub struct State {
    pub db: Db,
    pub auth: AuthList<String>,
}

impl State {
    pub fn new(dbloc: &str) -> anyhow::Result<Self> {
        Ok(Self {
            db: sled::open(dbloc)?,
            auth: AuthList::new(30 * 60),
        })
    }
}
