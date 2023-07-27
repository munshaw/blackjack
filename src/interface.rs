/// Actions that the user may take on their turn.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    PlayerStay,
    PlayerHit
}

/// Game events displayable to the user.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Event {
    PlayerBust,
    PlayerBlackjack,
    DealerBust,
    DealerBlackjack,
    DealerStay,
    DealerHit,
    PlayerWin,
    PlayerLoose,
    Tie
}

/// A trait for user interfaces. Implement this to create a new GUI, CUI, etc.
pub trait Interface {
    /// Get the players turn action.
    fn get_player_action(&self) -> Action;

    /// Display an event to the player.
    fn send_event(&self, event: Event);
}