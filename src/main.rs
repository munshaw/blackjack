use blackjack::card::Rank;
use blackjack::card_iter::CardIter;
use blackjack::cui::Cui;
use blackjack::deck::Deck;
use blackjack::draw::{DrawFrom, DrawTo};
use blackjack::hand::Hand;
use blackjack::interface::{Action, Event, Interface};
use blackjack::score::{Score, Value};

fn player_turn<UIT: Interface, DeckT: DrawFrom>(ui: &UIT, deck: &mut DeckT) -> Value {
    let mut hand = Hand::new();
    loop {
        hand.draw_from(deck).unwrap();
        ui.send(Event::PlayerHand(&hand));
        let score = hand.score();
        match score {
            Value::Bust => {
                ui.send(Event::PlayerBust);
                return score;
            }
            Value::Blackjack => {
                ui.send(Event::PlayerBlackjack);
                return score;
            }
            _ if Action::PlayerStay == ui.get_action() => return score,
            _ => {}
        };
    }
}

fn has_aces(cards: &Hand) -> bool {
    cards.iter().any(|c| c.rank == Rank::Ace)
}

fn dealer_turn<UIT: Interface, DeckT: DrawFrom>(ui: &UIT, deck: &mut DeckT) -> Value {
    let mut hand = Hand::new();
    loop {
        hand.draw_from(deck).unwrap();
        ui.send(Event::DealerHand(&hand));
        let score = hand.score();
        match score {
            Value::Bust => {
                ui.send(Event::DealerBust);
                return score;
            }
            Value::Blackjack => {
                ui.send(Event::DealerBlackjack);
                return score;
            }
            Value::Points(v) => {
                if v == 17 && has_aces(&hand) || v > 17 {
                    ui.send(Event::DealerStay);
                    return score;
                }
                ui.send(Event::DealerHit);
            }
        };
    }
}

fn determine_winner<UIT: Interface>(ui: &UIT, player_score: Value, dealer_score: Value) {
    match (player_score, dealer_score) {
        (p, d) if p == d => ui.send(Event::Tie),
        (Value::Blackjack, _) => ui.send(Event::PlayerWin),
        (_, Value::Blackjack) => ui.send(Event::PlayerLoose),
        (Value::Bust, _) => ui.send(Event::PlayerLoose),
        (_, Value::Bust) => ui.send(Event::PlayerWin),
        (Value::Points(p), Value::Points(d)) if p > d => ui.send(Event::PlayerWin),
        (Value::Points(p), Value::Points(d)) if p < d => ui.send(Event::PlayerLoose),
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
