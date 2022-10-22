use crate::result::Result;
use super::{
    data::HasData,
    data_manager::{ IsDataManager, CreateData, DeleteData, UpdateData, GetData },
    id::HasId,
    item::IsItem,
};

pub trait IsItemManager: CreateItem + DeleteItem + UpdateItem + GetItem + IsDataManager {}

impl<T: IsItemManager> IsDataManager for T {}

pub trait CreateItem {
    type Item: IsItem;

    fn create_item(&mut self, data: <Self::Item as HasData>::Data) -> Result<&Self::Item>;
}

impl<T: CreateItem> CreateData for T {
    type Id = <<T as CreateItem>::Item as HasId>::Id;
    type Data = <<T as CreateItem>::Item as HasData>::Data;

    fn create_data(&mut self, data: Self::Data) -> Result<(Self::Id, &Self::Data)> {
        let item = self.create_item(data)?;
        Ok((item.id(), item.data()))
    }
}

pub trait DeleteItem {
    type Item: IsItem;

    fn delete_item(
        &mut self,
        id: <Self::Item as HasId>::Id
    ) -> Result<<Self::Item as HasData>::Data>;
}

impl<T: DeleteItem> DeleteData for T {
    type Id = <<T as DeleteItem>::Item as HasId>::Id;
    type Data = <<T as DeleteItem>::Item as HasData>::Data;

    fn delete_data(&mut self, id: Self::Id) -> Result<Self::Data> {
        self.delete_item(id)
    }
}

pub trait UpdateItem {
    type Item: IsItem;

    fn update_item(
        &mut self,
        id: <Self::Item as HasId>::Id,
        data: <Self::Item as HasData>::Data
    ) -> Result<&Self::Item>;
}

impl<T: UpdateItem> UpdateData for T {
    type Id = <<T as UpdateItem>::Item as HasId>::Id;
    type Data = <<T as UpdateItem>::Item as HasData>::Data;

    fn update_data(&mut self, id: Self::Id, data: Self::Data) -> Result<&Self::Data> {
        Ok(self.update_item(id, data)?.data())
    }
}

pub trait GetItem {
    type Item: IsItem;

    fn get_item(&self, id: <Self::Item as HasId>::Id) -> Option<&Self::Item>;
}

impl<T: GetItem> GetData for T {
    type Id = <<Self as GetItem>::Item as HasId>::Id;
    type Data = <<Self as GetItem>::Item as HasData>::Data;

    fn get_data(&self, id: Self::Id) -> Option<&Self::Data> {
        Some(self.get_item(id)?.data())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    extern crate alloc;
    use std::collections::HashMap;
    use alloc::{ borrow::ToOwned, vec::Vec };

    use crate::result::{ err, Result };
    use super::super::{
        data::{ HasData, tests::ItemData },
        id::{ HasId, tests::ItemId },
        item::{ IsItem, tests::Item },
    };
    use super::{
        IsItemManager,
        CreateItem,
        DeleteItem,
        UpdateItem,
        GetItem,
        CreateData,
        DeleteData,
        UpdateData,
        GetData,
    };

    struct ItemManager(HashMap<ItemId, Item>);

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
}