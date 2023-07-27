use std::fmt::{Display, Formatter};

/// Playing card ranks.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

/// Playing card suits.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

/// Represents a French playing card.
#[derive(Debug, Eq, PartialEq)] // Don't implement Copy to prevent card duplication.
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Build a new playing card, of rank and suit.
    pub fn new(suit: &Suit, rank: &Rank) -> Card {
        Card {
            suit: *suit,
            rank: *rank,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}s", self.rank, self.suit)
    }
}
