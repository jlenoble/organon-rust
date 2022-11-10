use super::{ HasId, ItemId };

#[test]
fn has_id() {
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