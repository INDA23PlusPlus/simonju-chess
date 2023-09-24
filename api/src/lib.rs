//! This is a chess library lacking many important features. Use at your own risk!
//! 
//! To start a game, create a [`Game`] object.
//! ```
//! // Creates a game in starting position.
//! let mut game = api::default_game();
//! ```
//! 
//! To get the current board state, use [`Game::get_board_2d`].
//! 
//! To get the current player, use [`Game::get_player`].
//! 
//! To get all valid plys for the current player, use [`Game::get_plys`].
//! 
//! To get all valid plys for a specific piece (based on the current player), use [`Game::get_plys_from_pos`].
//! 
//! To make a ply, use [`Game::ply`].

mod game;
pub use crate::game::*;

mod board;
pub use crate::board::*;

/// Returns a [`Game`] object representing the beginning position in chess.
/// 
/// If you wish to start a game from a specific position, 
/// consider using [`fen_game`] instead.
/// 
/// # Examples
/// 
/// Basic Use:
/// ```
/// let mut game = api::default_game();
/// ```
pub fn default_game() -> Game {
    match Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1") {
        Ok(x) => x,
        Err(_) => panic!(),
    }
}

/// Returns a [`Game`] object representing an arbitrary position in chess.
/// 
/// # Examples
/// 
/// Default position:
/// ```
/// let game = api::fen_game("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
/// ```
/// 
/// Empty board:
/// ```
/// let game = api::fen_game("8/8/8/8/8/8/8/8 w - - 0 1");
/// ```
pub fn fen_game(fen: &str) -> Result<Game, FenParseError> {
    Game::from_fen(fen)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_init() {
    }
}
