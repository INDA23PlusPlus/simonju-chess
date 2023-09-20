pub use crate::board::*;

#[derive(Debug, Clone)]
pub struct PlyError;

impl std::fmt::Display for PlyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "invalid ply")
    }
}

pub enum Player {
    White,
    Black,
}

pub struct Ply {
    pub f: Pos, // f: from
    pub t: Pos, // t: to
}

pub struct Game {
    board: Board,
    player: Player,
    plys: Vec<Ply>,
    en_passant: Option<usize>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            player: Player::White,
            plys: Vec::new(),
            en_passant: None,
        }
    }

    pub fn ply() {
        
    }

    pub fn get_plys(&self) {
        match self.player {
            Player::White => for (index, tile) in self.board.get_tiles().iter().enumerate() {
                match tile {
                    tile!(P) => (),
                    tile!(N) => self.knight_plys(index),
                    tile!(B) => (),
                    tile!(R) => (),
                    tile!(Q) => (),
                    tile!(K) => (),
                    _ => (),
                }
            }
            Player::Black => for (index, tile) in self.board.get_tiles().iter().enumerate() {
                match tile {
                    tile!(p) => (),
                    tile!(b) => (),
                    tile!(r) => (),
                    tile!(q) => (),
                    tile!(k) => (),
                    tile!(n) => (),
                    _ => (),
                }
            }
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_tile(&self, pos: Pos) -> Option<&Tile> {
        match self.board.get_tile(Self::get_index(pos)?)? {
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

    // Returns the board position as an index.
    fn get_index(pos: Pos) -> Option<usize> {
        Some((1 + pos.x + 20 + pos.y * 10) as usize) 
    }

    fn pawn_plys(index: usize, origin: usize, player: Player) {
        todo!();
        // Push
        // Double Push
        // Capture
        // Promotion
        // En passant
        /*  . +20 .
         * +9 +10+11
         *  .  O  .
         *  .  .  .
         */
        /*  .  .  .
         *  .  O  .
         * -11-10-9
         *  . -20 .
         */
    }

    fn bishop_plys() {
        todo!();
        // Step
        // Iterate
        /* +9  . +11
         *  .  O  .
         * -11 . -9
         */
        loop {
            
        }
    }

    fn knight_plys(&self, origin: usize) {
        let knight_delta = [-21, -19, -12, -8, 8, 12, 19, 21];
        // Ply
        /*  . +19 . +21 .
         * +8  .  .  . +12
         *  .  .  O  .  .
         * -12 .  .  . -8
         *  . -21 . -19 .
         */
        for index in knight_delta.iter().map(|i| i + origin as isize) {
            match self.board.get_tile(index as usize) {
                _ => (),
            }
        }

    }

    fn rook_plys(index: usize, player: Player) {
        todo!();
        // Step
        // Iterate
        /*  . +10 .
         * -1  O +1
         *  . -10 .
         */
    }

    fn queen_plys() {
        todo!();
        // rook_plys()
        // bishop_plys()
        /* +9 +10+11
         * -1  O +1
         * -11-10-9
         */
    }

    fn king_plys() {
        todo!();
        // Step
        // Castling
        /* +9 +10+11
         * -1  O +1
         * -11-10-9
         */
    }
}