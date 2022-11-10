use crate::traits::{ data::HasData, id::HasId };

pub trait IsItem: HasId + HasData {
    fn new(data: <Self as HasData>::Data) -> Self;
}