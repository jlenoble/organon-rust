use crate::Result;
use super::{ data::IsData, id::IsId };

pub trait IsDataManager: CreateData + DeleteData + UpdateData + GetData {}

pub trait CreateData {
    type Id: IsId;
    type Data: IsData;

    fn create_data(&mut self, data: Self::Data) -> Result<(Self::Id, &Self::Data)>;
}

pub trait DeleteData {
    type Id: IsId;
    type Data: IsData;

    fn delete_data(&mut self, id: Self::Id) -> Result<Self::Data>;
}

pub trait UpdateData {
    type Id: IsId;
    type Data: IsData;

    fn update_data(&mut self, id: Self::Id, data: Self::Data) -> Result<&Self::Data>;
}

pub trait GetData {
    type Id: IsId;
    type Data: IsData;

    fn get_data(&self, id: Self::Id) -> Option<&Self::Data>;
}

#[cfg(test)]
mod tests {
    use crate::{ err, HashMap, Result, ToOwned, Vec };
    use super::super::{ id::{ IsId, tests::ItemId }, data::tests::ItemData };
    use super::{ IsDataManager, CreateData, DeleteData, UpdateData, GetData };

    struct DataManager(HashMap<ItemId, ItemData>);

    impl IsDataManager for DataManager {}

    impl CreateData for DataManager {
        type Id = ItemId;
        type Data = ItemData;

        fn create_data(&mut self, data: Self::Data) -> Result<(Self::Id, &Self::Data)> {
            let id = ItemId::new();
            if let Some(_) = self.0.insert(id, data) {
                Err(err!("new id already exists: out of sync"))
            } else {
                Ok((id, self.0.get(&id).unwrap()))
            }
        }
    }

    impl DeleteData for DataManager {
        type Id = ItemId;
        type Data = ItemData;

        fn delete_data(&mut self, id: ItemId) -> Result<Self::Data> {
            if let Some(data) = self.0.remove(&id) {
                Ok(data)
            } else {
                Err(err!("data already deleted: out of sync"))
            }
        }
    }

    impl UpdateData for DataManager {
        type Id = ItemId;
        type Data = ItemData;

        fn update_data(&mut self, id: ItemId, data: Self::Data) -> Result<&Self::Data> {
            if let Some(_) = self.0.insert(id, data) {
                Ok(self.0.get(&id).unwrap())
            } else {
                Err(err!("update of missing data: out of sync"))
            }
        }
    }

    impl GetData for DataManager {
        type Id = ItemId;
        type Data = ItemData;

        fn get_data(&self, id: Self::Id) -> Option<&Self::Data> {
            self.0.get(&id)
        }
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
}