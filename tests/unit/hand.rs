use blackjack::card::{Rank, Suit};
use blackjack::card_iter::CardIter;
use blackjack::card_like::{CardLike, MockCardLike};
use blackjack::draw::{CannotDrawFromEmpty, DrawTo, MockDrawFrom};
use blackjack::hand::Hand;
use blackjack::score::{Score, Value};
use std::collections::HashSet;

fn mock_card(card: &(Rank, Suit)) -> MockCardLike {
    let mut mock_card = MockCardLike::new();
    mock_card.expect_get_rank().return_const(card.0);
    mock_card.expect_get_suit().return_const(card.1);
    mock_card
}

fn mock_deck(mut cards: Vec<MockCardLike>) -> MockDrawFrom<MockCardLike> {
    let mut deck = MockDrawFrom::new();
    deck.expect_draw().returning(move || cards.pop());
    deck
}

fn assert_one_new_card_in_hand(
    initial_cards: &HashSet<(Rank, Suit)>,
    last_hand: &mut HashSet<(Rank, Suit)>,
    actual_hand: &Hand<MockCardLike>,
) {
    let mut actual_cards = HashSet::new();
    for actual_card in actual_hand.iter() {
        actual_cards.insert((actual_card.get_rank(), actual_card.get_suit()));
    }
    let mut count = 0;
    for card in actual_cards {
        if initial_cards.contains(&card) && !last_hand.contains(&card) {
            count += 1;
            last_hand.insert(card);
        }
    }
    assert_eq!(1, count)
}

#[test]
fn draw_and_iter() {
    let initial_cards = HashSet::from([
        (Rank::King, Suit::Club),
        (Rank::Eight, Suit::Heart),
        (Rank::Four, Suit::Diamond),
    ]);
    let mut deck = mock_deck(initial_cards.iter().map(|c| mock_card(c)).collect());
    let mut last_hand = HashSet::new();
    let mut actual_hand = Hand::new();

    assert_eq!(0, actual_hand.iter().len());
    for _ in initial_cards.iter() {
        actual_hand.draw_from(&mut deck).unwrap();
        assert_one_new_card_in_hand(&initial_cards, &mut last_hand, &actual_hand);
    }

    assert_eq!(
        actual_hand.draw_from(&mut deck).unwrap_err(),
        CannotDrawFromEmpty
    )
}

fn score_case(cards: Vec<(Rank, Suit)>, expected_score: Value) {
    let mut deck = mock_deck(cards.iter().map(|c| mock_card(c)).collect());
    let mut hand = Hand::new();
    for _ in cards {
        hand.draw_from(&mut deck).unwrap();
    }
    assert_eq!(expected_score, hand.score())
}

#[test]
fn score_four_five_jack() {
    score_case(
        vec![
            (Rank::Four, Suit::Club),
            (Rank::Five, Suit::Heart),
            (Rank::Jack, Suit::Diamond),
        ],
        Value::Points(19),
    );
}

#[test]
fn score_three() {
    score_case(vec![(Rank::Three, Suit::Club)], Value::Points(3));
}

#[test]
fn score_ace_queen() {
    score_case(
        vec![(Rank::Ace, Suit::Club), (Rank::Queen, Suit::Spade)],
        Value::Blackjack,
    );
}

#[test]
fn score_king_six_five() {
    score_case(
        vec![
            (Rank::King, Suit::Club),
            (Rank::Six, Suit::Spade),
            (Rank::Five, Suit::Diamond),
        ],
        Value::Blackjack,
    );
}

#[test]
fn score_king_six_seven() {
    score_case(
        vec![
            (Rank::King, Suit::Club),
            (Rank::Six, Suit::Spade),
            (Rank::Seven, Suit::Diamond),
        ],
        Value::Bust,
    );
}
