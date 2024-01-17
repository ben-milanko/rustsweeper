// RustSweeper
//
// A terminal Minesweeper game written in rust.
use crate::board::{Board, BOARD_SIZE, CRAB_COUNT};
use std::{io, process::exit};

pub mod board;
pub mod tile;

fn main() {
    // cSpell:disable-next-line
    println!(" |🦀| RUSTSWEEPER |🦀| ");
    println!("The board size is {BOARD_SIZE}x{BOARD_SIZE} with {CRAB_COUNT} 🦀");
    println!();

    let mut moves = 0;

    let mut board: Board = Board::default();

    board.init();
    board.render();

    loop {
        println!("Enter coordinate, as a capital letter and number");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let result = validate_input(command);

        match result {
            None => {
                continue;
            }
            Some(point) => {
                if board.tiles[point.0][point.1].crab {
                    board.reveal_crabs();

                    board.render();

                    println!("Oh no! You hit a crab");
                    if moves == 0 {
                        println!("Unlucky on the first turn.")
                    }
                    exit(0)
                } else {
                    board.flood_fill(point.0, point.1);
                }

                moves = moves + 1;

                if board.check_win() {
                    board.reveal_crabs();
                    board.render();

                    println!("CONGRATULATIONS YOU WON!!!");
                    println!("Won in {moves} moves");
                    exit(0)
                }
            }
        }

        board.render();
    }
}

fn validate_input(input: String) -> Option<(usize, usize)> {
    if input == "help" {
        println!("Enter the coordinate to test, e.g. A1");
        return None;
    } else {
        let mut iter = input.bytes().into_iter();

        if iter.len() > 3 {
            println!("Invalid coordinate, too long");
            return None;
        }

        let first = iter.next().expect("Invalid coordinate");
        let second = iter.next().expect("Invalid coordinate");

        if first < 65 || first > 72 {
            let c = char::from(first);
            println!("{c} is out of range A to H");
            return None;
        }

        if second < 49 || second > 56 {
            let c = char::from(second);
            println!("{c} is out of range 1 to 8");
            return None;
        }

        let first = first - 65;
        let second = second - 49;

        let x_index = usize::from(first);
        let y_index = usize::from(second);

        return Some((x_index, y_index));
    }
}
