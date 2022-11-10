use crate::{ Result, traits::{ collection::HasCollection, id::HasId, item::IsItem } };

pub trait IsSelection<'a>: HasId + HasCollection + SelectItem + UnselectItem {
    fn new<'b: 'a>(collection: &'b <Self as HasCollection>::Collection) -> Self;
}

pub trait SelectItem {
    type Item: IsItem;

    fn select_item(&mut self, id: <Self::Item as HasId>::Id) -> Result<&Self::Item>;
}

pub trait UnselectItem {
    type Item: IsItem;

    fn unselect_item(&mut self, id: <Self::Item as HasId>::Id) -> Result<&Self::Item>;
}