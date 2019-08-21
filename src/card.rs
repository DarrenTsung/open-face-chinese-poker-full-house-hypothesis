#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: u8,
    pub value: u8
}

impl Card {
    pub fn remaining_deck(discard: impl Fn(Card) -> bool) -> [Card; 47] {
        let mut deck = [Card { suit: 0, value: 0 }; 47];
        let mut index = 0;
        for suit in 1..=4 {
            for value in 1..=13 {
                let card = Card { suit, value };
                if discard(card) {
                    continue;
                }

                deck[index] = card;
                index += 1;
            }
        }

        if index != 47 {
            panic!("Incorrect remaining deck created, should be 47 cards, got {}", index);
        }

        deck
    }
}

pub struct RevealedCards([usize; 13]);

impl RevealedCards {
    pub fn new() -> Self {
        Self([0;13])
    }

    pub fn for_value(&self, card_value: u8) -> usize {
        self.0[(card_value - 1) as usize]
    }

    pub fn increment(&mut self, card_value: u8, count: usize) {
        self.0[(card_value - 1) as usize] += count;
    }

    pub fn reset(&mut self) {
        for i in 0..13 {
            self.0[i] = 0;
        }
    }
}
