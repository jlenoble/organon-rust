use crate::{
    err,
    HashMap,
    Result,
    traits::{ data::{ HasData, ItemData }, id::{ HasId, IsId, ItemId } },
};
use super::{ CreateItem, DeleteItem, GetItem, IsItem, IsItemManager, UpdateItem };

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Item {
    pub id: ItemId,
    pub data: ItemData,
}

impl IsItem for Item {
    fn new(data: ItemData) -> Item {
        Item {
            id: ItemId::new(),
            data,
        }
    }
}

impl HasId for Item {
    type Id = ItemId;

    fn id(&self) -> ItemId {
        self.id
    }
}

impl HasData for Item {
    type Data = ItemData;

    fn data(&self) -> &ItemData {
        &self.data
    }
}

pub(crate) struct ItemManager(pub HashMap<ItemId, Item>);

impl IsItemManager for ItemManager {}

impl CreateItem for ItemManager {
    type Item = Item;

    fn create_item(&mut self, data: ItemData) -> Result<&Item> {
        let item = Item::new(data);
        let id = item.id;
        if let Some(_) = self.0.insert(id, item) {
            Err(err!("new id already exists: out of sync"))
        } else {
            Ok(self.0.get(&id).unwrap())
        }
    }
}

impl DeleteItem for ItemManager {
    type Item = Item;

    fn delete_item(&mut self, id: ItemId) -> Result<ItemData> {
        if let Some(item) = self.0.remove(&id) {
            Ok(item.data().clone())
        } else {
            Err(err!("data already deleted: out of sync"))
        }
    }
}

impl UpdateItem for ItemManager {
    type Item = Item;

    fn update_item(&mut self, id: ItemId, data: ItemData) -> Result<&Item> {
        if let Some(_) = self.0.insert(id, Item { id, data }) {
            Ok(self.0.get(&id).unwrap())
        } else {
            Err(err!("update of missing data: out of sync"))
        }
    }
}

impl GetItem for ItemManager {
    type Item = Item;

    fn get_item(&self, id: ItemId) -> Option<&Item> {
        self.0.get(&id)
    }
}