use std::io;
use rand::{thread_rng, Rng};

fn main() {
    let mut player_count: usize;
    let mut games: usize;
    loop {
        println!("Please input the amount of players.");
        player_count = get_input().trim().parse::<usize>().unwrap_or(0);
        if player_count > 1 {
            break;
        } else {
            println!("Invalid input! Please input a value greater than 1.")
        }
    }
    loop {
        println!("Please input the amount of games to play.");
        games = get_input().trim().parse::<usize>().unwrap_or(0);
        if games > 1 {
            break;
        } else {
            println!("Invalid input! Please input a value greater than 1.")
        }
    }
    let mut wins = vec![0; player_count];

    for _i in 0..games {
        let mut players = vec![3; player_count];
        let winner = play_game(&mut players);
        wins[winner] += 1;
    }
    for (i, _) in wins.iter().enumerate().take(player_count) {
        println!("Player {} won {} times. {:.2}%", i + 1, wins[i], (wins[i] as f64) * 100.0 / (games as f64));
    }
}

fn play_game(players: &mut [i32]) -> usize {
    let mut done: bool = false;
    while !done {
        let mut i: usize = 0;
        for player in 0..players.len() {
            let dice: i32 = players[player].min(3);
            for _j in 0..dice {
                let roll: u32 = roll_dice();
                if roll == 4 { // left
                    players[player] -= 1;
                    if i == 0 {
                        let last_player = players.len() - 1;
                        players[last_player] += 1;
                    } else {
                        players[i - 1] += 1;
                    }
                }
                if roll == 5 { // right
                    players[player] -= 1;
                    if i == players.len() - 1 {
                        players[0] += 1;
                    } else {
                        players[i + 1] += 1;
                    }
                }
                if roll == 6 { // center
                    players[player] -= 1;
                }
            }
            i += 1;
            done = true;
            let mut playing: i32 = 0;
            for player in players.iter() {
                if *player > 0 {
                    playing += 1;
                }
            }
            if playing > 1 {
                done = false;
            }
            if done {
                break;
            }
        }
    }
    for player in 0..players.len() {
        if players[player] > 0 {
            return player;
        }
    }
    println!("Error: No winner found.");
    0
}

fn roll_dice() -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(1..7)
}

fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer);
    buffer
}
