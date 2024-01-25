mod test;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    number: usize,
}

#[derive(Debug)]
pub struct Puzzle {
    // Since we know that the grid is a square,
    // we can store it in a single (1-D) vector for efficiency
    tiles: Vec<Tile>,
    zero_index: usize,
    size: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Up,
    Right,
    Down,
    Left,
    None,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.number != 0 {
            write!(f, "{}", self.number)
        } else {
            // U+2007 is figure space, i.e. a space the same size as a number
            write!(f, "\u{2007}")
        }
    }
}

impl Puzzle {
    pub fn is_win(&self) -> bool {
        // Ensure all the numbers are correct
        for i in 1..(self.tiles.len() - 1) {
            if self.tiles[i - 1].number != i {
                return false;
            }
        }

        true
    }

    pub fn new_random(size: usize) -> Puzzle {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut rng = thread_rng();

        let mut tiles = Vec::new();

        for i in 0..(size * size) {
            tiles.push(Tile { number: i });
        }

        tiles.shuffle(&mut rng);

        Puzzle::new(size, tiles)
    }
    pub fn perform(&mut self, action: Action) {
        // For each action: ensure not on edge of board, then swap
        match action {
            Action::Down => {
                // Down means move the zero up
                // Top row means is first size elements
                if self.zero_index >= self.size {
                    // Swap with one row above
                    self.tiles
                        .swap(self.zero_index, self.zero_index - self.size);
                    self.zero_index -= self.size;
                }
            }
            Action::Left => {
                // Left means move the zero to the right
                // Rightmost is when mod size == size - 1
                if self.zero_index % self.size != self.size - 1 {
                    // Swap with one column to the right
                    self.tiles.swap(self.zero_index, self.zero_index + 1);
                    self.zero_index += 1;
                }
            }
            Action::Up => {
                // Up means move the zero down
                // Bottom row is last size elements
                if self.zero_index + self.size < self.tiles.len() {
                    // Swap with one row below
                    self.tiles
                        .swap(self.zero_index, self.zero_index + self.size);
                    self.zero_index += self.size;
                }
            }
            Action::Right => {
                // Right means move the zero left
                // Leftmost is when mod size == 0
                if self.zero_index % self.size != 0 {
                    // Swap with one column to the left
                    self.tiles.swap(self.zero_index, self.zero_index - 1);
                    self.zero_index -= 1;
                }
            }
            Action::None => (),
        }
    }
    fn new(size: usize, tiles: Vec<Tile>) -> Puzzle {
        // Search for the zero and save its location
        let zero_index = tiles.iter().position(|&t| t.number == 0).unwrap();

        Puzzle {
            size,
            tiles,
            zero_index,
        }
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use comfy_table::Table;

        let mut table = Table::new();

        let mut idx: usize = 0;
        for _i in 0..self.size {
            table.add_row(&self.tiles[idx..idx + self.size]);

            idx += self.size;
        }

        write!(f, "{table}")
    }
}