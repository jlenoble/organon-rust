pub trait IsData {}

pub trait HasData {
    type Data: IsData;

    fn data(&self) -> &Self::Data;
}

#[cfg(test)]
pub mod tests {
    extern crate alloc;
    use alloc::{ borrow::ToOwned, string::String };
    use super::{ HasData, IsData };

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ItemData(pub String);

    impl IsData for ItemData {}

    #[test]
    fn has_data() {
        struct Item {
            data: ItemData,
        }

        impl HasData for Item {
            type Data = ItemData;

            fn data(&self) -> &ItemData {
                &self.data
            }
        }

        let data = Item { data: ItemData("12".to_owned()) };

        assert_eq!(data.data, data.data);
        assert_eq!(&data.data, data.data());
        assert_eq!(data.data(), data.data());
    }
}