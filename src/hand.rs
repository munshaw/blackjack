use crate::behaviour::card_iter::CardIter;
use crate::behaviour::card_like::CardLike;
use crate::behaviour::draw::{CannotDrawFromEmpty, DrawFrom, DrawTo};
use crate::behaviour::score::{Score, Value};
use crate::card::Rank;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::slice::Iter;

/// Represents a hand of playing cards.
#[derive(Debug)]
pub struct Hand<C>(Vec<C>);

impl<C> Hand<C> {
    /// Build an empty hand.
    pub fn new() -> Hand<C> {
        Hand(Vec::new())
    }
}

impl<C> CardIter for Hand<C>
where
    C: CardLike,
{
    type Card = C;

    fn iter(&self) -> Iter<'_, Self::Card> {
        self.0.iter()
    }
}

impl<C, D> DrawTo<C, D> for Hand<C>
where
    C: CardLike,
    D: DrawFrom<C>,
{
    fn draw_from(&mut self, cards: &mut D) -> Result<(), CannotDrawFromEmpty> {
        match cards.draw() {
            None => Err(CannotDrawFromEmpty),
            Some(card) => {
                self.0.push(card);
                Ok(())
            }
        }
    }
}

impl<C> Score for Hand<C>
where
    C: CardLike,
{
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
            Ordering::Less => Value::Points(points, aces > 0),
            Ordering::Greater => Value::Bust,
            Ordering::Equal => Value::Blackjack,
        }
    }
}

impl<C> Display for Hand<C>
where
    C: CardLike + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = self.0.iter().fold(
            String::new(),
            |s, c| if s == "" { s } else { s + ", " } + &*c.to_string(),
        );
        write!(f, "{}", result)
    }
}
