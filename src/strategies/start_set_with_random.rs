use crate::*;

pub struct StartSetWithRandom {
    pub player_index: usize
}

impl Strategy for StartSetWithRandom {
    fn name(&self) -> String {
        format!("StartSetWithRandom(player_index={})", self.player_index)
    }

    fn run(
        &self,
        _triple_value: usize,
        other_cards_in_hand: [Card; 2],
        revealed_cards: &mut RevealedCards,
        shuffled_remaining_cards: &[Card],
    ) -> bool {
        // Reveal cards for players before
        let cards_to_reveal = self.player_index * 5;
        for card in shuffled_remaining_cards.iter().take(cards_to_reveal) {
            revealed_cards.increment(card.value, 1);
        }

        // Choose highest-probability to pair card
        let card_0_value = other_cards_in_hand[0].value;
        let card_1_value = other_cards_in_hand[1].value;

        // Less revealed cards == higher probability to pair
        let chosen_pair_value = if revealed_cards.for_value(card_0_value) > revealed_cards.for_value(card_1_value) {
            card_1_value
        } else {
            card_0_value
        };

        // Each player draws 8 random cards
        for card in shuffled_remaining_cards.iter().skip(cards_to_reveal).take(8) {
            if chosen_pair_value == card.value {
                return true;
            }
        }

        false
    }
}
