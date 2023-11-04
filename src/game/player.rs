use std::fmt;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}
