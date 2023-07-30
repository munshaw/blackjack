use blackjack::card::{Card, Rank, Suit};
use blackjack::card_like::CardLike;

#[test]
fn get_rank() {
    [
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
    ]
    .iter()
    .for_each(|r| get_rank_case(*r))
}

fn get_rank_case(rank: Rank) {
    let card = Card::new(Suit::Club, rank);
    let actual_rank = card.get_rank();
    assert_eq!(rank, actual_rank)
}

#[test]
fn get_suit() {
    [Suit::Spade, Suit::Heart, Suit::Club, Suit::Heart]
        .iter()
        .for_each(|s| get_suit_case(*s))
}

fn get_suit_case(suit: Suit) {
    let card = Card::new(suit, Rank::Ten);
    let actual_suit = card.get_suit();
    assert_eq!(suit, actual_suit)
}
