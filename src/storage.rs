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
    fn insert<V: Serialize + DeserializeOwned>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<Option<V>, Self::Error>;
    fn remove<V: DeserializeOwned>(&mut self, key: &K) -> Result<Option<V>, Self::Error>;
}

pub trait StorageTemp<K>: Storage {
    fn ttl(&self, key: &K) -> Result<Duration, Self::Error>;
}

impl<'a, T> Storage for &'a T
where
    T: Storage,
{
    type Error = T::Error;
}

impl<'a, T> Storage for &'a mut T
where
    T: Storage,
{
    type Error = T::Error;
}

impl<'a, K, T> StorageRead<K> for &'a T
where
    T: StorageRead<K>,
{
    fn get<V: DeserializeOwned>(&self, key: &K) -> Result<Option<V>, Self::Error> {
        <T as StorageRead<K>>::get(self, key)
    }

    fn exists(&self, key: &K) -> Result<bool, Self::Error> {
        <T as StorageRead<K>>::exists(self, key)
    }
}

impl<'a, K, T> StorageRead<K> for &'a mut T
where
    T: StorageRead<K>,
{
    fn get<V: DeserializeOwned>(&self, key: &K) -> Result<Option<V>, Self::Error> {
        <T as StorageRead<K>>::get(self, key)
    }

    fn exists(&self, key: &K) -> Result<bool, Self::Error> {
        <T as StorageRead<K>>::exists(self, key)
    }
}

impl<'a, K, T> StorageWrite<K> for &'a mut T
where
    T: StorageWrite<K>,
{
    fn insert<V: Serialize + DeserializeOwned>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<Option<V>, Self::Error> {
        <T as StorageWrite<K>>::insert(self, key, value)
    }

    fn remove<V: DeserializeOwned>(&mut self, key: &K) -> Result<Option<V>, Self::Error> {
        <T as StorageWrite<K>>::remove(self, key)
    }
}

impl<'a, K, T> StorageTemp<K> for &'a mut T
where
    T: StorageTemp<K>,
{
    fn ttl(&self, key: &K) -> Result<Duration, Self::Error> {
        <T as StorageTemp<K>>::ttl(self, key)
    }
}
