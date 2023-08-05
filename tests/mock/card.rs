use blackjack::card::{Rank, Suit};
use blackjack::card_like::MockCardLike;

pub fn mock_card(card: &(Rank, Suit)) -> MockCardLike {
    let mut mock_card = MockCardLike::new();
    mock_card.expect_get_rank().return_const(card.0);
    mock_card.expect_get_suit().return_const(card.1);
    mock_card
}
