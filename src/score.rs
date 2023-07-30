use mockall::automock;

/// The score of a blackjack hand.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Value {
    Bust,
    Blackjack,
    Points(u8),
}

/// The ability to evaluate the blackjack value of cards.
#[automock]
pub trait Score {
    fn score(&self) -> Value;
}
