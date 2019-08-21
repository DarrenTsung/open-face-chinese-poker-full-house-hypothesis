# open-face-chinese-poker-full-house-hypothesis
In the game of open-face Chinese poker (https://www.pokernews.com/poker-rules/chinese-poker.htm), I would like to calculate the optimal strategy for completing a full-house on the bottom row from an starting hand which contains a triple and two non-matching cards.

For example, this starting hand (where `4S` = the four of spades):
```rust
4S 4H 4D JH 8C
```

The most straight-forward strategy would be to place down the triple as well as the highest-probability card to make a pair (based on information revealed by opponents cards).

For example, if there was already a Jack out on the field and no 8s, you would place this set of cards on the bottom row:
```rust
4S 4H 4D 8C // -> look for an 8 in a different suit
```

This has the benefit of requiring only drawing 1 card matching the card you placed down.

However, I believe it may make sense to wait to choose the card used for the pair based on the amount of information revealed on the board when you choose your set and the amount of information to be revealed after you begin drawing.

For example, the most extreme case of this would be when you're going first in a four-person game. You have no information about cards drawn when you reveal your hand, and you would know which 15 of the cards were drew by your opponents once you begin drawing.

Therefore, you might choose to place this set of cards on the bottom row:
```
4S 4H 4D
```

And wait until you draw a card from the deck that has either no matching cards on the board (3-remaining) or only one matching card on the board (2-remaining) to add to your hand.

Of course, there are other considerations in a real game based on how your middle / top row are being constructed. But, purely for optimizing the chance for hitting a full-house, how do these strategies perform?

## Running the experiment
Requires Rust installed (https://www.rust-lang.org/).

```bash
# Runs with default 1000 runs per strategy (high variance)
cargo run

# Run with release mode and 1,000,000 runs per strategy
cargo run --release -- --runs 1000000
```

The output for 1 million runs of each strategy implemented is:
```bash
# This strategy chooses the highest probability card to pair given
# the revealed cards due to the player_index.
#
# Ie. if player_index is 0, you choose your hand before any
# opponents reveal their cards.
StartSetWithRandom(player_index=0) - 436768 / 1000000 (43.6768%)
StartSetWithRandom(player_index=1) - 469654 / 1000000 (46.9654%)
StartSetWithRandom(player_index=2) - 492854 / 1000000 (49.2854%)
StartSetWithRandom(player_index=3) - 512512 / 1000000 (51.2512%)

# This strategy waits till the drawing stage in order to choose a card
# to pair. It starts out by only adding the card to the set if there
# are 3-remaining, but will choose a 2-remaining card if `draws` has passed.
#
# This strategy assumes you are the first player and therefore choose
# before any opponents reveal their cards. The later stage player you
# are, the worse this strategy is (due to less cards revealed at draw stage).
#
# This strategy does worse the fewer players there are (^due to less
# cards revealed at draw stage / drawing overall).
ChoosePairFromDraw(players=3, draws=0) - 413787 / 1000000 (41.3787%)
ChoosePairFromDraw(players=3, draws=1) - 421961 / 1000000 (42.196099999999994%)
ChoosePairFromDraw(players=3, draws=2) - 420498 / 1000000 (42.0498%)
ChoosePairFromDraw(players=4, draws=0) - 443900 / 1000000 (44.39%)
ChoosePairFromDraw(players=4, draws=1) - 438193 / 1000000 (43.8193%)
ChoosePairFromDraw(players=4, draws=2) - 411987 / 1000000 (41.1987%)
```

## Conclusion
If there are 4 players and you go first, it is marginally better (44.39% vs 43.67%) to wait till the draw stage and choose the first 3-remaining or 2-remaining card for the pair in the full-house.
