use crate::card_like::CardLike;
use crate::hand::Hand;

/// Actions that the user may take on their turn.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    PlayerStay,
    PlayerHit,
}

/// Game events displayable to the user.
#[derive(Debug)]
pub enum Event<'a, C>
where
    C: CardLike,
{
    PlayerBust,
    PlayerBlackjack,
    DealerBust,
    DealerBlackjack,
    DealerStay,
    DealerHit,
    PlayerWin,
    PlayerLoose,
    Tie,
    PlayerHand(&'a Hand<C>),
    DealerHand(&'a Hand<C>),
}

/// A trait for user interfaces. Implement this to create a new GUI, CUI, etc.
pub trait Interface<C>
where
    C: CardLike,
{
    /// Get the player’s turn action.
    fn get_action(&mut self) -> Action;

    /// Display an event to the player.
    fn send(&mut self, event: Event<C>);
}
