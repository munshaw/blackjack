use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::io::stdin;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Score {
    Bust,
    Blackjack,
    Value(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
            rank: *rank,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}s", self.rank, self.suit)
    }
}

fn cards_to_string(cards: &Vec<Card>) -> String {
    cards.iter().fold(
        String::new(),
        |s, c| if s == "" { s } else { s + ", " } + &*c.to_string(),
    )
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

fn has_aces(cards: &Vec<Card>) -> bool {
    cards.iter().any(|c| c.rank == Rank::Ace)
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
        None => unreachable!(),
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

fn player_turn(deck: &mut Vec<Card>) -> Score {
    let mut player_cards: Vec<Card> = Vec::new();
    loop {
        draw(deck, &mut player_cards);
        println!("Your cards: {}", cards_to_string(&player_cards));
        let score = calculate_score(&mut player_cards);
        match score {
            Score::Bust => {
                println!("You bust!");
                return score;
            }
            Score::Blackjack => {
                println!("You blackjack!");
                return score;
            }
            Score::Value(_) if is_stay() => return score,
            _ => {}
        };
    }
}

fn dealer_turn(deck: &mut Vec<Card>) -> Score {
    let mut dealer_cards: Vec<Card> = Vec::new();
    loop {
        draw(deck, &mut dealer_cards);
        println!("Dealer's cards: {}", cards_to_string(&dealer_cards));
        let score = calculate_score(&mut dealer_cards);
        match score {
            Score::Bust => {
                println!("The dealer busts!");
                return score;
            }
            Score::Blackjack => {
                println!("The dealer blackjacks!");
                return score;
            }
            Score::Value(v) => {
                if v == 17 && has_aces(&dealer_cards) || v > 17 {
                    println!("The dealer stays.");
                    return score;
                }
                println!("The dealer hits.");
            }
        };
    }
}

fn determine_winner(player_score: Score, dealer_score: Score) {
    match (player_score, dealer_score) {
        (p, d) if p == d => println!("The game ended in a draw."),
        (Score::Blackjack, _) => println!("You win!"),
        (_, Score::Blackjack) => println!("The dealer wins!"),
        (Score::Bust, _) => println!("You lose!"),
        (_, Score::Bust) => println!("You win!"),
        (Score::Value(p), Score::Value(d)) if p > d => println!("You win!"),
        (Score::Value(p), Score::Value(d)) if p < d => println!("The dealer wins!"),
        _ => unreachable!(),
    }
}

fn main() {
    let mut deck = new_deck();
    let player_score = player_turn(&mut deck);
    let dealer_score = dealer_turn(&mut deck);
    determine_winner(player_score, dealer_score);
}
