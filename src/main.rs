use blackjack::card::Rank;
use blackjack::cui::Cui;
use blackjack::deck::{Deck, Draw};
use blackjack::hand::{CardIter, DrawFrom, Hand, Score, Value};
use blackjack::interface::{Action, Event, Interface};

fn player_turn<UIT: Interface, DeckT: Draw>(ui: &UIT, deck: &mut DeckT) -> Value {
    let mut hand = Hand::new();
    loop {
        hand.draw_from(deck).unwrap();
        println!("Your cards: {}", &hand);
        let score = hand.score();
        match score {
            Value::Bust => {
                ui.send_event(Event::PlayerBust);
                return score;
            }
            Value::Blackjack => {
                ui.send_event(Event::PlayerBlackjack);
                return score;
            }
            _ if Action::PlayerStay == ui.get_player_action() => return score,
            _ => {}
        };
    }
}

fn has_aces(cards: &Hand) -> bool {
    cards.iter().any(|c| c.rank == Rank::Ace)
}

fn dealer_turn<UIT: Interface, DeckT: Draw>(ui: &UIT, deck: &mut DeckT) -> Value {
    let mut hand = Hand::new();
    loop {
        hand.draw_from(deck).unwrap();
        println!("Dealer's cards: {}", &hand);
        let score = hand.score();
        match score {
            Value::Bust => {
                ui.send_event(Event::DealerBust);
                return score;
            }
            Value::Blackjack => {
                ui.send_event(Event::DealerBlackjack);
                return score;
            }
            Value::Points(v) => {
                if v == 17 && has_aces(&hand) || v > 17 {
                    ui.send_event(Event::DealerStay);
                    return score;
                }
                ui.send_event(Event::DealerHit);
            }
        };
    }
}

fn determine_winner<UIT: Interface>(ui: &UIT, player_score: Value, dealer_score: Value) {
    match (player_score, dealer_score) {
        (p, d) if p == d => ui.send_event(Event::Tie),
        (Value::Blackjack, _) => ui.send_event(Event::PlayerWin),
        (_, Value::Blackjack) => ui.send_event(Event::PlayerLoose),
        (Value::Bust, _) => ui.send_event(Event::PlayerLoose),
        (_, Value::Bust) => ui.send_event(Event::PlayerWin),
        (Value::Points(p), Value::Points(d)) if p > d => ui.send_event(Event::PlayerWin),
        (Value::Points(p), Value::Points(d)) if p < d => ui.send_event(Event::PlayerLoose),
        _ => unreachable!(),
    }
}

fn main() {
    let ui = Cui::new();
    let mut deck = Deck::new();
    let player_score = player_turn(&ui, &mut deck);
    let dealer_score = dealer_turn(&ui, &mut deck);
    determine_winner(&ui, player_score, dealer_score);
}
