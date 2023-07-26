use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io::stdin;

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

#[derive(Debug, Copy, Clone)]
enum Score {
    Bust,
    Blackjack,
    Value(u8),
}

impl Card {
    fn new(suit: &Suit, rank: &Rank) -> Card {
        Card {
            suit: *suit,
            rank: *rank,
        }
    }
}

fn new_deck() -> Vec<Card> {
    let suits = [Suit::Spade, Suit::Heart, Suit::Clubs, Suit::Diamonds];
    let ranks = [
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
    let mut deck: Vec<Card> = Vec::new();
    suits
        .iter()
        .for_each(|s| ranks.iter().for_each(|r| deck.push(Card::new(s, r))));
    deck.shuffle(&mut thread_rng());
    deck
}

fn calculate_score(cards: &Vec<Card>) -> Score {
    let mut value = 0;
    let mut aces = 0;

    for card in cards {
        value += match card.rank {
            Rank::Ace => {
                aces += 1;
                11
            }
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
        };
    }

    // Try valuing aces at 1 to avoid busting.
    while value > 21 && aces > 0 {
        value -= 10;
        aces -= 1;
    }

    match value.cmp(&{ 21 }) {
        Ordering::Less => Score::Value(value),
        Ordering::Greater => Score::Bust,
        Ordering::Equal => Score::Blackjack,
    }
}

fn draw(deck: &mut Vec<Card>, cards: &mut Vec<Card>) {
    match deck.pop() {
        None => panic!(),
        Some(c) => cards.push(c),
    }
}

fn is_stay() -> bool {
    loop {
        println!("Would you like to (h)it or (s)tay?");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "h" {
            return false;
        } else if input.trim().to_lowercase() == "s" {
            return true;
        }
    }
}

fn player_turn(deck: &mut Vec<Card>, player_cards: &mut Vec<Card>) -> Score {
    loop {
        draw(deck, player_cards);
        println!("Your cards: {:#?}", player_cards);
        let score = calculate_score(player_cards);
        match score {
            Score::Bust => {
                println!("Bust!");
                return score;
            }
            Score::Blackjack => {
                println!("Blackjack!");
                return score;
            }
            Score::Value(v) => {
                println!("Your score: {}", v);
                if is_stay() {
                    return score;
                }
            }
        };
    }
}

fn main() {
    let mut deck = new_deck();
    let mut player_cards: Vec<Card> = Vec::new();
    player_turn(&mut deck, &mut player_cards);
}
