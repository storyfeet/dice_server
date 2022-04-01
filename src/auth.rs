use err_tools::*;
use rand::Rng;
use serde_derive::*;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize)]
pub struct Auth<T: Clone> {
    k: u64,
    p: u64,
    expires: u64,
    data: T,
}

#[derive(Clone)]
struct AuthInner<T: Clone> {
    mp: BTreeMap<u64, Auth<T>>,
}

#[derive(Clone)]
pub struct AuthList<T: Clone> {
    mp: Arc<RwLock<AuthInner<T>>>,
    ttl: u64,
}

impl<T: Clone> AuthList<T> {
    pub fn new(ttl: u64) -> Self {
        Self {
            mp: Arc::new(RwLock::new(AuthInner {
                mp: BTreeMap::new(),
            })),
            ttl,
        }
    }
    pub fn new_auth(&self, data: T) -> Auth<T> {
        let expires = (SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .expect("Now is before the UNIX_EPOCH")
            .as_secs()
            + self.ttl;
        let mut tr = rand::thread_rng();
        let mut inner = self.mp.write().expect("Could not lock");
        let mut k: u64 = tr.gen();
        while let Some(_) = inner.mp.get(&k) {
            k = tr.gen();
        }
        let p = tr.gen();
        let res = Auth {
            k,
            p,
            expires,
            data,
        };
        inner.mp.insert(k, res.clone());
        res
    }

    pub fn check(&self, k: u64, p: u64) -> anyhow::Result<T> {
        let inner = self.mp.read().ok().e_str("Poisoned RwLock")?;
        let a = inner.mp.get(&k).e_str("Token Key not valid")?;
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
        Ok(self.new_auth(r))
    }
}
