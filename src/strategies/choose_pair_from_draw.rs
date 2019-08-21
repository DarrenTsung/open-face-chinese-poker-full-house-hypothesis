use crate::*;

pub struct ChoosePairFromDraw {
    pub number_of_players: usize,
    /// If draws_to_transition_to_two_remaining is 1 then we only
    /// choose the first draw if it is 3-remaining, afterwards we choose
    /// if it is 2-remaining.
    pub draws_to_transition_to_two_remaining: usize,
}

impl Strategy for ChoosePairFromDraw {
    fn name(&self) -> String {
        format!(
            "ChoosePairFromDraw(players={}, draws={})",
            self.number_of_players,
            self.draws_to_transition_to_two_remaining,
        )
    }

    fn run(
        &self,
        _triple_value: usize,
        _other_cards_in_hand: [Card; 2],
        revealed_cards: &mut RevealedCards,
        shuffled_remaining_cards: &[Card],
    ) -> bool {
        let mut remaining_card_index = 0;

        // For other players, reveal cards
        for _ in 0..self.number_of_players - 1 {
            for i in remaining_card_index..remaining_card_index+5 {
                revealed_cards.increment(shuffled_remaining_cards[i].value, 1);
            }
            remaining_card_index += 5;
        }

        let mut chosen_pair_value = None;
        // Each player draws 8 cards
        for draw_index in 0..8 {
            for player_index in 0..self.number_of_players {
                let card = shuffled_remaining_cards[remaining_card_index];
                remaining_card_index += 1;

                if player_index == 0 {
                    if let Some(value) = chosen_pair_value {
                        if card.value == value {
                            return true;
                        }
                    } else {
                        let revealed_target = if draw_index >= self.draws_to_transition_to_two_remaining {
                            1
                        } else {
                            0
                        };
                        if revealed_cards.for_value(card.value) <= revealed_target {
                            chosen_pair_value = Some(card.value);
                        }
                    }
                }

                revealed_cards.increment(card.value, 1);
            }
        }

        false
    }
}
