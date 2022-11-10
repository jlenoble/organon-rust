use crate::{
    HashMap,
    ToOwned,
    traits::{
        data::{ CreateData, DeleteData, UpdateData, GetData, HasData, ItemData },
        id::{ HasId, ItemId },
    },
    Vec,
};
use super::{ Item, ItemManager, CreateItem, DeleteItem, UpdateItem, GetItem };

#[test]
fn is_item() {
    let item = Item { id: ItemId(12), data: ItemData("12".to_owned()) };

    assert_eq!(item.id, item.id);
    assert_eq!(item.id, item.id());
    assert_eq!(item.id(), item.id());

    assert_eq!(item.data, item.data);
    assert_eq!(&item.data, item.data());
    assert_eq!(item.data(), item.data());

    assert_eq!(item, item);
    assert_eq!(item, item.clone());
}

#[test]
fn create_item() {
    let mut manager = ItemManager(HashMap::new());

    let (id, data) = {
        let mng = &mut manager;
        if let Ok(item) = mng.create_item(ItemData("item".to_owned())) {
            (item.id(), item.data().clone())
        } else {
            panic!("failed to create data");
        }
    };

    assert_eq!(Item { id, data }, *manager.get_item(id).unwrap());
}

#[test]
fn delete_item() {
    let mut manager = ItemManager(HashMap::new());
    let mut ids: Vec<ItemId> = Vec::new();

    let ids = {
        let mng = &mut manager;
        ids.push(
            if let Ok(item) = mng.create_item(ItemData("item1".to_owned())) {
                item.id()
            } else {
                panic!("failed to create data");
            }
        );
        ids.push(
            if let Ok(item) = mng.create_item(ItemData("item2".to_owned())) {
                item.id()
            } else {
                panic!("failed to create data");
            }
        );
        ids.push(
            if let Ok(item) = mng.create_item(ItemData("item3".to_owned())) {
                item.id()
            } else {
                panic!("failed to create data");
            }
        );
        ids
    };

    assert_eq!(manager.0.len(), 3);

    let data = manager.delete_item(ids[0]).unwrap();

    assert_eq!(manager.0.len(), 2);
    assert_eq!(data, ItemData("item1".to_owned()));

    let data = manager.delete_item(ids[2]).unwrap();

    assert_eq!(manager.0.len(), 1);
    assert_eq!(data, ItemData("item3".to_owned()));

    let data = manager.delete_item(ids[1]).unwrap();

    assert_eq!(manager.0.len(), 0);
    assert_eq!(data, ItemData("item2".to_owned()));
}

#[test]
fn update_item() {
    let mut manager = ItemManager(HashMap::new());

    let (id, data) = {
        let mng = &mut manager;
        if let Ok(item) = mng.create_item(ItemData("item1".to_owned())) {
            (item.id(), item.data().clone())
        } else {
            panic!("failed to create data");
        }
    };

    assert_eq!(ItemData("item1".to_owned()), data);
    assert_eq!(Item { id, data }, *manager.get_item(id).unwrap());

    let (id, data) = {
        let mng = &mut manager;
        if let Ok(item) = mng.update_item(id, ItemData("item2".to_owned())) {
            (item.id(), item.data().clone())
        } else {
            panic!("failed to update data");
        }
    };

    assert_eq!(ItemData("item2".to_owned()), data);
    assert_eq!(Item { id, data: ItemData("item2".to_owned()) }, *manager.get_item(id).unwrap());
}

#[test]
fn create_data() {
    let mut manager = ItemManager(HashMap::new());

    let (id, data) = {
        let mng = &mut manager;
        if let Ok((id, data)) = mng.create_data(ItemData("item".to_owned())) {
            (id, data.clone())
        } else {
            panic!("failed to create data");
        }
    };

    assert_eq!(data, *manager.get_data(id).unwrap());
}

#[test]
fn delete_data() {
    let mut manager = ItemManager(HashMap::new());
    let mut ids: Vec<ItemId> = Vec::new();

    let ids = {
        let mng = &mut manager;
        ids.push(
            if let Ok((id, _)) = mng.create_data(ItemData("item1".to_owned())) {
                id
            } else {
                panic!("failed to create data");
            }
        );
        ids.push(
            if let Ok((id, _)) = mng.create_data(ItemData("item2".to_owned())) {
                id
            } else {
                panic!("failed to create data");
            }
        );
        ids.push(
            if let Ok((id, _)) = mng.create_data(ItemData("item3".to_owned())) {
                id
            } else {
                panic!("failed to create data");
            }
        );
        ids
    };

    assert_eq!(manager.0.len(), 3);

    let data = manager.delete_data(ids[0]).unwrap();

    assert_eq!(manager.0.len(), 2);
    assert_eq!(data, ItemData("item1".to_owned()));

    let data = manager.delete_data(ids[2]).unwrap();

    assert_eq!(manager.0.len(), 1);
    assert_eq!(data, ItemData("item3".to_owned()));

    let data = manager.delete_data(ids[1]).unwrap();

    assert_eq!(manager.0.len(), 0);
    assert_eq!(data, ItemData("item2".to_owned()));
}

#[test]
fn update_data() {
    let mut manager = ItemManager(HashMap::new());

    let (id, data) = {
        let mng = &mut manager;
        if let Ok((id, data)) = mng.create_data(ItemData("item1".to_owned())) {
            (id, data.clone())
        } else {
            panic!("failed to create data");
        }
    };

    assert_eq!(ItemData("item1".to_owned()), data);
    assert_eq!(ItemData("item1".to_owned()), *manager.get_data(id).unwrap());

    let data = {
        let mng = &mut manager;
        if let Ok(data) = mng.update_data(id, ItemData("item2".to_owned())) {
            data.clone()
        } else {
            panic!("failed to update data");
        }
    };

    assert_eq!(ItemData("item2".to_owned()), data);
    assert_eq!(ItemData("item2".to_owned()), *manager.get_data(id).unwrap());
}