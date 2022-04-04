use err_tools::*;
use rand::{seq::SliceRandom, Rng};
use serde_derive::*;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

const KCHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890";

fn gen_key<R: Rng>(r: &mut R) -> String {
    let b = KCHARS.as_bytes();
    let mut res = String::new();
    for _ in 0..15 {
        res.push(*(b.choose(r).expect("KCHARS empty")) as char);
    }
    res.push('_');
    for _ in 0..15 {
        res.push(*(b.choose(r).expect("KCHARS empty")) as char);
    }
    res
}

#[derive(Clone, Serialize)]
pub struct Auth<T: Clone> {
    k: String,
    expires: u64,
    pub data: T,
}

#[derive(Clone)]
struct AuthInner<T: Clone> {
    mp: BTreeMap<String, Auth<T>>,
}

#[derive(Clone)]
///Key is two parts, first to name, second to authorize, split on _
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
        let mut k = gen_key(&mut tr);
        let mut ksp = k.split('_').next().expect("split has at least 1 member");
        while let Some(_) = inner.mp.get(ksp) {
            k = gen_key(&mut tr);
            ksp = k.split('_').next().expect("split has at least 1 member");
        }
        let res = Auth {
            k: k.clone(),
            expires,
            data,
        };
        inner.mp.insert(ksp.to_string(), res.clone());
        res
    }

    pub fn check_query(&self, qs: &str) -> anyhow::Result<Auth<T>> {
        let mp = crate::uri_reader::QueryMap::new(qs).map;
        let kp = mp.get("k").e_str("auth needs key k")?;
        self.check(kp)
    }

    pub fn check(&self, k: &str) -> anyhow::Result<Auth<T>> {
        let inner = self.mp.read().ok().e_str("Poisoned RwLock")?;
        let a = inner
            .mp
            .get(k.split("_").next().e_str("Key Bad")?)
            .e_str("Token Key not valid")?;
        if a.k != k {
            return e_str("Pass not valid");
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("now before unix Epoch")
            .as_secs();
        match now < a.expires {
            true => Ok(a.clone()),
            false => e_str("Token Expired"),
        }
    }

    pub fn renew(&self, ar: &str) -> anyhow::Result<Auth<T>> {
        let r = self.check(ar)?;
        Ok(self.new_auth(r.data.clone()))
    }
}
