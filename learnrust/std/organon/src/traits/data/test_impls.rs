use crate::{ err, HashMap, Result, String, traits::id::{ IsId, ItemId } };
use super::{ CreateData, DeleteData, GetData, IsData, IsDataManager, UpdateData };

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ItemData(pub String);

impl IsData for ItemData {}

pub(crate) struct DataManager(pub HashMap<ItemId, ItemData>);

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