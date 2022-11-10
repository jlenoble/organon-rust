use crate::{
    AtomicUsize,
    err,
    HashSet,
    Result,
    Ordering,
    traits::{
        collection::{ HasCollection, Collection },
        id::{ HasId, IsId, ItemId },
        item::{ Item, GetItem },
    },
};
use super::{ IsSelection, SelectItem, UnselectItem };

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SelectionId(usize);

impl IsId for SelectionId {
    fn new() -> SelectionId {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        SelectionId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub(crate) struct Selection<'a> {
    id: SelectionId,
    collection: &'a Collection,
    pub ids: HashSet<ItemId>,
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