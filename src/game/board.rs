use super::Player;

const SIZE: usize = 3;

pub struct Board {
    pub size: usize,
    cells: [[Option<Player>; SIZE]; SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            size: SIZE,
            cells: [[None; SIZE]; SIZE],
        }
    }
    pub fn is_occupied(&self, row: usize, col: usize) -> bool {
        self.cells[row][col].is_some()
    }

    pub fn place_mark(&mut self, row: usize, col: usize, player: Player) {
        if !self.is_occupied(row, col) {
            self.cells[row][col] = Some(player);
        }
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(Option::is_some)
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<Player> {
        self.cells[row][col]
    }

    pub fn available_positions(&self) -> Vec<(usize, usize)> {
        let mut available = Vec::new();

        for (row, cols) in self.cells.iter().enumerate() {
            for (col, cell) in cols.iter().enumerate() {
                if cell.is_none() {
                    available.push((row, col));
                }
            }
        }

        available
    }

    pub fn print(&self) {
        for row in &self.cells {
            for cell in row {
                print!(
                    "{} ",
                    match cell {
                        Some(player) => player.to_string(),
                        None => String::from("-"),
                    }
                );
            }
            println!();
        }
    }
}
