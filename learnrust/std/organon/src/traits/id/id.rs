pub trait IsId {
    fn new() -> Self;
}

pub trait HasId {
    type Id: IsId;

    fn id(&self) -> Self::Id;
}