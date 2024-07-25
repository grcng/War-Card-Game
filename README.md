# War-Card-Game
This project is an implementation of the classic card game `War` in Elixir, Haskell, and Rust. The game simulates a shuffled deck of cards and deals them to two players, who take turns revealing cards and engaging in "war" when ties occur. The game continues until one player runs out of cards. Each implementation follows the same rules and technical details.

## Table of Contents

  * [Game Description](#game-description)  
  * [Technical Details](#technical-details)  
  * [Usage](#usage)

## Game Description
The game starts with a shuffled deck of cards. The deck is passed into the program already shuffled. The program will deal the cards in an alternating fashion to each player, so that each player has 26 cards. Each card is dealt on top of the previous, so that the top card of each pile after dealing will be the last card that was dealt. 

In each round, both players reveal the top card of their pile. The player with the higher card (by rank) wins both cards.

### Card Ranking
The card ranks, in ascending order, are: 2-10, Jack, Queen, King, Ace. Aces are considered high.

### War!
If the revealed cards are tied, there is war! Each player turns up one card face down followed by one card face up. The player with the higher face-up card takes both piles (six cards – the two original cards that were tied, plus the four cards from the war). If the turned-up cards are again the same rank, each player places another card face down and turns another card face up. The player with the higher card takes all 10 cards, and so on.

When one player cannot play or draw a card because their pile is empty, they are the loser, and the other the winner. This applies during a war as well.

## Technical Details
### Input
The input to the program represents a shuffled deck of cards as a permutation of 52 integers. Each integer between 1-13 occurs four times, corresponding to cards according to a predefined table. Suits are not represented because the game of War doesn't require them.
|  1  | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |  11  |   12  |  13  |
|:---:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:--:|:----:|:-----:|:----:|
| Ace | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | Jack | Queen | King |

### Game Flow
The program deals two piles from the input permutation.
The game proceeds until one player can no longer draw cards.
When adding cards to a player's pile, they should be added in decreasing order by rank.

### Output
The program returns the pile of the winning player. The pile contains all 52 integers from the original input permutation and is ordered according to how the game played out.

## Usage
An automated tester is provided for each implementation (Elixir/ Haskell/ Rust). Here's how to test each version from Shell:

### Elixir
Navigate to the `war_ex` directory.
Run following command:

```
mix test
```

### Haskell
Navigate to the `war_hs` directory.
Run following command:

```
cabal test
```

### Rust
Navigate to the `war_rs` directory.
Run following command:

```
cargo test
```
