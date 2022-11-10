use super::{ data::HasData, id::HasId };

pub trait IsItem: HasId + HasData {
    fn new(data: <Self as HasData>::Data) -> Self;
}

#[cfg(test)]
pub mod tests {
    use crate::ToOwned;
    use super::super::{ data::{ HasData, tests::ItemData }, id::{ HasId, IsId, ItemId } };
    use super::IsItem;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Item {
        pub id: ItemId,
        pub data: ItemData,
    }

    impl IsItem for Item {
        fn new(data: ItemData) -> Item {
            Item {
                id: ItemId::new(),
                data,
            }
        }
    }

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
    fn is_item() {
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