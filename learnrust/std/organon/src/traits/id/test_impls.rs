use crate::{ AtomicUsize, Ordering };
use super::IsId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ItemId(pub usize);

impl IsId for ItemId {
    fn new() -> ItemId {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        ItemId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}