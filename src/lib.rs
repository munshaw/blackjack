/// Enums and traits for blackjack user interfaces.
pub mod interface;

/// Traits for drawing to/from a collection of cards.
pub mod draw;

/// Trait for scoring a hand.
pub mod score;

/// Trait for iterating through a collection of cards.
pub mod card_iter;

/// Trait for playing card like entities
pub mod card_like;

/// Implementation of single deck blackjack.
pub mod blackjack;

/// CUI implementation of the blackjack user interface.
pub mod cui;

/// Functionality related to playing cards.
pub mod card;

/// Functionality related to hands of playing cards.
pub mod hand;

/// Functionality related to decks of playing cards.
pub mod deck;
