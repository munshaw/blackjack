use blackjack::backend::blackjack::Blackjack;
use blackjack::backend::deck::Deck;
use blackjack::backend::hand::Hand;
use blackjack::cui::Cui;

fn main() {
    Blackjack::new(
        &mut Cui::new(),
        &mut Deck::new(),
        &mut Hand::new(),
        &mut Hand::new(),
    )
    .start();
}
