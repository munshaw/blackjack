use blackjack::blackjack::Blackjack;
use blackjack::cui::Cui;
use blackjack::deck::Deck;
use blackjack::hand::Hand;

fn main() {
    Blackjack::new(
        &mut Cui::new(),
        &mut Deck::new(),
        &mut Hand::new(),
        &mut Hand::new(),
    )
    .start();
}
