use crate::{
    ToOwned,
    traits::{ data::{ GetData, ItemData }, id::{ HasId, ItemId }, item::{ GetItem, Item } },
    Vec,
    vec,
};
use super::{ Collection, CollectionId };

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