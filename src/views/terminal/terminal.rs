use crate::game::{Game, GameState};
use std::io::{self, Write};

pub fn play_game_in_terminal() {
    println!("Select number of human players (1 or 2): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num_human_players: u8 = input.trim().parse().unwrap_or(1); // Default to 1 if parsing fails

    let mut game = Game::new_with_human_players(num_human_players);

    while game.state == GameState::InProgress {
        game.print_board();
        println!("Current turn: {:?}", game.get_current_player());

        if game.is_current_player_human() {
            let mut input = String::new();
            println!("Enter your move in the format 'row col': ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let parts: Vec<&str> = input.trim().split_whitespace().collect();

            if parts.len() != 2 {
                println!(
                    "Invalid input. Please enter row and column numbers separated by a space."
                );
                continue;
            }

            let row: usize = match parts[0].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid row number.");
                    continue;
                }
            };

            let col: usize = match parts[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid column number.");
                    continue;
                }
            };

            match game.make_move(row, col) {
                Ok(()) => (),
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
        } else {
            println!("Computer is making a move...");
            game.computer_move();
        }

        if let GameState::Winner(player) = game.state {
            game.print_board();
            println!("{:?} wins!", player);
            break;
        } else if let GameState::Draw = game.state {
            game.print_board();
            println!("It's a draw!");
            break;
        }
    }
}
