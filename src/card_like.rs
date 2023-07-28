use crate::card::{Rank, Suit};

/// A trait for something to act as a playing card.
pub trait CardLike {
    /// Get the rank of the card.
    fn get_rank(&self) -> Rank;

    /// Get the suit of the card.
    fn get_suit(&self) -> Suit;
}
