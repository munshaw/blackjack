use crate::card_like::CardLike;
use std::slice::Iter;

/// The ability to iterate through cards.
pub trait CardIter {
    type Card: CardLike;

    /// Returns an iterator over the slice.
    fn iter(&self) -> Iter<'_, Self::Card>;
}
