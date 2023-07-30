use blackjack::blackjack::Blackjack;
use blackjack::cui::Cui;
use blackjack::deck::Deck;

fn main() {
    Blackjack::new(&Cui::new(), &mut Deck::new()).start();
}
