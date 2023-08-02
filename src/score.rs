use mockall::automock;

/// The score of a blackjack hand.
/// Value::Points() returns the actual point value, and if it counts as soft.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Value {
    Bust,
    Blackjack,
    Points(u8, bool),
}

/// The ability to evaluate the blackjack value of cards.
#[automock]
pub trait Score {
    fn score(&self) -> Value;
}
