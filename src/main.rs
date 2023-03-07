use crate::game_core::*;
use std::io;

mod game_core;

// remember to name the output file pbrain-<name>.exe
fn main() {
    let mut game = GoGame::new();
    println!("{}", game.board);
    loop {
        println!("Enter x-y coordinates (1-{})\nx y", BOARD_SIZE);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let (x, y) = match parse_tuple::<usize>(&input) {
            Some((x, y)) => (x, y),
            None => {
                println!("{}", game.board);
                eprintln!("Invalid coordinates. Try again.");
                continue;
            }
        };

        if x < 1 || x > BOARD_SIZE || y < 1 || y > BOARD_SIZE {
            println!("{}", game.board);
            eprintln!("Invalid coordinates. Try again.");
            continue;
        }

        let x = x - 1;
        let y = y - 1;

        if !game.board.is_legal(x, y) {
            println!("{}", game.board);
            eprintln!("Invalid coordinates. Try again.");
            continue;
        }
        game.make_move(x, y);
        println!("{}", game.board);
    }
}

fn parse_tuple<T: std::str::FromStr>(input: &str) -> Option<(T, T)> {
    let numbers: Result<Vec<T>, _> = input
        .trim()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<T>, _>>();
    match numbers {
        Ok(mut nums) => {
            if nums.len() != 2 {
                return None;
            }
            Some((nums.remove(0), nums.remove(0)))
        }
        Err(_) => None,
    }
}
