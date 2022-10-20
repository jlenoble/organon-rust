use super::{ id::HasId, data::HasData };

pub trait IsItem: HasId + HasData {}

#[cfg(test)]
pub mod tests {
    extern crate alloc;
    use alloc::borrow::ToOwned;
    use super::super::{ id::{ HasId, tests::ItemId }, data::{ HasData, tests::ItemData } };
    use super::IsItem;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Item {
        id: ItemId,
        data: ItemData,
    }

    impl IsItem for Item {}

    impl HasId for Item {
        type Id = ItemId;

        fn id(&self) -> ItemId {
            self.id
        }
    }

    impl HasData for Item {
        type Data = ItemData;

        fn data(&self) -> &ItemData {
            &self.data
        }
    }

    #[test]
    fn test_is_item() {
        let item = Item { id: ItemId(12), data: ItemData("12".to_owned()) };

        assert_eq!(item.id, item.id);
        assert_eq!(item.id, item.id());
        assert_eq!(item.id(), item.id());

        assert_eq!(item.data, item.data);
        assert_eq!(&item.data, item.data());
        assert_eq!(item.data(), item.data());

        assert_eq!(item, item);
        assert_eq!(item, item.clone());
    }
}