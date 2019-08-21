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
