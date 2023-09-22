use crate::board::*;

use super::Game;

#[derive(Debug, Clone)]
pub enum FenParseError {
    CouldNotRead,
    InvalidPiece(char),
    InvalidColor(char),
    InvalidCastling(char),
    InvalidEnPassant(char),
    InvalidPlyClock(char),
    InvalidMoveClock(char),
    RankOverflow,
    FileOverflow,
    Unknown,
}

impl std::fmt::Display for FenParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {

        write!(formatter, "fen parsing error")
    }
}

impl From<char> for FenParseError {
    fn from(error: char) -> Self {
        FenParseError::InvalidPiece(error)
    }
}

impl Game {
    pub fn from_fen(fen: &str) -> Result<Self, FenParseError>{
        let mut game = Self::new();

        match game.read_fen(fen) {
            Ok(_) => Ok(game),
            Err(e) => Err(e),
        }
    }

    pub fn read_fen(&mut self, fen: &str) -> Result<(), FenParseError>{
        let mut iter = fen.split_whitespace();

        // 1. Piece placement data.
        let piece_placement_data = iter.next().ok_or(FenParseError::CouldNotRead)?;

        let mut game = Game::new();

        let mut pos = Pos { 
            rank: 7,
            file: 0,
        };

        for char in piece_placement_data.chars() {
            if let Some(empty_count) = char.to_digit(10) {
                if pos.file + empty_count as i32 > 8 {
                    return Err(FenParseError::FileOverflow);
                } else {
                    pos.file += empty_count as i32;
                }

            } else if char == '/' {
                pos.rank -= 1;
                pos.file = 0;

                if pos.rank < 0 {
                    return Err(FenParseError::RankOverflow);
                }
            } else {
                let tile = match char {
                    'P' => tile!(P),
                    'B' => tile!(B),
                    'N' => tile!(N),
                    'R' => tile!(R),
                    'Q' => tile!(Q),
                    'K' => tile!(K),
                    'p' => tile!(p),
                    'b' => tile!(b),
                    'n' => tile!(n),
                    'r' => tile!(r),
                    'q' => tile!(q),
                    'k' => tile!(k),
                    _ => return Err(FenParseError::InvalidPiece(char)),
                };

                match game.board.set_tile(Board::get_index(pos), tile) {
                    Ok(_) => (),
                    Err(_) => return Err(FenParseError::Unknown),
                };

                pos.file += 1;
            }
        }

        // 2. Active color.
        let active_color = iter.next().ok_or(FenParseError::CouldNotRead)?;

        game.player = match active_color.chars().next() {
            Some('w') => Color::White,
            Some('b') => Color::Black,
            x => return match x {
                Some(x) => Err(FenParseError::InvalidColor(x)),
                None => Err(FenParseError::InvalidColor(' ')),
            }
        };

        // 3. Castling rights.
        let castling_rights = iter.next().ok_or(FenParseError::CouldNotRead)?;

        // 4. En passant square.
        let en_passant_square = iter.next().ok_or(FenParseError::CouldNotRead)?;

        // 5. Ply clock.
        let ply_clock = iter.next().ok_or(FenParseError::CouldNotRead)?;

        // 6. Move clock.
        let move_clock = iter.next().ok_or(FenParseError::CouldNotRead)?;

        self.renew();

        self.board = game.board;
        self.player = game.player;

        self.gen_plys();

        Ok(())
    }

    fn write_fen(&self) -> String {
        todo!();
    }
}