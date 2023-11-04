use super::Board;
use super::GameState;
use super::Player;
use super::AI;

pub struct Game {
    pub board: Board,
    pub current_player: Player,
    pub state: GameState,
    num_human_players: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_player: Player::X,
            state: GameState::InProgress,
            num_human_players: 1,
        }
    }

    pub fn computer_move(&mut self) {
        AI::computer_move(self)
    }

    pub fn print_board(&self) {
        self.board.print()
    }

    pub fn new_with_human_players(num_human_players: u8) -> Self {
        let mut game = Self::new();
        game.num_human_players = num_human_players;
        game
    }

    pub fn is_current_player_human(&self) -> bool {
        match self.num_human_players {
            1 => self.current_player == Player::X,
            2 => true,
            _ => false,
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if self.state != GameState::InProgress {
            return Err("Game has ended");
        }
        if row >= self.board.size || col >= self.board.size {
            return Err("Move out of bounds");
        }
        if self.board.is_occupied(row, col) {
            return Err("Cell is already occupied");
        }

        self.board.place_mark(row, col, self.current_player);
        self.check_winner();
        if self.state == GameState::InProgress {
            self.toggle_player();
        }
        Ok(())
    }

    fn check_winner(&mut self) {
        if self.check_rows_for_winner()
            || self.check_columns_for_winner()
            || self.check_diagonals_for_winner()
        {
            return;
        }

        if self.is_draw() {
            self.state = GameState::Draw;
        }
    }

    fn check_rows_for_winner(&mut self) -> bool {
        for row in 0..self.board.size {
            if self.check_line_for_winner((row, 0), (0, 1)) {
                return true;
            }
        }
        false
    }

    fn check_columns_for_winner(&mut self) -> bool {
        for col in 0..self.board.size {
            if self.check_line_for_winner((0, col), (1, 0)) {
                return true;
            }
        }
        false
    }

    fn check_diagonals_for_winner(&mut self) -> bool {
        self.check_line_for_winner((0, 0), (1, 1))
            || self.check_line_for_winner((0, self.board.size - 1), (1, -1))
    }

    fn check_line_for_winner(&mut self, start: (usize, usize), direction: (isize, isize)) -> bool {
        let (mut row, mut col) = start;
        let player = match self.board.get_cell(row, col) {
            Some(player) => player,
            None => return false,
        };

        for _ in 0..self.board.size - 1 {
            row = (row as isize + direction.0) as usize;
            col = (col as isize + direction.1) as usize;
            if self.board.get_cell(row, col) != Some(player) {
                return false;
            }
        }

        self.state = GameState::Winner(player);
        true
    }

    fn is_draw(&self) -> bool {
        self.board.is_full()
    }

    pub fn get_current_player(&self) -> Player {
        self.current_player
    }

    fn toggle_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}
