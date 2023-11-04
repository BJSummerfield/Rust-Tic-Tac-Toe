use crate::game::Game;
use rand::{seq::SliceRandom, thread_rng};

pub trait AI {
    fn computer_move(&mut self);
}

impl AI for Game {
    fn computer_move(&mut self) {
        let mut rng = thread_rng();
        let available_positions = self.board.available_positions();

        if let Some(&(row, col)) = available_positions.as_slice().choose(&mut rng) {
            self.make_move(row, col)
                .expect("Should never fail since the position is available");
        }
    }
}
