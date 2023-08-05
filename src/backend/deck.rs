use crate::backend::card::{Card, Rank, Suit};
use crate::behaviour::draw::DrawFrom;
use rand::seq::SliceRandom;
use rand::thread_rng;

const RANKS: [Rank; 13] = [
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

const SUITS: [Suit; 4] = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];

/// Represents a deck of playing cards.
#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Deck {
    /// Build a freshly shuffled deck.
    pub fn new() -> Deck {
        let mut deck: Vec<Card> = Vec::new();
        SUITS
            .iter()
            .for_each(|s| RANKS.iter().for_each(|r| deck.push(Card::new(*s, *r))));
        deck.shuffle(&mut thread_rng());
        Deck(deck)
    }
}

impl DrawFrom<Card> for Deck {
    fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }
}
