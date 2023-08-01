use blackjack::blackjack::Blackjack;
use blackjack::cui::Cui;
use blackjack::deck::Deck;

fn main() {
    Blackjack::new(&mut Cui::new(), &mut Deck::new()).start();
}
