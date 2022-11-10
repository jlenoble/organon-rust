use crate::{ traits::id::IsId, Result };
use super::data::IsData;

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