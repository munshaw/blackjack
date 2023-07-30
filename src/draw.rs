use crate::card_like::CardLike;
use mockall::automock;

/// Error used when attempting to draw a card from an empty card collection.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CannotDrawFromEmpty;

/// The ability to draw cards from this collection of cards.
#[automock]
pub trait DrawFrom<C>
where
    C: CardLike,
{
    /// Draw a card from this collection.
    fn draw(&mut self) -> Option<C>;
}

/// The ability to draw from a drawable collection of cards.
#[automock]
pub trait DrawTo<C, D>
where
    C: CardLike,
    D: DrawFrom<C>,
{
    /// Draw a card from a drawable collection of cards into this.
    fn draw_from(&mut self, cards: &mut D) -> Result<(), CannotDrawFromEmpty>;
}
