use crate::card_iter::CardIter;
use crate::interface::{Action, Event, Interface};
use std::fmt::Display;
use std::io::stdin;

/// Simple CUI interface for a blackjack game.
#[derive(Debug)]
pub struct Cui {}

impl Cui {
    /// Create a new Cui.
    pub fn new() -> Cui {
        Cui {}
    }
}

impl<H> Interface<H> for Cui
where
    H: CardIter + Display,
{
    fn get_action(self: &mut Cui) -> Action {
        loop {
            println!("Would you like to (h)it or (s)tay?");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "h" {
                return Action::PlayerHit;
            } else if input.trim().to_lowercase() == "s" {
                return Action::PlayerStay;
            }
        }
    }

    fn send(self: &mut Cui, event: Event<H>) {
        match event {
            Event::PlayerWin => println!("You win!"),
            Event::PlayerLoose => println!("The dealer wins!"),
            Event::Tie => println!("The game has ended in a draw."),
            Event::PlayerBust => println!("You bust!"),
            Event::PlayerBlackjack => println!("You blackjack!"),
            Event::DealerHit => println!("The dealer hits."),
            Event::DealerStay => println!("The dealer stays."),
            Event::DealerBust => println!("The dealer busts!"),
            Event::DealerBlackjack => println!("The dealer blackjacks!"),
            Event::PlayerHand(hand) => println!("Your cards: {}", hand),
            Event::DealerHand(hand) => println!("Dealers hand: {}", hand),
        }
    }
}
