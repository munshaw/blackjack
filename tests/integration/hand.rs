use blackjack::backend::deck::Deck;
use blackjack::backend::hand::Hand;
use blackjack::behaviour::card_iter::CardIter;
use blackjack::behaviour::draw::{CannotDrawFromEmpty, DrawTo};
use blackjack::behaviour::score::Score;
use blackjack::behaviour::score::Value::Points;

#[test]
fn draw_one_card() {
    let mut deck = Deck::new();
    let mut hand = Hand::new();

    assert_eq!(0, hand.iter().count());

    hand.draw_from(&mut deck).unwrap();

    assert_eq!(1, hand.iter().count());
}

#[test]
fn draw_all_cards() {
    let mut deck = Deck::new();
    let mut hand = Hand::new();

    assert_eq!(0, hand.iter().count());

    for i in 1..53 {
        hand.draw_from(&mut deck).unwrap();
        assert_eq!(i, hand.iter().count());
    }

    assert_eq!(Err(CannotDrawFromEmpty), hand.draw_from(&mut deck));
}

#[test]
fn score_one_card() {
    let mut deck = Deck::new();
    let mut hand = Hand::new();

    assert_eq!(0, hand.iter().count());

    hand.draw_from(&mut deck).unwrap();

    if let Points(points, _) = hand.score() {
        assert!(points > 0);
    } else {
        assert!(false)
    }
}

#[test]
fn iterate_through_five_cards() {
    let mut deck = Deck::new();
    let mut hand = Hand::new();

    for _ in 0..5 {
        hand.draw_from(&mut deck).unwrap();
    }

    assert_eq!(5, hand.iter().count())
}

#[test]
fn iterate_through_all_cards() {
    let mut deck = Deck::new();
    let mut hand = Hand::new();

    for _ in 0..52 {
        hand.draw_from(&mut deck).unwrap();
    }

    assert_eq!(52, hand.iter().count())
}
