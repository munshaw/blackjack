use blackjack::card_iter::CardIter;
use blackjack::deck::Deck;
use blackjack::draw::{CannotDrawFromEmpty, DrawTo};
use blackjack::hand::Hand;
use blackjack::score::Score;
use blackjack::score::Value::Points;

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
