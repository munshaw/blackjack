use blackjack::card::Card;
use blackjack::deck::Deck;
use blackjack::draw::DrawFrom;
use std::collections::HashSet;

#[test]
fn has_52_cards() {
    let mut deck = Deck::new();
    let mut size = 0;
    while deck.draw() != None {
        size += 1;
    }
    assert_eq!(size, 52);
}

#[test]
fn has_unique_cards() {
    let mut deck = Deck::new();
    let mut cards: HashSet<Card> = HashSet::new();
    loop {
        if let Some(card) = deck.draw() {
            assert!(!cards.contains(&card));
            cards.insert(card);
        } else {
            break;
        }
    }
}
