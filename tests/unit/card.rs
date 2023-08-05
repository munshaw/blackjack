use blackjack::behaviour::card_like::CardLike;
use blackjack::card::{Card, Rank, Suit};

const RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

const SUITS: [Suit; 4] = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];

#[test]
fn get_rank() {
    RANKS.iter().for_each(|r| get_rank_case(*r))
}

fn get_rank_case(rank: Rank) {
    let card = Card::new(Suit::Club, rank);
    let actual_rank = card.get_rank();
    assert_eq!(rank, actual_rank)
}

#[test]
fn get_suit() {
    SUITS.iter().for_each(|s| get_suit_case(*s))
}

fn get_suit_case(suit: Suit) {
    let card = Card::new(suit, Rank::Ten);
    let actual_suit = card.get_suit();
    assert_eq!(suit, actual_suit)
}

#[test]
fn to_string() {
    RANKS
        .iter()
        .for_each(|r| SUITS.iter().for_each(|s| to_string_case(*s, *r)))
}

fn to_string_case(suit: Suit, rank: Rank) {
    let expected_string = format!("{:?} of {:?}s", rank, suit);
    let card = Card::new(suit, rank);
    let actual_string = card.to_string();
    assert_eq!(expected_string, actual_string)
}
