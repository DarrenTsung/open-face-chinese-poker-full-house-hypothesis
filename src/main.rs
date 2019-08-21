mod card;
mod opt;
mod strategies;

use structopt::StructOpt;

use self::card::*;
use self::opt::*;
use self::strategies::Strategy;
use antidote::Mutex;
use pbr::ProgressBar;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::Arc;

fn main() {
    let opt = Opt::from_args();

    // Note that the starting hand can be hard-coded
    // as the probability for completing a full house
    // does not depend on the card values in the full house.
    let triple_value = 4;
    let other_cards_in_hand = [Card { suit: 3, value: 11 }, Card { suit: 4, value: 8 }];

    let mut strategies: Vec<Arc<Strategy>> = vec![Arc::new(strategies::StartSetWithRandom)];

    for index in 0..4 {
        strategies.push(Arc::new(strategies::ChoosePairFromDraw {
            number_of_players: 4,
            draws_to_transition_to_two_remaining: index,
        }));
    }

    let runs = (0..opt.runs).collect::<Vec<_>>();
    for strategy in strategies {
        println!("");
        let pb = Arc::new(Mutex::new(ProgressBar::new(opt.runs as u64)));

        let successful_runs: usize = runs
            .par_iter()
            .map(|_| {
                let mut revealed_cards = RevealedCards::new();
                // The triple is revealed to the player
                revealed_cards.increment(4, 3);
                // The other cards are revealed
                revealed_cards.increment(11, 1);
                revealed_cards.increment(8, 1);

                let mut remaining_deck = Card::remaining_deck(|card| {
                    (card.value == 4 && card.suit != 1) // discard if was a triplet
                    || (card.value == 11 && card.suit == 3) // discard if other card
                    || (card.value == 8 && card.suit == 4) // discard if other card
                });
                remaining_deck.shuffle(&mut thread_rng());

                let successful = strategy.run(
                    triple_value,
                    other_cards_in_hand,
                    &mut revealed_cards,
                    &remaining_deck,
                );
                pb.lock().inc();

                if successful {
                    1
                } else {
                    0
                }
            })
            .sum();

        pb.lock().finish_print(&format!(
            "{} - {} / {} ({}%)",
            strategy.name(),
            successful_runs,
            opt.runs,
            (successful_runs as f64) / (opt.runs as f64) * 100.0,
        ));
    }
}
