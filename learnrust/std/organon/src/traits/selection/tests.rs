use crate::{
    HashSet,
    ToOwned,
    traits::{
        collection::{ Collection, CollectionId },
        data::ItemData,
        id::{ ItemId },
        item::{ Item },
    },
    Vec,
    vec,
};

use super::{ IsSelection, SelectItem, UnselectItem, Selection };

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