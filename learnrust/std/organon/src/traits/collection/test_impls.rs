use crate::{ AtomicUsize, err, Ordering, Result, Vec };
use super::super::{
    data::{ HasData, ItemData },
    id::{ HasId, IsId, ItemId },
    item::{ IsItem, Item, IsItemManager, CreateItem, DeleteItem, UpdateItem, GetItem },
};
use super::IsCollection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CollectionId(pub usize);

impl IsId for CollectionId {
    fn new() -> CollectionId {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        CollectionId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Collection {
    pub id: CollectionId,
    pub items: Vec<Item>,
}

impl IsCollection for Collection {}

impl IsItemManager for Collection {}

impl HasId for Collection {
    type Id = CollectionId;

    fn id(&self) -> CollectionId {
        self.id
    }
}

impl CreateItem for Collection {
    type Item = Item;

    fn create_item(&mut self, data: ItemData) -> Result<&Item> {
        self.items.push(Item::new(data));
        Ok(self.items.last().unwrap())
    }
}

impl DeleteItem for Collection {
    type Item = Item;

    fn delete_item(&mut self, id: ItemId) -> Result<ItemData> {
        if let Some(pos) = self.items.iter().position(|item| item.id() == id) {
            Ok(self.items.remove(pos).data().clone())
        } else {
            Err(err!("already deleted or inexistant"))
        }
    }
}

impl UpdateItem for Collection {
    type Item = Item;

    fn update_item(&mut self, id: ItemId, data: ItemData) -> Result<&Item> {
        if let Some(pos) = self.items.iter().position(|item| item.id() == id) {
            self.items[pos] = Item { id, data };
            Ok(&self.items[pos])
        } else {
            Err(err!("not found"))
        }
    }
}

impl GetItem for Collection {
    type Item = Item;

    fn get_item(&self, id: ItemId) -> Option<&Item> {
        let pos = self.items.iter().position(|item| item.id() == id)?;
        Some(&self.items[pos])
    }
}