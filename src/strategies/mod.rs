mod start_set_with_random;
mod choose_pair_from_draw;

pub use self::choose_pair_from_draw::*;
pub use self::start_set_with_random::*;

use crate::*;

pub trait Strategy: Send + Sync {
    /// Returns the display name of the strategy.
    fn name(&self) -> String;

    /// Returns whether a full-house was made or not.
    fn run(
        &self,
        triple_value: usize,
        other_cards_in_hand: [Card; 2],
        revealed_cards: &mut RevealedCards,
        shuffled_remaining_cards: &[Card],
    ) -> bool;
}
