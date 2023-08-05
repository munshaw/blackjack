use blackjack::behaviour::card_iter::CardIter;
use blackjack::behaviour::card_like::{CardLike, MockCardLike};
use blackjack::behaviour::draw::{CannotDrawFromEmpty, DrawFrom, DrawTo, MockDrawFrom};
use blackjack::behaviour::score::{Score, Value};
use blackjack::card::Rank;
use std::cmp::Ordering;
use std::slice::Iter;

pub struct FakeHand(Vec<MockCardLike>);

impl FakeHand {
    pub fn new() -> FakeHand {
        FakeHand(Vec::new())
    }
}

impl CardIter for FakeHand {
    type Card = MockCardLike;

    fn iter(&self) -> Iter<'_, Self::Card> {
        self.0.iter()
    }
}

impl DrawTo<MockCardLike, MockDrawFrom<MockCardLike>> for FakeHand {
    fn draw_from(
        &mut self,
        cards: &mut MockDrawFrom<MockCardLike>,
    ) -> Result<(), CannotDrawFromEmpty> {
        match cards.draw() {
            None => Err(CannotDrawFromEmpty),
            Some(card) => {
                self.0.push(card);
                Ok(())
            }
        }
    }
}

impl Score for FakeHand {
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
