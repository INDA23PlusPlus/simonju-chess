mod game;
pub use crate::game::*;

mod board;
pub use crate::board::*;

pub fn default_game() -> Game {
    Game::new()
}

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
