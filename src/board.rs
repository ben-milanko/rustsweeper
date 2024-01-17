use crate::tile::Tile;
use rand::Rng;

// const SIZE: u8 = 8;
const CRAB_COUNT: u8 = 8;
pub const BOARD_SIZE: usize = 8;

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub tiles: [[Tile; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn init(&mut self) {
        while self.count_crabs() < CRAB_COUNT {
            let x = rand::thread_rng().gen_range(0..BOARD_SIZE);
            let y = rand::thread_rng().gen_range(0..BOARD_SIZE);

            self.tiles[x][y] = Tile {
                crab: true,
                ..self.tiles[x][y]
            };
        }

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let count = self.get_count(x, y);

                self.tiles[x][y] = Tile {
                    count: count,
                    ..self.tiles[x][y]
                };
            }
        }
    }

    pub fn count_crabs(&self) -> u8 {
        let mut count: u8 = 0;

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if self.tiles[x][y].crab {
                    count = count + 1;
                }
            }
        }

        return count;
    }

    pub fn get_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return 0;
        }

        for x_d in 0..=2 {
            for y_d in 0..=2 {
                // skip the tile itself
                if x_d == 1 && y_d == 1 {
                    continue;
                }

                // make sure the tile is within the valid range
                let ix = (x + x_d).checked_add_signed(-1);
                let iy = (y + y_d).checked_add_signed(-1);

                // with negative overflow the result of the above checked add signed will be None
                if ix.is_some_and(|ix| ix < BOARD_SIZE) && iy.is_some_and(|iy| iy < BOARD_SIZE) {
                    let tile = self.tiles[x + x_d - 1][y + y_d - 1];

                    if tile.crab {
                        count = count + 1;
                    }
                }
            }
        }

        count
    }

    pub fn reveal_crabs(&mut self) {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if self.tiles[x][y].crab {
                    self.tiles[x][y] = Tile {
                        revealed: true,
                        ..self.tiles[x][y]
                    }
                }
            }
        }
    }

    pub fn check_win(&self) -> bool {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let tile = self.tiles[x][y];

                // if a tile is safe and is also not revealed, the game has not been won
                if !tile.crab && !tile.revealed {
                    return false;
                }
            }
        }

        true
    }

    // from the given tile, recursively flood fills the surrounding tiles
    //
    // the recursive function instance exits on:
    // - a revealed tile
    // - a crab tile
    // - a tile with non-zero count
    pub fn flood_fill(&mut self, x: usize, y: usize) {
        let x_index = usize::from(x);
        let y_index = usize::from(y);

        let tile = self.tiles[x_index][y_index];

        if tile.revealed || tile.crab {
            return;
        }

        self.tiles[x_index][y_index] = Tile {
            revealed: true,
            ..tile
        };

        if tile.count > 0 {
            return;
        }

        // recursively
        for x_d in 0..=2 {
            for y_d in 0..=2 {
                // skip the tile itself
                if x_d == 1 && y_d == 1 {
                    continue;
                }

                // make sure the tile is within the valid range
                let ix = (x + x_d).checked_add_signed(-1);
                let iy = (y + y_d).checked_add_signed(-1);

                // with negative overflow the result of the above checked add signed will be None
                if ix.is_some_and(|ix| ix < BOARD_SIZE) && iy.is_some_and(|iy| iy < BOARD_SIZE) {
                    self.flood_fill(x + x_d - 1, y + y_d - 1);
                }
            }
        }
    }

    pub fn render(&self) {
        for y in 0..(BOARD_SIZE + 3) {
            for x in 0..(BOARD_SIZE + 3) {
                if x == 0 && y == 0 {
                    print!("    ")
                } else if y == 0 && x > 0 && x < BOARD_SIZE + 1 {
                    let value_u8: Result<u8, _> = (x + 64).try_into();

                    match value_u8 {
                        Ok(v) => {
                            let c: char = char::from(v);

                            print!("{c} ");
                        }
                        Err(e) => println!("Failed to convert: {:?}", e),
                    }
                } else if x == 0 && y > 1 && y < BOARD_SIZE + 2 {
                    let display = y - 1;
                    print!("{display} ")
                } else if x == 1 || x == BOARD_SIZE + 2 && y != 0 {
                    print!("| ")
                } else if y == 1 && x > 1 || y == BOARD_SIZE + 2 && x != 0 {
                    print!("- ")
                } else if y > 1 && x > 1 && y < BOARD_SIZE + 2 && x < BOARD_SIZE + 2 {
                    let tile = self.tiles[x - 2][y - 2];

                    if tile.crab && tile.revealed {
                        // crabs have no extra spaces because they take up to spaces in the monospace font
                        print!("🦀")
                    } else if tile.revealed {
                        // if the tile is revealed show the count for it
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
}
