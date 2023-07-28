use std::{borrow::Cow, time::Duration};

use crate::Table;

pub trait Storage {
    type Error;
}

pub trait StorageRead<TableType: Table>: Storage {
    fn get(
        &self,
        key: &TableType::Key,
    ) -> Result<Option<Cow<'_, TableType::OwnedValue>>, Self::Error>;
    fn exists(&self, key: &TableType::Key) -> Result<bool, Self::Error>;
}

pub trait StorageWrite<TableType: Table>: Storage {
    fn insert(
        &mut self,
        key: &TableType::Key,
        value: &TableType::Value,
    ) -> Result<Option<TableType::OwnedValue>, Self::Error>;
    fn remove(
        &mut self,
        key: &TableType::Key,
    ) -> Result<Option<TableType::OwnedValue>, Self::Error>;
}

pub trait StorageTemp<TableType: Table>: Storage {
    fn ttl(&self, key: &TableType::Key) -> Result<Duration, Self::Error>;
}

impl<T> Storage for &T
where
    T: Storage,
{
    type Error = T::Error;
}

impl<T> Storage for &mut T
where
    T: Storage,
{
    type Error = T::Error;
}

impl<T, TableType> StorageRead<TableType> for &T
where
    T: StorageRead<TableType>,
    TableType: Table,
{
    fn get(
        &self,
        key: &TableType::Key,
    ) -> Result<Option<Cow<'_, TableType::OwnedValue>>, Self::Error> {
        <T as StorageRead<TableType>>::get(self, key)
    }

    fn exists(&self, key: &TableType::Key) -> Result<bool, Self::Error> {
        <T as StorageRead<TableType>>::exists(self, key)
    }
}

impl<T, TableType> StorageRead<TableType> for &mut T
where
    T: StorageRead<TableType>,
    TableType: Table,
{
    fn get(
        &self,
        key: &TableType::Key,
    ) -> Result<Option<Cow<'_, TableType::OwnedValue>>, Self::Error> {
        <T as StorageRead<TableType>>::get(self, key)
    }

    fn exists(&self, key: &TableType::Key) -> Result<bool, Self::Error> {
        <T as StorageRead<TableType>>::exists(self, key)
    }
}

impl<T, TableType> StorageWrite<TableType> for &mut T
where
    T: StorageWrite<TableType>,
    TableType: Table,
{
    fn insert(
        &mut self,
        key: &TableType::Key,
        value: &TableType::Value,
    ) -> Result<Option<TableType::OwnedValue>, Self::Error> {
        <T as StorageWrite<TableType>>::insert(self, key, value)
    }

    fn remove(
        &mut self,
        key: &TableType::Key,
    ) -> Result<Option<TableType::OwnedValue>, Self::Error> {
        <T as StorageWrite<TableType>>::remove(self, key)
    }
}

impl<T, TableType> StorageTemp<TableType> for &T
where
    T: StorageTemp<TableType>,
    TableType: Table,
{
    fn ttl(&self, key: &TableType::Key) -> Result<Duration, Self::Error> {
        <T as StorageTemp<TableType>>::ttl(self, key)
    }
}

impl<T, TableType> StorageTemp<TableType> for &mut T
where
    T: StorageTemp<TableType>,
    TableType: Table,
{
    fn ttl(&self, key: &TableType::Key) -> Result<Duration, Self::Error> {
        <T as StorageTemp<TableType>>::ttl(self, key)
    }
}
