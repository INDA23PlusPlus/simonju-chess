mod game;
pub use crate::game::*;

mod board;
pub use crate::board::*;

pub fn game() -> Game {
    Game::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_init() {
    }
}
