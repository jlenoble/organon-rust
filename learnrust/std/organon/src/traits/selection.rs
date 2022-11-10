use crate::Result;
use super::{ collection::HasCollection, id::HasId, item::IsItem };

pub trait IsSelection<'a>: HasId + HasCollection + SelectItem + UnselectItem {
    fn new<'b: 'a>(collection: &'b <Self as HasCollection>::Collection) -> Self;
}

pub trait SelectItem {
    type Item: IsItem;

    fn select_item(&mut self, id: <Self::Item as HasId>::Id) -> Result<&Self::Item>;
}

pub trait UnselectItem {
    type Item: IsItem;

    fn unselect_item(&mut self, id: <Self::Item as HasId>::Id) -> Result<&Self::Item>;
}

#[cfg(test)]
mod tests {
    use crate::{ AtomicUsize, err, HashSet, Result, Ordering, ToOwned, Vec, vec };
    use super::super::{
        collection::{ HasCollection, tests::{ Collection, CollectionId } },
        data::ItemData,
        id::{ HasId, IsId, ItemId },
        item::tests::Item,
        item_manager::GetItem,
    };
    use super::{ IsSelection, SelectItem, UnselectItem };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct SelectionId(usize);

    impl IsId for SelectionId {
        fn new() -> SelectionId {
            static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
            SelectionId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
        }
    }

    struct Selection<'a> {
        id: SelectionId,
        collection: &'a Collection,
        ids: HashSet<ItemId>,
    }

    impl<'a> IsSelection<'a> for Selection<'a> {
        fn new<'b: 'a>(collection: &'b Collection) -> Self {
            Self {
                id: SelectionId::new(),
                collection,
                ids: HashSet::new(),
            }
        }
    }

    impl<'a> HasId for Selection<'a> {
        type Id = SelectionId;

        fn id(&self) -> SelectionId {
            self.id
        }
    }

    impl<'a> HasCollection for Selection<'a> {
        type Collection = Collection;

        fn collection(&self) -> &Collection {
            self.collection
        }
    }

    impl<'a> SelectItem for Selection<'a> {
        type Item = Item;

        fn select_item(&mut self, id: ItemId) -> Result<&Item> {
            let id = {
                if let Some(item) = self.collection().get_item(id) {
                    item.id
                } else {
                    return Err(err!("deleted or inexistant item"));
                }
            };

            if self.ids.insert(id) {
                Ok(self.collection().get_item(id).unwrap())
            } else {
                Err(err!("item already selected"))
            }
        }
    }

    impl<'a> UnselectItem for Selection<'a> {
        type Item = Item;

        fn unselect_item(&mut self, id: ItemId) -> Result<&Item> {
            let id = {
                if let Some(item) = self.collection().get_item(id) {
                    item.id
                } else {
                    return Err(err!("deleted or inexistant item"));
                }
            };

            if self.ids.remove(&id) {
                Ok(self.collection().get_item(id).unwrap())
            } else {
                Err(err!("item already unselected"))
            }
        }
    }

    #[test]
    fn select_item() {
        let col = Collection {
            id: CollectionId(1),
            items: vec!["item1", "item2", "item3", "item4", "item5"]
                .into_iter()
                .enumerate()
                .map(|(i, t)| Item { id: ItemId(i), data: ItemData(t.to_owned()) })
                .collect::<Vec<Item>>(),
        };

        let mut sel = Selection::new(&col);

        if let Err(e) = sel.select_item(ItemId(1)) {
            panic!("{}", e);
        }
        if let Err(e) = sel.select_item(ItemId(3)) {
            panic!("{}", e);
        }

        assert_eq!(sel.ids, vec![ItemId(1), ItemId(3)].into_iter().collect::<HashSet<ItemId>>());
        assert_eq!(sel.ids, vec![ItemId(3), ItemId(1)].into_iter().collect::<HashSet<ItemId>>());
    }

    #[test]
    fn unselect_item() {
        let col = Collection {
            id: CollectionId(1),
            items: vec!["item1", "item2", "item3", "item4", "item5"]
                .into_iter()
                .enumerate()
                .map(|(i, t)| Item { id: ItemId(i), data: ItemData(t.to_owned()) })
                .collect::<Vec<Item>>(),
        };

        let mut sel = Selection::new(&col);

        if let Err(e) = sel.select_item(ItemId(1)) {
            panic!("{}", e);
        }
        if let Err(e) = sel.select_item(ItemId(3)) {
            panic!("{}", e);
        }
        if let Err(e) = sel.select_item(ItemId(4)) {
            panic!("{}", e);
        }

        assert_eq!(
            sel.ids,
            vec![ItemId(3), ItemId(1), ItemId(4)].into_iter().collect::<HashSet<ItemId>>()
        );

        if let Err(e) = sel.unselect_item(ItemId(4)) {
            panic!("{}", e);
        }
        if let Err(e) = sel.unselect_item(ItemId(1)) {
            panic!("{}", e);
        }

        assert_eq!(sel.ids, vec![ItemId(3)].into_iter().collect::<HashSet<ItemId>>());
    }
}