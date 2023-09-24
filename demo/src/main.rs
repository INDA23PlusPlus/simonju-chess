use api::*;
use std::io::{self, stdin};

fn main() {
    /*
    // Preferred method for creating a new game.
    let mut game = api::default_game();

    loop {
        println!("Board");
        let mut board_string = String::new();

        // Use Game::get_board_2d or Game::get_board_1d() to get the current board state.
        // Use Game::get_tile() to get the current state of a single tile.
        let board = game.get_board_2d();

        for rank in (0..8 as usize).rev() {
            board_string.push_str("| ");
            for file in 0..8 as usize {
                // Every tile is either a piece with a color, empty,
                // or a sentinel value that should be ignored.
                board_string.push_str(match board[rank][file] {
                    Tile::Pawn(Color::White)   => "P ",
                    Tile::Bishop(Color::White) => "B ", 
                    Tile::Knight(Color::White) => "N ", 
                    Tile::Rook(Color::White)   => "R ", 
                    Tile::Queen(Color::White)  => "Q ", 
                    Tile::King(Color::White)   => "K ", 
                    Tile::Pawn(Color::Black)   => "p ", 
                    Tile::Bishop(Color::Black) => "b ", 
                    Tile::Knight(Color::Black) => "n ", 
                    Tile::Rook(Color::Black)   => "r ", 
                    Tile::Queen(Color::Black)  => "q ", 
                    Tile::King(Color::Black)   => "k ", 
                    Tile::Empty => ". ",
                    _ => "",
                });
            }
            board_string.push_str("|\n");
        }
        println!("{board_string}");

        // Use Game::get_player() to get the current player.
        println!("{}'s Turn", game.get_player());

        println!("Valid Plys:");
        // Use Game::get_plys() to get the current pseudo-legal plys.
        for ply in game.get_plys() {
            println!("{}", ply);
        }

        'input: loop {
            println!("Enter Ply:");
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(_) => continue 'input,
            }

            // Use Game::ply_str() Game::ply() to make plys with strings and positions respectively.
            match game.ply_str(input.as_str()) {
                Ok(_) => break 'input,
                Err(_) => continue 'input,
            }
        }
    }
    */
}