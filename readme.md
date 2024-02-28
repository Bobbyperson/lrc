# Left Right Center

This is a rust program designed to simulate the dice game left right center. It's purpose is to determine if any players have an advantage based on turn order.
This is my first ever rust program.

### Building

`cargo build --release`

### Usage

Simply run the program and input how many players and games you want to simulate.

### Findings

Players who's turn is in the latter half of all players have a negligible advantage over others. This advantage diminishes quickly as the total amount of players increase. 