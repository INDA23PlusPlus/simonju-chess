pub use crate::board::*;

mod fen;
pub use fen::*;

mod ply_gen;
pub use ply_gen::*;

pub struct Game {
    board: Board,
    player: Color,
    plys: Vec<Ply>,
    en_passant: Option<usize>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            player: Color::White,
            plys: Vec::new(),
            en_passant: None,
        };

        game.gen_plys();

        game
    }

    pub fn renew(&mut self) {
        let new_game = Self::new();

        self.board = new_game.board;
        self.player = new_game.player;
        self.plys = new_game.plys;
        self.en_passant = new_game.en_passant;
    }

    pub fn ply(&mut self, origin: Pos, destination: Pos) -> Result<(), PlyError> {
        let origin_index = Board::get_index(origin);
        let destination_index = Board::get_index(destination);
        let ply = Ply {
            origin: origin_index,
            destination: destination_index,
        };

        if !(self.plys.contains(&ply)) {
            println!("Invalid Ply: {}", ply.to_string());
            return Err(PlyError::InvalidPly);
        }

        let tile = match self.board.rem_tile(origin_index) {
            Ok(t) => t,
            Err(_) => return Err(PlyError::Unknown),
        };

        match self.board.set_tile(destination_index, tile) {
            Ok(_) => (),
            Err(_) => return Err(PlyError::Unknown),
        }

        self.gen_plys();

        self.player = match self.player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.gen_plys();

        return Ok(());
    }

    pub fn ply_str(&mut self, &str) -> Result<(), PlyError> {

    }

    pub fn get_plys(&self) -> &Vec<Ply> {
        &self.plys
    }

    pub fn get_player(&self) -> &Color {
        &self.player
    }

    pub fn get_tile(&self, pos: Pos) -> Option<&Tile> {
        match self.board.get_tile(Board::get_index(pos))? {
            tile!(_) => None,
            x => Some(x),
        }
    }

    pub fn get_board_1d(&self) -> [&Tile; 64] {
        todo!();
    }
    
    pub fn get_board_2d(&self) -> [[&Tile; 8]; 8] {
        todo!();
    }

    fn get_pos_from_str(str: &str) -> Result<(Option<Pos>, Option<Pos>), ()> {
        let pos_pair = (None, None);
        for char in str.to_lowercase().chars() {
            match char {

                _ => return Err(()), 
            }
        }

        Ok(pos_pair)
    }
}


impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut str = String::new();

        // Board.
        for rank in (0..=7).rev() {
            str.push_str("| ");
            for file in 0..=7 {
                let pos = Pos {
                    rank,
                    file,
                };

                let tile = match self.get_tile(pos) {
                    Some(t) => match t {
                        tile!(P) => "P ",
                        tile!(N) => "N ",
                        tile!(B) => "B ",
                        tile!(R) => "R ",
                        tile!(Q) => "Q ",
                        tile!(K) => "K ",
                        tile!(p) => "p ",
                        tile!(n) => "n ",
                        tile!(b) => "b ",
                        tile!(r) => "r ",
                        tile!(q) => "q ",
                        tile!(k) => "k ",
                        tile!(.) => ". ",
                        _ => "",
                    },
                    None => "",
                };

                str.push_str(tile);
            }

            str.push_str("|\n");
        }

        // Plys.
        for ply in self.plys.as_slice() {
            let mut string = ply.to_string();
            string.push('\n');
            str.push_str(string.as_str());
        }

        write!(f, "{}", str)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board() {
        let mut game = Game::from_fen("8/8/8/pppppppp/PPPPPPPP/8/8/8 w KQkq - 0 1").unwrap();

        game.ply( Pos {

        });

        println!("{:?}", game);
    }
}