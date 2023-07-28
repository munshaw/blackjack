use crate::card::Card;

/// Error used when attempting to draw a card from an empty card collection.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CannotDrawFromEmpty;

/// The ability to draw cards from this collection of cards.
pub trait DrawFrom {
    /// Draw a card from this collection.
    fn draw(&mut self) -> Option<Card>;
}

/// The ability to draw from a drawable collection of cards.
pub trait DrawTo {
    /// Draw a card from a drawable collection of cards into this.
    fn draw_from<DrawT: DrawFrom>(&mut self, cards: &mut DrawT) -> Result<(), CannotDrawFromEmpty>;
}
