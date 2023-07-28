use crate::card_like::CardLike;
use crate::hand::Hand;
use std::fmt::Display;

/// Actions that the user may take on their turn.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    PlayerStay,
    PlayerHit,
}

/// Game events displayable to the user.
#[derive(Debug)]
pub enum Event<'a, CardT: CardLike + Display> {
    PlayerBust,
    PlayerBlackjack,
    DealerBust,
    DealerBlackjack,
    DealerStay,
    DealerHit,
    PlayerWin,
    PlayerLoose,
    Tie,
    PlayerHand(&'a Hand<CardT>),
    DealerHand(&'a Hand<CardT>),
}

/// A trait for user interfaces. Implement this to create a new GUI, CUI, etc.
pub trait Interface<CardT: CardLike + Display> {
    /// Get the player’s turn action.
    fn get_action(&self) -> Action;

    /// Display an event to the player.
    fn send(&self, event: Event<CardT>);
}
