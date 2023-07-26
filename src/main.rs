use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Copy, Clone)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
enum Suit {
    Spade,
    Heart,
    Clubs,
    Diamonds,
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn new(suit: &Suit, rank: &Rank) -> Card {
        Card {
            suit: *suit,
            rank: *rank
        }
    }
}

fn new_deck() -> Vec<Card> {
    let suits = [Suit::Spade, Suit::Heart, Suit::Clubs, Suit::Diamonds];
    let ranks = [Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven,
        Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King];
    let mut deck: Vec<Card> = Vec::new();
    suits.iter().for_each(|s| ranks.iter().for_each(|r| deck.push(Card::new(s, r))));
    deck.shuffle(&mut thread_rng());
    deck
}

fn main() {
    let deck = new_deck();

    println!("{:#?}", deck);
}
