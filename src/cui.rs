use crate::interface::{Action, Interface, Event};
use std::io::stdin;

/// Simple CUI interface for a blackjack game.
#[derive(Debug)]
pub struct Cui {

}

impl Cui {
    /// Create a new Cui.
    pub fn new() -> Cui {
        Cui { }
    }
}

impl Interface for Cui {
    fn get_player_action(&self) -> Action {
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

    fn send_event(&self, event: Event) {
        match event {
            Event::PlayerWin => println!("You win!"),
            Event::PlayerLoose => println!("The dealer has won!"),
            Event::Tie => println!("The game has ended in a draw."),
            Event::PlayerBust => println!("You bust!"),
            Event::PlayerBlackjack => println!("You blackjack!"),
            Event::DealerHit => println!("The dealer hits."),
            Event::DealerStay => println!("The dealer stays."),
            Event::DealerBust => println!("The dealer busts!"),
            Event::DealerBlackjack => println!("The dealer blackjacks!"),
        }
    }
}