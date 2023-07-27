use blackjack::card::{Card, Rank, Suit};
use blackjack::cui::Cui;
use blackjack::interface::{Action, Event, Interface};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Score {
    Bust,
    Blackjack,
    Value(u8),
}

fn cards_to_string(cards: &Vec<Card>) -> String {
    cards.iter().fold(
        String::new(),
        |s, c| if s == "" { s } else { s + ", " } + &*c.to_string(),
    )
}

fn new_deck() -> Vec<Card> {
    let suits = [Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];
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

fn player_turn<UIT: Interface>(ui: &UIT, deck: &mut Vec<Card>) -> Score {
    let mut player_cards: Vec<Card> = Vec::new();
    loop {
        draw(deck, &mut player_cards);
        println!("Your cards: {}", cards_to_string(&player_cards));
        let score = calculate_score(&mut player_cards);
        match score {
            Score::Bust => {
                ui.send_event(Event::PlayerBust);
                return score;
            }
            Score::Blackjack => {
                ui.send_event(Event::PlayerBlackjack);
                return score;
            }
            Score::Value(_) if Action::PlayerStay == ui.get_player_action() => return score,
            _ => {}
        };
    }
}

fn dealer_turn<UIT: Interface>(ui: &UIT, deck: &mut Vec<Card>) -> Score {
    let mut dealer_cards: Vec<Card> = Vec::new();
    loop {
        draw(deck, &mut dealer_cards);
        println!("Dealer's cards: {}", cards_to_string(&dealer_cards));
        let score = calculate_score(&mut dealer_cards);
        match score {
            Score::Bust => {
                ui.send_event(Event::DealerBust);
                return score;
            }
            Score::Blackjack => {
                ui.send_event(Event::DealerBlackjack);
                return score;
            }
            Score::Value(v) => {
                if v == 17 && has_aces(&dealer_cards) || v > 17 {
                    ui.send_event(Event::DealerStay);
                    return score;
                }
                ui.send_event(Event::DealerHit);
            }
        };
    }
}

fn determine_winner<UIT: Interface>(ui: &UIT, player_score: Score, dealer_score: Score) {
    match (player_score, dealer_score) {
        (p, d) if p == d => ui.send_event(Event::Tie),
        (Score::Blackjack, _) => ui.send_event(Event::PlayerWin),
        (_, Score::Blackjack) => ui.send_event(Event::PlayerLoose),
        (Score::Bust, _) => ui.send_event(Event::PlayerLoose),
        (_, Score::Bust) => ui.send_event(Event::PlayerWin),
        (Score::Value(p), Score::Value(d)) if p > d => ui.send_event(Event::PlayerWin),
        (Score::Value(p), Score::Value(d)) if p < d => ui.send_event(Event::PlayerLoose),
        _ => unreachable!(),
    }
}

fn main() {
    let ui = Cui::new();
    let mut deck = new_deck();
    let player_score = player_turn(&ui, &mut deck);
    let dealer_score = dealer_turn(&ui, &mut deck);
    determine_winner(&ui, player_score, dealer_score);
}
