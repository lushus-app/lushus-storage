use std::{borrow::Cow, time::Duration};

use crate::{StorageRead, StorageTemp, StorageWrite, Table};

pub struct StorageMut<'a, T: 'a + ?Sized, TableType: Table>(
    &'a mut T,
    core::marker::PhantomData<TableType>,
);

impl<'a, T: StorageRead<TableType>, TableType: Table> StorageMut<'a, T, TableType> {
    #[inline(always)]
    pub fn get(
        self,
        key: &TableType::Key,
    ) -> Result<Option<Cow<'a, TableType::OwnedValue>>, T::Error> {
        let self_: &'a T = self.0;
        self_.get(key)
    }

    #[inline(always)]
    pub fn exists(self, key: &TableType::Key) -> Result<bool, T::Error> {
        self.0.exists(key)
    }
}

impl<'a, T: StorageWrite<TableType>, TableType: Table> StorageMut<'a, T, TableType> {
    #[inline(always)]
    pub fn insert(
        self,
        key: &TableType::Key,
        value: &TableType::Value,
    ) -> Result<Option<TableType::OwnedValue>, T::Error> {
        self.0.insert(key, value)
    }

    #[inline(always)]
    pub fn remove(self, key: &TableType::Key) -> Result<Option<TableType::OwnedValue>, T::Error> {
        self.0.remove(key)
    }
}

impl<'a, T: StorageTemp<TableType>, TableType: Table> StorageMut<'a, T, TableType> {
    #[inline(always)]
    pub fn ttl(self, key: &TableType::Key) -> Result<Duration, T::Error> {
        self.0.ttl(key)
    }
}

pub trait StorageAsMut {
    #[inline(always)]
    fn storage<TableType>(&mut self) -> StorageMut<Self, TableType>
    where
        TableType: Table,
    {
        self.storage_as_mut()
    }

    #[inline(always)]
    fn storage_as_mut<TableType>(&mut self) -> StorageMut<Self, TableType>
    where
        TableType: Table,
    {
        StorageMut(self, Default::default())
    }
}

impl<T> StorageAsMut for T {}
