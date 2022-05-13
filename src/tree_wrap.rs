use serde::{de::DeserializeOwned, Serialize};
use sled::IVec;

pub trait TreeGetPut {
    fn tget<K: AsRef<[u8]>>(&self, key: K) -> anyhow::Result<Option<IVec>>;
    fn insert<K: AsRef<[u8]>, V: Into<IVec>>(
        &self,
        key: K,
        value: V,
    ) -> anyhow::Result<Option<IVec>>;

    fn get_item<V: DeserializeOwned>(&self, key: &str) -> anyhow::Result<Option<V>> {
        match self.tget(key)? {
            Some(v) => Ok(serde_json::from_str(std::str::from_utf8(v.as_ref())?)?),
            None => Ok(None),
        }
    }
    fn put_item<V: Serialize>(&self, k: &str, v: &V) -> anyhow::Result<()> {
        self.insert(k.as_bytes(), serde_json::to_string(v)?.as_bytes())?;
        Ok(())
    }
}

impl TreeGetPut for sled::Tree {
    fn tget<K: AsRef<[u8]>>(&self, key: K) -> anyhow::Result<Option<IVec>> {
        self.get(key).map_err(|e| e.into())
    }
    fn insert<K: AsRef<[u8]>, V: Into<IVec>>(
        &self,
        key: K,
        value: V,
    ) -> anyhow::Result<Option<IVec>> {
        self.insert(key, value).map_err(|e| e.into())
    }
}
impl TreeGetPut for sled::transaction::TransactionalTree {
    fn tget<K: AsRef<[u8]>>(&self, key: K) -> anyhow::Result<Option<IVec>> {
        self.get(key).map_err(|e| e.into())
    }
    fn insert<K: AsRef<[u8]>, V: Into<IVec>>(
        &self,
        key: K,
        value: V,
    ) -> anyhow::Result<Option<IVec>> {
        self.insert(key.as_ref(), value).map_err(|e| e.into())
    }
}
