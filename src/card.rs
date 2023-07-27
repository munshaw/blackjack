use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::slice::Iter;

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

/// Represents a hand of playing cards.
#[derive(Debug)]
pub struct Hand(Vec<Card>);

impl Hand {
    /// Build an empty hand.
    pub fn new() -> Hand {
        Hand(Vec::new())
    }
}

/// The ability to iterate through cards.
pub trait CardIter {
    /// Returns an iterator over the slice.
    fn iter(&self) -> Iter<'_, Card>;
}

impl CardIter for Hand {
    fn iter(&self) -> Iter<'_, Card> {
        self.0.iter()
    }
}

/// Error used when attempting to draw a card from an empty card collection.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CannotDrawFromEmpty;

/// The ability to draw from a drawable collection of cards.
pub trait DrawFrom {
    /// Draw a card from a drawable collection of cards into this.
    fn draw_from<DrawT: Draw>(&mut self, cards: &mut DrawT) -> Result<(), CannotDrawFromEmpty>;
}

impl DrawFrom for Hand {
    fn draw_from<DrawT: Draw>(&mut self, cards: &mut DrawT) -> Result<(), CannotDrawFromEmpty> {
        match cards.draw() {
            None => Err(CannotDrawFromEmpty),
            Some(card) => {
                self.0.push(card);
                Ok(())
            }
        }
    }
}

/// The score of a blackjack hand.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Value {
    Bust,
    Blackjack,
    Points(u8),
}

/// The ability to evaluate the blackjack value of cards.
pub trait Score {
    fn score(&self) -> Value;
}

impl Score for Hand {
    fn score(&self) -> Value {
        let mut points = 0;
        let mut aces = 0;

        for card in &self.0[0..] {
            points += match card.rank {
                Rank::Ace => {
                    aces += 1;
                    11
                }
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten => 10,
                Rank::Jack => 10,
                Rank::Queen => 10,
                Rank::King => 10,
            };
        }

        // Try valuing aces at 1 to avoid busting.
        while points > 21 && aces > 0 {
            points -= 10;
            aces -= 1;
        }

        match points.cmp(&{ 21 }) {
            Ordering::Less => Value::Points(points),
            Ordering::Greater => Value::Bust,
            Ordering::Equal => Value::Blackjack,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self.0.iter().fold(
            String::new(),
            |s, c| if s == "" { s } else { s + ", " } + &*c.to_string(),
        );
        write!(f, "{}", result)
    }
}
