use crate::card::Rank;
use crate::card_iter::CardIter;
use crate::card_like::CardLike;
use crate::draw::{DrawFrom, DrawTo};
use crate::interface::{Action, Event, Interface};
use crate::score::{Score, Value};
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
        loop {
            self.player_hand
                .draw_from(self.deck)
                .expect("Cannot draw from empty deck.");
            self.ui.send(Event::PlayerHand(&self.player_hand));
            let score = self.player_hand.score();
            match score {
                Value::Bust => {
                    self.ui.send(Event::PlayerBust);
                    return score;
                }
                Value::Blackjack => {
                    self.ui.send(Event::PlayerBlackjack);
                    return score;
                }
                _ if Action::PlayerStay == self.ui.get_action() => return score,
                _ => {}
            };
        }
    }

    fn has_aces(cards: &H) -> bool {
        cards.iter().any(|c| c.get_rank() == Rank::Ace)
    }

    fn is_dealer_staying(&self, points: u8, hand: &H) -> bool {
        points == 17 && !Self::has_aces(&hand) || points > 17
    }

    /// Make the dealer have their turn.
    fn dealer_turn(&mut self) -> Value {
        loop {
            self.dealer_hand
                .draw_from(self.deck)
                .expect("Cannot draw from empty deck.");
            self.ui.send(Event::DealerHand(&self.dealer_hand));
            let score = self.dealer_hand.score();
            match score {
                Value::Bust => {
                    self.ui.send(Event::DealerBust);
                    return score;
                }
                Value::Blackjack => {
                    self.ui.send(Event::DealerBlackjack);
                    return score;
                }
                Value::Points(v) => {
                    if self.is_dealer_staying(v, &self.dealer_hand) {
                        self.ui.send(Event::DealerStay);
                        return score;
                    }
                    self.ui.send(Event::DealerHit);
                }
            };
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
            (Value::Points(p), Value::Points(d)) if p > d => self.ui.send(Event::PlayerWin),
            (Value::Points(p), Value::Points(d)) if p < d => self.ui.send(Event::PlayerLoose),
            _ => unreachable!(),
        }
    }
}
