use super::Player;

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Winner(Player),
    Draw,
}
