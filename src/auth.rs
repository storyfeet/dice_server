use err_tools::*;
use rand::Rng;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Auth<T: Clone> {
    k: u64,
    p: u64,
    expires: u64,
    data: T,
}

#[derive(Clone)]
pub struct AuthList<T: Clone> {
    mp: Arc<RwLock<BTreeMap<u64, Auth<T>>>>,
}

impl<T: Clone> AuthList<T> {
    pub fn new() -> Self {
        Self {
            mp: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
    pub fn new_auth(&self, data: T, ttl: Duration) -> Auth<T> {
        let expires = (SystemTime::now() + ttl)
            .duration_since(UNIX_EPOCH)
            .expect("Now is before the UNIX_EPOCH")
            .as_secs();
        let mut tr = rand::thread_rng();
        let mut mp = self.mp.write().expect("Could not lock");
        let mut k: u64 = tr.gen();
        while let Some(_) = mp.get(&k) {
            k = tr.gen();
        }
        let p = tr.gen();
        let res = Auth {
            k,
            p,
            expires,
            data,
        };
        mp.insert(k, res.clone());
        res
    }

    pub fn check(&self, k: u64, p: u64) -> anyhow::Result<T> {
        let mp = self.mp.read().ok().e_str("Poisoned RwLock")?;
        let a = mp.get(&k).e_str("Token Key not valid")?;
        if a.p != p {
            return e_str("Pass not valid");
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("now before unix Epoch")
            .as_secs();
        match now < a.expires {
            true => Ok(a.data.clone()),
            false => e_str("Token Expired"),
        }
    }

    pub fn renew(&self, k: u64, p: u64) -> anyhow::Result<Auth<T>> {
        let r = self.check(k, p)?;
        Ok(self.new_auth(r, Duration::from_secs(30 * 60)))
    }
}
