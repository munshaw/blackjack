use blackjack::blackjack::Blackjack;
use blackjack::cui::Cui;
use blackjack::deck::Deck;

fn main() {
    let ui = Cui::new();
    let mut deck = Deck::new();
    let mut blackjack = Blackjack::new(&ui, &mut deck);
    let player_score = blackjack.player_turn();
    let dealer_score = blackjack.dealer_turn();
    blackjack.determine_winner(player_score, dealer_score);
}
