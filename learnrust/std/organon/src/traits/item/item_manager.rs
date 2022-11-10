use crate::{
    traits::{
        data::{ HasData, IsDataManager, CreateData, DeleteData, UpdateData, GetData },
        id::HasId,
    },
    Result,
};
use super::item::IsItem;

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