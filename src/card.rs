use rand::seq::SliceRandom;
use rand::thread_rng;
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

/// Represents a deck of playing cards.
#[derive(Debug)]
pub struct Deck(Vec<Card>);

/// The ability to draw cards from this collection of cards.
pub trait Draw {
    /// Draw a card from this collection.
    fn draw(&mut self) -> Option<Card>;
}

impl Deck {
    /// Build a freshly shuffled deck.
    pub fn new() -> Deck {
        let suits = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];
        let mut deck: Vec<Card> = Vec::new();
        suits
            .iter()
            .for_each(|s| ranks.iter().for_each(|r| deck.push(Card::new(s, r))));
        deck.shuffle(&mut thread_rng());
        Deck(deck)
    }
}

impl Draw for Deck {
    fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }
}
