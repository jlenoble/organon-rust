pub trait IsData {}

pub trait HasData {
    type Data: IsData;

    fn data(&self) -> &Self::Data;
}