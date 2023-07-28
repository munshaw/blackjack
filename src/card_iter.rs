use crate::card_like::CardLike;
use std::fmt::Display;
use std::slice::Iter;

/// The ability to iterate through cards.
pub trait CardIter {
    /// The type to iterate through.
    type CardT: CardLike + Display;

    /// Returns an iterator over the slice.
    fn iter(&self) -> Iter<'_, Self::CardT>;
}
