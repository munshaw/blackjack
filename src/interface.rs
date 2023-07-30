use crate::card_like::CardLike;
use crate::hand::Hand;
use mockall::automock;
use std::fmt::Display;

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
    C: CardLike + Display,
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
#[automock]
pub trait Interface<C>
where
    C: CardLike + Display,
{
    /// Get the player’s turn action.
    fn get_action(&self) -> Action;

    /// Display an event to the player.
    fn send<'a>(&self, event: Event<'a, C>);
}
