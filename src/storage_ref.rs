use std::{borrow::Cow, time::Duration};

use crate::{StorageRead, StorageTemp, Table};

pub struct StorageRef<'a, T: 'a + ?Sized, TableType: Table>(
    &'a T,
    core::marker::PhantomData<TableType>,
);

impl<'a, T: StorageRead<TableType>, TableType: Table> StorageRef<'a, T, TableType> {
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

impl<'a, T: StorageTemp<TableType>, TableType: Table> StorageRef<'a, T, TableType> {
    #[inline(always)]
    pub fn ttl(self, key: &TableType::Key) -> Result<Duration, T::Error> {
        self.0.ttl(key)
    }
}

pub trait StorageAsRef {
    #[inline(always)]
    fn storage<TableType>(&self) -> StorageRef<Self, TableType>
    where
        TableType: Table,
    {
        self.storage_as_ref()
    }

    #[inline(always)]
    fn storage_as_ref<TableType>(&self) -> StorageRef<Self, TableType>
    where
        TableType: Table,
    {
        StorageRef(self, Default::default())
    }
}

impl<T> StorageAsRef for T {}
