use super::{ id::HasId, item::GetItem };

pub trait IsCollection: HasId + GetItem {}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::{ borrow::ToOwned, vec, vec::Vec };
    use core::sync::atomic::{ AtomicUsize, Ordering };
    use crate::traits::data::tests::ItemData;

    use super::super::{
        data::GetData,
        id::{ HasId, IsId, tests::ItemId },
        item::{ GetItem, tests::Item },
    };
    use super::IsCollection;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct CollectionId(usize);

    impl IsId for CollectionId {
        fn new() -> CollectionId {
            static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
            CollectionId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Collection {
        id: CollectionId,
        items: Vec<Item>,
    }

    impl IsCollection for Collection {}

    impl HasId for Collection {
        type Id = CollectionId;

        fn id(&self) -> CollectionId {
            self.id
        }
    }

    impl GetItem for Collection {
        type Item = Item;

        fn get_item(&self, id: ItemId) -> Option<&Item> {
            if id.0 < self.items.len() { Some(&self.items[id.0]) } else { None }
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