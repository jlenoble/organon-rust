use super::{ id::HasId, data::{ GetData, HasData } };

pub trait IsItem: HasId + HasData {}

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

#[cfg(test)]
pub mod tests {
    extern crate alloc;
    use alloc::borrow::ToOwned;
    use super::super::{ id::{ HasId, tests::ItemId }, data::{ HasData, tests::ItemData } };
    use super::IsItem;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Item {
        pub id: ItemId,
        pub data: ItemData,
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