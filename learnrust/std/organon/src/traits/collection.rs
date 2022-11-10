use super::{ id::HasId, item::IsItemManager };

pub trait IsCollection: HasId + IsItemManager {}

pub trait HasCollection {
    type Collection: IsCollection;

    fn collection(&self) -> &Self::Collection;
}

#[cfg(test)]
pub mod tests {
    use crate::{ AtomicUsize, err, Ordering, Result, ToOwned, Vec, vec };
    use super::super::{
        data::{ GetData, HasData, ItemData },
        id::{ HasId, IsId, ItemId },
        item::{ IsItem, Item, IsItemManager, CreateItem, DeleteItem, UpdateItem, GetItem },
    };
    use super::IsCollection;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CollectionId(pub usize);

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

    #[test]
    fn has_id() {
        let col = Collection {
            id: CollectionId(1),
            items: vec!["item1", "item2", "item3"]
                .into_iter()
                .enumerate()
                .map(|(i, t)| Item { id: ItemId(i), data: ItemData(t.to_owned()) })
                .collect::<Vec<Item>>(),
        };

        assert_eq!(col.id, col.id);
        assert_eq!(col.id, col.id());
        assert_eq!(col.id(), col.id());
        assert_eq!(col.id(), CollectionId(1));
    }

    #[test]
    fn get_item() {
        let col = Collection {
            id: CollectionId(1),
            items: vec!["item1", "item2", "item3"]
                .into_iter()
                .enumerate()
                .map(|(i, t)| Item { id: ItemId(i), data: ItemData(t.to_owned()) })
                .collect::<Vec<Item>>(),
        };

        assert_eq!(*col.get_item(ItemId(0)).unwrap(), Item {
            id: ItemId(0),
            data: ItemData("item1".to_owned()),
        });
        assert_eq!(*col.get_item(ItemId(1)).unwrap(), Item {
            id: ItemId(1),
            data: ItemData("item2".to_owned()),
        });
        assert_eq!(*col.get_item(ItemId(2)).unwrap(), Item {
            id: ItemId(2),
            data: ItemData("item3".to_owned()),
        });
    }

    #[test]
    fn get_data() {
        let col = Collection {
            id: CollectionId(1),
            items: vec!["item1", "item2", "item3"]
                .into_iter()
                .enumerate()
                .map(|(i, t)| Item { id: ItemId(i), data: ItemData(t.to_owned()) })
                .collect::<Vec<Item>>(),
        };

        let id1 = ItemId(0);
        let id2 = ItemId(1);
        let id3 = ItemId(2);

        assert_eq!(*col.get_item(id1).unwrap(), Item {
            id: id1,
            data: col.get_data(id1).unwrap().clone(),
        });
        assert_eq!(*col.get_item(id2).unwrap(), Item {
            id: id2,
            data: col.get_data(id2).unwrap().clone(),
        });
        assert_eq!(*col.get_item(id3).unwrap(), Item {
            id: id3,
            data: col.get_data(id3).unwrap().clone(),
        });
    }
}