pub trait Table {
    type Key: ?Sized + ToOwned;
    type OwnedKey: From<<Self::Key as ToOwned>::Owned> + Clone;
    type Value: ?Sized + ToOwned;
    type OwnedValue: From<<Self::Value as ToOwned>::Owned> + Clone;
}
