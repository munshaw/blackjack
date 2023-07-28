use crate::hand::Hand;

/// Actions that the user may take on their turn.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    PlayerStay,
    PlayerHit,
}

/// Game events displayable to the user.
#[derive(Debug)]
pub enum Event<'a> {
    PlayerBust,
    PlayerBlackjack,
    DealerBust,
    DealerBlackjack,
    DealerStay,
    DealerHit,
    PlayerWin,
    PlayerLoose,
    Tie,
    PlayerHand(&'a Hand),
    DealerHand(&'a Hand),
}

/// A trait for user interfaces. Implement this to create a new GUI, CUI, etc.
pub trait Interface {
    /// Get the player's turn action.
    fn get_action(&self) -> Action;

    /// Display an event to the player.
    fn send(&self, event: Event);
}
