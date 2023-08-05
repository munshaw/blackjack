use crate::mock::card::mock_card;
use blackjack::backend::card::{Rank, Suit};
use blackjack::behaviour::card_like::MockCardLike;
use blackjack::behaviour::draw::MockDrawFrom;

pub fn mock_deck(cards: Vec<(Rank, Suit)>) -> MockDrawFrom<MockCardLike> {
    let mut deck = MockDrawFrom::new();
    let mut mock_cards: Vec<MockCardLike> = cards.iter().rev().map(|c| mock_card(c)).collect();
    deck.expect_draw().returning(move || mock_cards.pop());
    deck
}
