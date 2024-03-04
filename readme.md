# Left Right Center

This is a rust program designed to simulate the luck based dice game [left right center](https://bargames101.com/lcr-game-rules/). It's purpose is to determine if any players have an advantage based on turn order. This is my first ever rust program.

### Building

Ensure you have rust and git installed.

1. `git clone https://github.com/Bobbyperson12/lrc`
1. `cd lrc`
1. `cargo build --release`
1. `./target/release/lrc`

### Usage

Simply run the program and it will ask you how many players and games you want to simulate. You can also pass this information in the command line. For example, if you want 5 players and 100 games, run `lrc 5 100`.

### Findings

Players who's turn is in the latter half of all players have a negligible advantage over the other half. This advantage diminishes quickly as the total amount of players increase. However, the advantage is much greater for two and three total players. When there are two players, the player who goes second will win ~61.80% of the time. When there are three players, the third player will win ~36.51% of the time. Logically speaking, these advantages likely occur because players who go first are the first to potentially lose tokens.