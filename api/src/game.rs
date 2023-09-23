pub use crate::board::*;

mod fen;
pub use fen::*;

mod ply_gen;
pub use ply_gen::*;


/// Represents a game of chess.
/// 
/// All user interaction should be handled through Game objects.
pub struct Game {
    board: Board,
    player: Color,
    plys: Vec<Ply>,
    en_passant: Option<usize>,
}

impl Game {
    pub(crate) fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            player: Color::White,
            plys: Vec::new(),
            en_passant: None,
        };

        game.gen_plys();

        game
    }

    pub(crate) fn renew(&mut self) {
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
            println!("Invalid Ply: {}", ply);
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

    pub fn ply_str(&mut self, str: &str) -> Result<(), PlyError> {
        let ply = Self::get_pos_from_str(str);
        match ply {
            Some(x) => {
                match x.0 {
                    Some(origin) => {
                        match x.1 {
                            Some(destination) => {
                                self.ply(origin, destination)
                            },
                            None => Err(PlyError::InvalidPly),
                        }
                    },
                    None => Err(PlyError::InvalidPly),
                }
            }, 
            None => Err(PlyError::InvalidPly),
        }
    }

    pub fn get_plys(&self) -> &Vec<Ply> {
        &self.plys
    }

    pub fn get_plys_from_pos(&self, pos: Pos) -> Vec<&Ply> {
        let mut plys = Vec::new();

        for ply in &self.plys {
            if Board::get_pos(ply.origin) == pos {
                plys.push(ply)
            }
        }

        plys
    }

    pub fn get_plys_from_str(&self, str: &str) -> Vec<&Ply> {
        self.get_plys_from_pos(match Self::get_pos_from_str(str) {
            Some(pos_pos) => match pos_pos.0 {
                Some(pos) => pos,
                None => return vec![],
            },
            None => return vec![],
        })
    }

    pub fn get_player(&self) -> &Color {
        &self.player
    }

    pub fn get_tile_from_pos(&self, pos: Pos) -> Option<&Tile> {
        match self.board.get_tile(Board::get_index(pos))? {
            tile!(_) => None,
            x => Some(x),
        }
    }

    pub fn get_tile_from_str(&self, str: &str) -> Option<&Tile> {
        self.get_tile_from_pos(Self::get_pos_from_str(str)?.0?)
    }

    pub fn get_board_1d(&self) -> [&Tile; 64] {
        let mut board = [&Tile::Empty; 64];

        let index = 0;
        for tile in self.board.get_tiles() {
            match tile {
                tile!(_) => (),
                _ => board[index] = tile,
            }
        }

        board
    }
    
    /// 
    pub fn get_board_2d(&self) -> [[&Tile; 8]; 8]{
        let mut board = [[&Tile::Empty; 8]; 8];

        for rank in 0..8 {
            for file in 0..8 {
                let tile = self.get_tile_from_pos(Pos {
                    rank: rank as i32,
                    file: file as i32,
                }).unwrap(); // Very bad!
                match tile {
                    tile!(_) => (),
                    _ => board[rank][file] = tile,
                }
            }
        }

        board
    }

    fn get_pos_from_str(str: &str) -> Option<(Option<Pos>, Option<Pos>)> {
        let mut pos_pair: (Option<Pos>, Option<Pos>) = (None, None);

        let binding = str.split_whitespace().collect::<String>().to_lowercase();
        let mut iter = binding.chars();

        let origin_file = match iter.next()? {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return None,
        };

        let origin_rank = match iter.next()? {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return None,
        };

        let destination_file = match iter.next()? {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return None,
        };

        let destination_rank = match iter.next()? {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return None,
        };

        pos_pair.0 = Some(Pos {
            file: origin_file, 
            rank: origin_rank,
        });

        pos_pair.1 = Some(Pos {
            file: destination_file, 
            rank: destination_rank,
        });

        Some(pos_pair)
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

                let tile = match self.get_tile_from_pos(pos) {
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
        let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        println!("{:?}", game);

        game.ply_str("a2 a3");

        println!("{:?}", game);
    }
}