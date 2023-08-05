use crate::fake::hand::FakeHand;
use crate::mock::deck::mock_deck;
use blackjack::blackjack::Blackjack;
use blackjack::card::{Rank, Suit};
use blackjack::card_iter::CardIter;
use blackjack::card_like::CardLike;
use blackjack::interface::{Action, Event, Interface};

struct MockInterface {
    player_actions: Vec<Action>,
    player_bust: usize,
    player_blackjack: usize,
    dealer_bust: usize,
    dealer_blackjack: usize,
    dealer_stay: usize,
    dealer_hit: usize,
    player_win: usize,
    player_loose: usize,
    tie: usize,
    player_hands: Vec<Vec<(Rank, Suit)>>,
    dealer_hands: Vec<Vec<(Rank, Suit)>>,
}

impl MockInterface {
    fn new() -> MockInterface {
        MockInterface {
            player_actions: vec![],
            player_bust: 0,
            player_blackjack: 0,
            dealer_bust: 0,
            dealer_blackjack: 0,
            dealer_stay: 0,
            dealer_hit: 0,
            player_win: 0,
            player_loose: 0,
            tie: 0,
            player_hands: vec![],
            dealer_hands: vec![],
        }
    }

    fn set_player_actions(&mut self, actions: Vec<Action>) {
        self.player_actions = actions;
        self.player_actions.reverse();
    }

    fn verify_player_bust_times(&self, times: usize) {
        assert_eq!(times, self.player_bust)
    }

    fn verify_player_blackjack_times(&self, times: usize) {
        assert_eq!(times, self.player_blackjack)
    }

    fn verify_dealer_bust_times(&self, times: usize) {
        assert_eq!(times, self.dealer_bust)
    }

    fn verify_dealer_blackjack_times(&self, times: usize) {
        assert_eq!(times, self.dealer_blackjack)
    }

    fn verify_dealer_stay_times(&self, times: usize) {
        assert_eq!(times, self.dealer_stay)
    }

    fn verify_dealer_hit_times(&self, times: usize) {
        assert_eq!(times, self.dealer_hit)
    }

    fn verify_player_win_times(&self, times: usize) {
        assert_eq!(times, self.player_win)
    }

    fn verify_player_loose_times(&self, times: usize) {
        assert_eq!(times, self.player_loose)
    }

    fn verify_tie_times(&self, times: usize) {
        assert_eq!(times, self.tie)
    }

    fn verify_player_hands(&self, hands: Vec<Vec<(Rank, Suit)>>) {
        assert_eq!(self.player_hands, hands)
    }

    fn verify_dealer_hands(&self, hands: Vec<Vec<(Rank, Suit)>>) {
        assert_eq!(self.dealer_hands, hands)
    }
}

impl Interface<FakeHand> for MockInterface {
    fn get_action(&mut self) -> Action {
        self.player_actions.pop().unwrap()
    }

    fn send(&mut self, event: Event<FakeHand>) {
        match event {
            Event::PlayerBust => self.player_bust += 1,
            Event::PlayerBlackjack => self.player_blackjack += 1,
            Event::DealerBust => self.dealer_bust += 1,
            Event::DealerBlackjack => self.dealer_blackjack += 1,
            Event::DealerStay => self.dealer_stay += 1,
            Event::DealerHit => self.dealer_hit += 1,
            Event::PlayerWin => self.player_win += 1,
            Event::PlayerLoose => self.player_loose += 1,
            Event::Tie => self.tie += 1,
            Event::PlayerHand(hand) => self
                .player_hands
                .push(hand.iter().map(|c| (c.get_rank(), c.get_suit())).collect()),
            Event::DealerHand(hand) => self
                .dealer_hands
                .push(hand.iter().map(|c| (c.get_rank(), c.get_suit())).collect()),
        }
    }
}

#[test]
fn player_six_five_nine_stay_dealer_two_six_jack() {
    let card1 = (Rank::Six, Suit::Diamond);
    let card2 = (Rank::Five, Suit::Club);
    let card3 = (Rank::Nine, Suit::Spade);
    let card4 = (Rank::Two, Suit::Heart);
    let card5 = (Rank::Six, Suit::Diamond);
    let card6 = (Rank::Jack, Suit::Diamond);
    let actions = vec![Action::PlayerHit, Action::PlayerHit, Action::PlayerStay];
    let mut ui = MockInterface::new();
    ui.set_player_actions(actions);
    let mut deck = mock_deck(vec![card1, card2, card3, card4, card5, card6]);
    Blackjack::new(
        &mut ui,
        &mut deck,
        &mut FakeHand::new(),
        &mut FakeHand::new(),
    )
    .start();
    ui.verify_player_hands(vec![
        vec![card1],
        vec![card1, card2],
        vec![card1, card2, card3],
    ]);
    ui.verify_dealer_hands(vec![
        vec![card4],
        vec![card4, card5],
        vec![card4, card5, card6],
    ]);
    ui.verify_player_bust_times(0);
    ui.verify_player_blackjack_times(0);
    ui.verify_dealer_bust_times(0);
    ui.verify_dealer_blackjack_times(0);
    ui.verify_dealer_stay_times(1);
    ui.verify_dealer_hit_times(2);
    ui.verify_player_win_times(1);
    ui.verify_player_loose_times(0);
    ui.verify_tie_times(0);
}

#[test]
fn player_king_seven_ten_bust_dealer_ace_jack() {
    let card1 = (Rank::King, Suit::Diamond);
    let card2 = (Rank::Seven, Suit::Club);
    let card3 = (Rank::Ten, Suit::Spade);
    let card4 = (Rank::Ace, Suit::Heart);
    let card5 = (Rank::Jack, Suit::Diamond);
    let actions = vec![Action::PlayerHit, Action::PlayerHit];
    let mut ui = MockInterface::new();
    ui.set_player_actions(actions);
    let mut deck = mock_deck(vec![card1, card2, card3, card4, card5]);
    Blackjack::new(
        &mut ui,
        &mut deck,
        &mut FakeHand::new(),
        &mut FakeHand::new(),
    )
    .start();
    ui.verify_player_hands(vec![
        vec![card1],
        vec![card1, card2],
        vec![card1, card2, card3],
    ]);
    ui.verify_dealer_hands(vec![vec![card4], vec![card4, card5]]);
    ui.verify_player_bust_times(1);
    ui.verify_player_blackjack_times(0);
    ui.verify_dealer_bust_times(0);
    ui.verify_dealer_blackjack_times(1);
    ui.verify_dealer_stay_times(0);
    ui.verify_dealer_hit_times(1);
    ui.verify_player_win_times(0);
    ui.verify_player_loose_times(1);
    ui.verify_tie_times(0);
}

#[test]
fn player_king_seven_four_blackjack_dealer_six_eight_jack() {
    let card1 = (Rank::King, Suit::Diamond);
    let card2 = (Rank::Seven, Suit::Club);
    let card3 = (Rank::Four, Suit::Club);
    let card4 = (Rank::Six, Suit::Spade);
    let card5 = (Rank::Eight, Suit::Heart);
    let card6 = (Rank::Jack, Suit::Diamond);
    let actions = vec![Action::PlayerHit, Action::PlayerHit];
    let mut ui = MockInterface::new();
    ui.set_player_actions(actions);
    let mut deck = mock_deck(vec![card1, card2, card3, card4, card5, card6]);
    Blackjack::new(
        &mut ui,
        &mut deck,
        &mut FakeHand::new(),
        &mut FakeHand::new(),
    )
    .start();
    ui.verify_player_hands(vec![
        vec![card1],
        vec![card1, card2],
        vec![card1, card2, card3],
    ]);
    ui.verify_dealer_hands(vec![
        vec![card4],
        vec![card4, card5],
        vec![card4, card5, card6],
    ]);
    ui.verify_player_bust_times(0);
    ui.verify_player_blackjack_times(1);
    ui.verify_dealer_bust_times(1);
    ui.verify_dealer_blackjack_times(0);
    ui.verify_dealer_stay_times(0);
    ui.verify_dealer_hit_times(2);
    ui.verify_player_win_times(1);
    ui.verify_player_loose_times(0);
    ui.verify_tie_times(0);
}

#[test]
fn player_ace_king_blackjack_dealer_ace_king() {
    let card1 = (Rank::Ace, Suit::Diamond);
    let card2 = (Rank::King, Suit::Club);
    let card3 = (Rank::Ace, Suit::Club);
    let card4 = (Rank::King, Suit::Spade);
    let actions = vec![Action::PlayerHit, Action::PlayerHit];
    let mut ui = MockInterface::new();
    ui.set_player_actions(actions);
    let mut deck = mock_deck(vec![card1, card2, card3, card4]);
    Blackjack::new(
        &mut ui,
        &mut deck,
        &mut FakeHand::new(),
        &mut FakeHand::new(),
    )
    .start();
    ui.verify_player_hands(vec![vec![card1], vec![card1, card2]]);
    ui.verify_dealer_hands(vec![vec![card3], vec![card3, card4]]);
    ui.verify_player_bust_times(0);
    ui.verify_player_blackjack_times(1);
    ui.verify_dealer_bust_times(0);
    ui.verify_dealer_blackjack_times(1);
    ui.verify_dealer_stay_times(0);
    ui.verify_dealer_hit_times(1);
    ui.verify_player_win_times(0);
    ui.verify_player_loose_times(0);
    ui.verify_tie_times(1);
}

#[test]
fn player_ace_nine_king_four_dealer_ace_six_queen() {
    let card1 = (Rank::Ace, Suit::Diamond);
    let card2 = (Rank::Nine, Suit::Club);
    let card3 = (Rank::King, Suit::Spade);
    let card4 = (Rank::Four, Suit::Heart);
    let card5 = (Rank::Ace, Suit::Heart);
    let card6 = (Rank::Six, Suit::Diamond);
    let card7 = (Rank::Queen, Suit::Diamond);
    let actions = vec![Action::PlayerHit, Action::PlayerHit, Action::PlayerHit];
    let mut ui = MockInterface::new();
    ui.set_player_actions(actions);
    let mut deck = mock_deck(vec![card1, card2, card3, card4, card5, card6, card7]);
    Blackjack::new(
        &mut ui,
        &mut deck,
        &mut FakeHand::new(),
        &mut FakeHand::new(),
    )
    .start();
    ui.verify_player_hands(vec![
        vec![card1],
        vec![card1, card2],
        vec![card1, card2, card3],
        vec![card1, card2, card3, card4],
    ]);
    ui.verify_dealer_hands(vec![
        vec![card5],
        vec![card5, card6],
        vec![card5, card6, card7],
    ]);
    ui.verify_player_bust_times(1);
    ui.verify_player_blackjack_times(0);
    ui.verify_dealer_bust_times(0);
    ui.verify_dealer_blackjack_times(0);
    ui.verify_dealer_stay_times(1);
    ui.verify_dealer_hit_times(2);
    ui.verify_player_win_times(0);
    ui.verify_player_loose_times(1);
    ui.verify_tie_times(0);
}

#[test]
fn player_seven_ten_stay_dealer_six_ten_nine() {
    let card1 = (Rank::Seven, Suit::Diamond);
    let card2 = (Rank::Ten, Suit::Club);
    let card3 = (Rank::Six, Suit::Diamond);
    let card4 = (Rank::Ten, Suit::Club);
    let card5 = (Rank::Nine, Suit::Spade);
    let actions = vec![Action::PlayerHit, Action::PlayerStay];
    let mut ui = MockInterface::new();
    ui.set_player_actions(actions);
    let mut deck = mock_deck(vec![card1, card2, card3, card4, card5]);
    Blackjack::new(
        &mut ui,
        &mut deck,
        &mut FakeHand::new(),
        &mut FakeHand::new(),
    )
    .start();
    ui.verify_player_hands(vec![vec![card1], vec![card1, card2]]);
    ui.verify_dealer_hands(vec![
        vec![card3],
        vec![card3, card4],
        vec![card3, card4, card5],
    ]);
    ui.verify_player_bust_times(0);
    ui.verify_player_blackjack_times(0);
    ui.verify_dealer_bust_times(1);
    ui.verify_dealer_blackjack_times(0);
    ui.verify_dealer_stay_times(0);
    ui.verify_dealer_hit_times(2);
    ui.verify_player_win_times(1);
    ui.verify_player_loose_times(0);
    ui.verify_tie_times(0);
}

#[test]
fn player_two_ten_stay_dealer_two_six_jack() {
    let card1 = (Rank::Two, Suit::Diamond);
    let card2 = (Rank::Six, Suit::Club);
    let card3 = (Rank::Two, Suit::Heart);
    let card4 = (Rank::Six, Suit::Diamond);
    let card5 = (Rank::Jack, Suit::Diamond);
    let actions = vec![Action::PlayerHit, Action::PlayerStay];
    let mut ui = MockInterface::new();
    ui.set_player_actions(actions);
    let mut deck = mock_deck(vec![card1, card2, card3, card4, card5]);
    Blackjack::new(
        &mut ui,
        &mut deck,
        &mut FakeHand::new(),
        &mut FakeHand::new(),
    )
    .start();
    ui.verify_player_hands(vec![vec![card1], vec![card1, card2]]);
    ui.verify_dealer_hands(vec![
        vec![card3],
        vec![card3, card4],
        vec![card3, card4, card5],
    ]);
    ui.verify_player_bust_times(0);
    ui.verify_player_blackjack_times(0);
    ui.verify_dealer_bust_times(0);
    ui.verify_dealer_blackjack_times(0);
    ui.verify_dealer_stay_times(1);
    ui.verify_dealer_hit_times(2);
    ui.verify_player_win_times(0);
    ui.verify_player_loose_times(1);
    ui.verify_tie_times(0);
}
