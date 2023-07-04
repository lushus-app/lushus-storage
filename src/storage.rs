use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};

pub trait Storage {
    type Error;
}

pub trait StorageRead<K>: Storage {
    fn get<T: DeserializeOwned>(&self, key: &K) -> Result<Option<T>, Self::Error>;
    fn exists(&self, key: &K) -> Result<bool, Self::Error>;
}

pub trait StorageWrite<K>: Storage {
    fn insert<T: Serialize + DeserializeOwned>(
        &mut self,
        key: &K,
        value: &T,
    ) -> Result<Option<T>, Self::Error>;
    fn remove<T: DeserializeOwned>(&mut self, key: &K) -> Result<Option<T>, Self::Error>;
}

pub trait StorageTemp<K>: Storage {
    fn ttl(&self, key: &K) -> Result<Duration, Self::Error>;
}
