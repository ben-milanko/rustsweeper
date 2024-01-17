// RustSweeper
//
// A terminal Minesweeper game written in rust.

// Default size of the game, the game will be SIZExSIZE
use rand::Rng;
use std::{io, process::exit};

const SIZE: u8 = 8;
const CRAB_COUNT: u8 = 8;

#[derive(Copy, Clone, Debug)]
struct Tile {
    count: u8,
    crab: bool,
    revealed: bool,
}

fn main() {
    // cSpell:disable-next-line
    println!(" |🦀| RUSTSWEEPER |🦀| ");
    println!("The board size is {SIZE}x{SIZE} with {CRAB_COUNT} 🦀");
    println!();

    let mut moves = 0;

    let empty_tile: Tile = Tile {
        count: 0,
        crab: false,
        revealed: false,
    };

    // initialise the board
    let mut board: [[Tile; 8]; 8] = [[empty_tile; 8]; 8];

    render(&board);

    // add the crabs
    while count_crabs(&board) < CRAB_COUNT {
        let x = rand::thread_rng().gen_range(0..SIZE);
        let y = rand::thread_rng().gen_range(0..SIZE);

        let x_index = usize::from(x);
        let y_index = usize::from(y);

        board[x_index][y_index] = Tile {
            crab: true,
            ..board[x_index][y_index]
        };
    }

    for x in 0..SIZE {
        for y in 0..SIZE {
            let count = get_count(&board, x, y);

            let x_index = usize::from(x);
            let y_index = usize::from(y);

            board[x_index][y_index] = Tile {
                count: count,
                ..board[x_index][y_index]
            };
        }
    }

    loop {
        println!("Enter coordinate, as a capital letter and number");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        if command == "help" {
            println!("Enter the coordinate to test, e.g. A1")
        } else {
            let mut iter = command.bytes().into_iter();

            if iter.len() > 3 {
                println!("Invalid coordinate, too long");
                continue;
            }

            let first = iter.next().expect("Invalid coordinate");
            let second = iter.next().expect("Invalid coordinate");

            if first < 65 || first > 72 {
                let c = char::from(first);
                println!("{c} is out of range A to H");
                continue;
            }

            if second < 49 || second > 56 {
                let c = char::from(second);
                println!("{c} is out of range 1 to 8");
                continue;
            }

            let first = first - 65;
            let second = second - 49;

            let x_index = usize::from(first);
            let y_index = usize::from(second);

            if board[x_index][y_index].crab {
                reveal_crabs(&mut board);

                render(&board);

                println!("Oh no! You hit a crab");
                if moves == 0 {
                    println!("Unlucky on the first turn.")
                }
                exit(0)
            } else {
                flood_fill(&mut board, first, second);
            }

            moves = moves + 1;

            if check_win(&board) {
                reveal_crabs(&mut board);
                render(&board);

                println!("CONGRATULATIONS YOU WON!!!");
                println!("Won in {moves} moves");
                exit(0)
            }
        }

        render(&board);
    }
}

fn render(&board: &[[Tile; 8]; 8]) {
    for y in 0..(SIZE + 3) {
        for x in 0..(SIZE + 3) {
            if x == 0 && y == 0 {
                print!("    ")
            } else if y == 0 && x > 0 && x < SIZE + 1 {
                let c: char = char::from(64 + x);

                print!("{c} ");
            } else if x == 0 && y > 1 && y < SIZE + 2 {
                let display = y - 1;
                print!("{display} ")
            } else if x == 1 || x == SIZE + 2 && y != 0 {
                print!("| ")
            } else if y == 1 && x > 1 || y == SIZE + 2 && x != 0 {
                print!("- ")
            } else if y > 1 && x > 1 && y < SIZE + 2 && x < SIZE + 2 {
                let x_index = usize::from(x - 2);
                let y_index = usize::from(y - 2);

                let tile = board[x_index][y_index];

                if tile.crab && tile.revealed {
                    // crabs have no extra spaces because they take up to spaces in the monospace font
                    print!("🦀")
                } else if tile.revealed {
                    let c = tile.count;
                    print!("{c} ")
                } else {
                    print!("x ")
                }
            } else {
                print!("  ")
            }
        }
        println!()
    }
}

fn count_crabs(&board: &[[Tile; 8]; 8]) -> u8 {
    let mut count: u8 = 0;

    for x in 0..SIZE {
        for y in 0..SIZE {
            let x_index = usize::from(x);
            let y_index = usize::from(y);

            if board[x_index][y_index].crab {
                count = count + 1;
            }
        }
    }

    return count;
}

// gets the count of adjacent crabs for a single tile at x and y
// counts the 8 tile ring around the tile
fn get_count(&board: &[[Tile; 8]; 8], x: u8, y: u8) -> u8 {
    let mut count = 0;

    if x >= SIZE || y >= SIZE {
        return 0;
    }

    for x_d in 0..=2 {
        for y_d in 0..=2 {
            // skip the tile itself
            if x_d == 1 && y_d == 1 {
                continue;
            }

            let ix = i16::from(x + x_d) - 1;
            let iy = i16::from(y + y_d) - 1;

            let i_size = i16::from(SIZE);

            if ix < 0 || ix >= i_size || iy < 0 || iy >= i_size {
                continue;
            }

            let x_index = usize::from(x + x_d - 1);
            let y_index = usize::from(y + y_d - 1);

            let tile = board[x_index][y_index];

            if tile.crab {
                count = count + 1;
            }
        }
    }

    count
}

fn reveal_crabs(board: &mut [[Tile; 8]; 8]) {
    for x in 0..SIZE {
        for y in 0..SIZE {
            let x_index = usize::from(x);
            let y_index = usize::from(y);

            if board[x_index][y_index].crab {
                board[x_index][y_index] = Tile {
                    revealed: true,
                    ..board[x_index][y_index]
                }
            }
        }
    }
}

fn flood_fill(board: &mut [[Tile; 8]; 8], x: u8, y: u8) {
    let x_index = usize::from(x);
    let y_index = usize::from(y);

    let tile = board[x_index][y_index];

    if tile.revealed || tile.crab {
        return;
    }

    board[x_index][y_index] = Tile {
        revealed: true,
        ..tile
    };

    if tile.count > 0 {
        return;
    }

    for x_d in 0..=2 {
        for y_d in 0..=2 {
            // skip the tile itself
            if x_d == 1 && y_d == 1 {
                continue;
            }

            // diagonals aren't checked in flood fill?
            // if (x_d == 0 && y_d == 0) || () {
            //     continue;
            // }

            let ix = i16::from(x + x_d) - 1;
            let iy = i16::from(y + y_d) - 1;

            let i_size = i16::from(SIZE);

            if ix < 0 || ix >= i_size || iy < 0 || iy >= i_size {
                continue;
            }

            flood_fill(board, x + x_d - 1, y + y_d - 1);
        }
    }
}

fn check_win(&board: &[[Tile; 8]; 8]) -> bool {
    for x in 0..SIZE {
        for y in 0..SIZE {
            let x_index = usize::from(x);
            let y_index = usize::from(y);

            let tile = board[x_index][y_index];

            // if a tile is safe and is also not revealed, the game has not been won
            if !tile.crab && !tile.revealed {
                return false;
            }
        }
    }

    true
}
