use crate::card::{Card, Rank, Suit};
use crate::draw::DrawFrom;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Represents a deck of playing cards.
#[derive(Debug)]
pub struct Deck(Vec<Card>);

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

impl DrawFrom for Deck {
    fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }
}
