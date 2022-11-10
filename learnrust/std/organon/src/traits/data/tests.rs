use crate::{ HashMap, ToOwned, traits::id::ItemId, Vec };
use super::{ CreateData, DataManager, DeleteData, GetData, HasData, ItemData, UpdateData };

#[test]
fn has_data() {
    struct Item {
        data: ItemData,
    }

    impl HasData for Item {
        type Data = ItemData;

        fn data(&self) -> &ItemData {
            &self.data
        }
    }

    let data = Item { data: ItemData("12".to_owned()) };

    assert_eq!(data.data, data.data);
    assert_eq!(&data.data, data.data());
    assert_eq!(data.data(), data.data());
}

#[test]
fn create_data() {
    let mut manager = DataManager(HashMap::new());

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
    let mut manager = DataManager(HashMap::new());
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
    let mut manager = DataManager(HashMap::new());

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