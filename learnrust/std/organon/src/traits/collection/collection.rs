use crate::traits::{ id::HasId, item::IsItemManager };

pub trait IsCollection: HasId + IsItemManager {}

pub trait HasCollection {
    type Collection: IsCollection;

    fn collection(&self) -> &Self::Collection;
}

#[cfg(test)]
pub mod tests {}