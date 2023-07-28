use crate::card::Card;
use std::slice::Iter;

/// The ability to iterate through cards.
pub trait CardIter {
    /// Returns an iterator over the slice.
    fn iter(&self) -> Iter<'_, Card>;
}
