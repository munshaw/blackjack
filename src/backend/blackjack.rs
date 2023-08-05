use crate::behaviour::card_iter::CardIter;
use crate::behaviour::card_like::CardLike;
use crate::behaviour::draw::{DrawFrom, DrawTo};
use crate::behaviour::interface::{Action, Event, Interface};
use crate::behaviour::score::{Score, Value};
use std::marker::PhantomData;

/// Represents a game of single deck blackjack.
pub struct Blackjack<'a, C, U, D, H>
where
    C: CardLike,
    U: Interface<H>,
    D: DrawFrom<C>,
    H: CardIter + Score + DrawTo<C, D>,
{
    _c: PhantomData<C>,
    ui: &'a mut U,
    deck: &'a mut D,
    player_hand: &'a mut H,
    dealer_hand: &'a mut H,
}

impl<'a, C, U, D, H> Blackjack<'a, C, U, D, H>
where
    C: CardLike,
    U: Interface<H>,
    D: DrawFrom<C>,
    H: CardIter + Score + DrawTo<C, D>,
{
    /// Create a new game of single player blackjack.
    pub fn new(
        ui: &'a mut U,
        deck: &'a mut D,
        player_hand: &'a mut H,
        dealer_hand: &'a mut H,
    ) -> Blackjack<'a, C, U, D, H> {
        Blackjack {
            _c: Default::default(),
            ui,
            deck,
            player_hand,
            dealer_hand,
        }
    }

    /// Start a game of single player blackjack.
    pub fn start(&mut self) {
        let player_score = self.player_turn();
        let dealer_score = self.dealer_turn();
        self.determine_winner(player_score, dealer_score);
    }

    /// Make the player have their turn.
    fn player_turn(&mut self) -> Value {
        self.player_hand
            .draw_from(self.deck)
            .expect("Can't draw from empty deck.");
        self.ui.send(Event::PlayerHand(&self.player_hand));
        let score = self.player_hand.score();
        match score {
            Value::Bust => self.ui.send(Event::PlayerBust),
            Value::Blackjack => self.ui.send(Event::PlayerBlackjack),
            _ if Action::PlayerHit == self.ui.get_action() => return self.player_turn(),
            _ => {}
        };
        score
    }

    fn is_dealer_hitting(&self, points: u8, is_soft: bool) -> bool {
        points < 17 || is_soft && points == 17
    }

    /// Make the dealer have their turn.
    fn dealer_turn(&mut self) -> Value {
        loop {
            self.dealer_hand
                .draw_from(self.deck)
                .expect("Can't draw from empty deck.");
            self.ui.send(Event::DealerHand(&self.dealer_hand));
            let score = self.dealer_hand.score();
            match score {
                Value::Bust => self.ui.send(Event::DealerBust),
                Value::Blackjack => self.ui.send(Event::DealerBlackjack),
                Value::Points(s, v) if self.is_dealer_hitting(s, v) => {
                    self.ui.send(Event::DealerHit);
                    continue;
                }
                _ => self.ui.send(Event::DealerStay),
            }
            return score;
        }
    }

    /// Determine the winner of the game.
    fn determine_winner(&mut self, player_score: Value, dealer_score: Value) {
        match (player_score, dealer_score) {
            (p, d) if p == d => self.ui.send(Event::Tie),
            (Value::Blackjack, _) => self.ui.send(Event::PlayerWin),
            (_, Value::Blackjack) => self.ui.send(Event::PlayerLoose),
            (Value::Bust, _) => self.ui.send(Event::PlayerLoose),
            (_, Value::Bust) => self.ui.send(Event::PlayerWin),
            (Value::Points(p, _), Value::Points(d, _)) if p > d => self.ui.send(Event::PlayerWin),
            (Value::Points(p, _), Value::Points(d, _)) if p < d => self.ui.send(Event::PlayerLoose),
            _ => unreachable!(),
        }
    }
}
