use crate::card_like::CardLike;
use std::fmt::Display;

/// Error used when attempting to draw a card from an empty card collection.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CannotDrawFromEmpty;

/// The ability to draw cards from this collection of cards.
pub trait DrawFrom<CardT: CardLike + Display> {
    /// Draw a card from this collection.
    fn draw(&mut self) -> Option<CardT>;
}

/// The ability to draw from a drawable collection of cards.
pub trait DrawTo<DrawFromT: DrawFrom<Self::Card>> {
    type Card: CardLike + Display;

    /// Draw a card from a drawable collection of cards into this.
    fn draw_from(&mut self, cards: &mut DrawFromT) -> Result<(), CannotDrawFromEmpty>;
}
