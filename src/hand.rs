use crate::card::Rank;
use crate::card_iter::CardIter;
use crate::card_like::CardLike;
use crate::draw::{CannotDrawFromEmpty, DrawFrom, DrawTo};
use crate::score::{Score, Value};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::slice::Iter;

/// Represents a hand of playing cards.
#[derive(Debug)]
pub struct Hand<CardT: CardLike + Display>(Vec<CardT>);

impl<CardT: CardLike + Display> Hand<CardT> {
    /// Build an empty hand.
    pub fn new() -> Hand<CardT> {
        Hand(Vec::new())
    }
}

impl<CardT: CardLike + Display> CardIter for Hand<CardT> {
    type CardT = CardT;

    fn iter(&self) -> Iter<'_, Self::CardT> {
        self.0.iter()
    }
}

impl<CardT: CardLike + Display, DrawFromT: DrawFrom<CardT>> DrawTo<DrawFromT> for Hand<CardT> {
    type Card = CardT;

    fn draw_from(&mut self, cards: &mut DrawFromT) -> Result<(), CannotDrawFromEmpty> {
        match cards.draw() {
            None => Err(CannotDrawFromEmpty),
            Some(card) => {
                self.0.push(card);
                Ok(())
            }
        }
    }
}

impl<CardT: CardLike + Display> Score for Hand<CardT> {
    fn score(&self) -> Value {
        let mut points = 0;
        let mut aces = 0;

        for card in &self.0[0..] {
            points += match card.get_rank() {
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

impl<CardT: CardLike + Display> Display for Hand<CardT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self.0.iter().fold(
            String::new(),
            |s, c| if s == "" { s } else { s + ", " } + &*c.to_string(),
        );
        write!(f, "{}", result)
    }
}
