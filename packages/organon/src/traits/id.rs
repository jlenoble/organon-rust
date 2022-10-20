pub trait IsId {
    fn new() -> Self;
}

pub trait HasId {
    type Id: IsId;

    fn id(&self) -> Self::Id;
}

#[cfg(test)]
pub mod tests {
    use core::sync::atomic::{ AtomicUsize, Ordering };
    use super::{ HasId, IsId };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ItemId(pub usize);

    impl IsId for ItemId {
        fn new() -> ItemId {
            static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
            ItemId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
        }
    }

    #[test]
    fn test_has_id() {
        struct Item {
            id: ItemId,
        }

        impl HasId for Item {
            type Id = ItemId;

            fn id(&self) -> ItemId {
                self.id
            }
        }

        let id = Item { id: ItemId(12) };

        assert_eq!(id.id, id.id);
        assert_eq!(id.id, id.id());
        assert_eq!(id.id(), id.id());
    }
}