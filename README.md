# War-Card-Game
This project is an implementation of the classic card game "War" in Elixir, Haskell, and Rust. The game simulates a shuffled deck of cards and deals them to two players, who take turns revealing cards and engaging in "war" when ties occur. The game continues until one player runs out of cards. Each implementation follows the same rules and technical details.

## Table of Contents

[Game Description](##Game-Description)
[Technical Details](##Technical-Details)
[Usage](##Usage)

## Game Description
The game starts with a shuffled deck of cards. The deck is passed into the program already shuffled. Each player is dealt 26 cards, and cards are placed on top of each other. In each round, both players reveal the top card of their pile. The player with the higher card wins both cards. If cards are tied, a "war" occurs. During war, additional cards are turned face down and face up until a winner is determined. The game continues until one player runs out of cards.

### Card Ranking
The card ranks, in ascending order, are: 2-10, Jack, Queen, King, Ace. Aces are considered high.

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
