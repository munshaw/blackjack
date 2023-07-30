use crate::card::Card;
use crate::card_like::CardLike;
use mockall::automock;
use std::fmt::Display;
use std::slice::Iter;

/// The ability to iterate through cards.
#[automock(type Card = Card;)]
pub trait CardIter {
    type Card: CardLike + Display;

    /// Returns an iterator over the slice.
    fn iter(&self) -> Iter<'_, Self::Card>;
}
