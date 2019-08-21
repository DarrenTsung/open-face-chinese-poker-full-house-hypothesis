mod card;
mod opt;
mod strategies;

use structopt::StructOpt;

use self::card::*;
use self::opt::*;
use self::strategies::Strategy;
use pbr::ProgressBar;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let opt = Opt::from_args();

    let mut rng = thread_rng();

    // Note that the starting hand can be hard-coded
    // as the probability for completing a full house
    // does not depend on the card values in the full house.
    let triple_value = 4;
    let other_cards_in_hand = [Card { suit: 3, value: 11 }, Card { suit: 4, value: 8 }];
    let mut revealed_cards = RevealedCards::new();
    let mut remaining_deck = Card::remaining_deck(|card| {
        (card.value == 4 && card.suit != 1) // discard if was a triplet
        || (card.value == 11 && card.suit == 3) // discard if other card
        || (card.value == 8 && card.suit == 4) // discard if other card
    });

    let strategies = vec![&strategies::StartSetWithRandom];
    for strategy in strategies {
        let mut pb = ProgressBar::new(opt.runs as u64);

        let mut successful_runs = 0;
        for _ in 0..opt.runs {
            revealed_cards.reset();
            // The triple is revealed to the player
            revealed_cards.increment(4, 3);
            // The other cards are revealed
            revealed_cards.increment(11, 1);
            revealed_cards.increment(8, 1);

            remaining_deck.shuffle(&mut rng);

            let successful = strategy.run(
                triple_value,
                other_cards_in_hand,
                &mut revealed_cards,
                &remaining_deck,
            );
            if successful {
                successful_runs += 1;
            }
            pb.inc();
        }

        pb.finish_print(&format!(
            "{} - {} / {} ({}%)",
            strategy.name(),
            successful_runs,
            opt.runs,
            (successful_runs as f64) / (opt.runs as f64) * 100.0,
        ))
    }
}
