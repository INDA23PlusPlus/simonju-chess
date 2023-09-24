macro_rules! tile {
    (P) => { Tile::Pawn(Color::White)   };
    (N) => { Tile::Knight(Color::White) };
    (B) => { Tile::Bishop(Color::White) };
    (R) => { Tile::Rook(Color::White)   };
    (Q) => { Tile::Queen(Color::White)  };
    (K) => { Tile::King(Color::White)   };
    (p) => { Tile::Pawn(Color::Black)   };
    (n) => { Tile::Knight(Color::Black) };
    (b) => { Tile::Bishop(Color::Black) };
    (r) => { Tile::Rook(Color::Black)   };
    (q) => { Tile::Queen(Color::Black)  };
    (k) => { Tile::King(Color::Black)   };
    (.) => { Tile::Empty                };
    (_) => { Tile::Sentinel             };

    (white) => { 
        Tile::Pawn(Color::White)   | 
        Tile::Knight(Color::White) | 
        Tile::Bishop(Color::White) | 
        Tile::Rook(Color::White)   |
        Tile::Queen(Color::White)  |
        Tile::King(Color::White)
    };
    
    (black) => { 
        Tile::Pawn(Color::Black)   | 
        Tile::Knight(Color::Black) | 
        Tile::Bishop(Color::Black) | 
        Tile::Rook(Color::Black)   |
        Tile::Queen(Color::Black)  |
        Tile::King(Color::Black)
    };
}

macro_rules! out_of_bounds {
    ($index:expr) => {
        BoardError::OutOfBounds(Board::get_pos($index), $index)
    };
}

pub(crate) use tile;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 12;

/// Represents an error occuring in the back-end board representation.
#[derive(Debug, Clone)]
pub enum BoardError {
    OutOfBounds(Pos, usize)
}

impl std::fmt::Display for BoardError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "invalid board operation")
    }
}

/// Represents both piece colors (see [`Tile`]) and player colors (see [`Game`]).
/// 
/// [`Game`]: `super::Game`
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            Color::White => "White",
            Color::Black => "Black",
        };

        write!(f, "{string}")
    }
}

/// Represents tiles on the board.
/// 
/// Note, sentinel tiles are only used internally. 
/// 
/// All pieces have a [`Color`].
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
    Empty,
    Sentinel,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let char = match self {
            tile!(P) => 'P',
            tile!(N) => 'N',
            tile!(B) => 'B',
            tile!(R) => 'R',
            tile!(Q) => 'Q',
            tile!(K) => 'K',
            tile!(p) => 'p',
            tile!(n) => 'n',
            tile!(b) => 'b',
            tile!(r) => 'r',
            tile!(q) => 'q',
            tile!(k) => 'k',
            tile!(.) => '.',
            tile!(_) => '_',
        };

        write!(f, "{char}")
    }
}

/// Represents zero-indexed positions on the board.
/// 
/// Positions are used commonly used when interfacing with the user.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub rank: i32,
    pub file: i32,
}


/// Represents the chessboard as a one dimensional array.
#[derive(Debug)]
pub struct Board {
    tiles: [Tile; BOARD_WIDTH * BOARD_HEIGHT],
}

impl Board {
    /// Returns the tile at a given index.
    pub(crate) fn get_tile(&self, index: usize) -> Option<&Tile> {
        self.tiles.get(index) 
    }

    /// Sets a tile value at a given index, 
    /// or produces an error if the index is out of bounds, or the tile is a sentinel value.
    pub(crate) fn set_tile(&mut self, index: usize, piece: Tile) -> Result<(), BoardError> {
        match self.tiles.get_mut(index) {
            Some(tile) => match piece {
                // Sentinel tiles cannot be set.
                Tile::Sentinel => Err(out_of_bounds!(index)),
                _ => { *tile = piece; Ok(()) },
            },
            None => Err(out_of_bounds!(index)),
        }
    }

    /// Sets a tile to empty and returns the removed tile, 
    /// or produces an error if the index is out of bounds or the tile is a sentinel value.
    pub(crate) fn rem_tile(&mut self, index: usize) -> Result<Tile, BoardError> {
        match self.tiles.get_mut(index) {
            Some(tile) => match tile {
                // Sentinel tiles cannot be removed.
                Tile::Sentinel => Err(out_of_bounds!(index)),
                _ => { 
                    let val = tile.clone();
                    *tile = Tile::Empty; 
                    Ok(val)
                },
            },
            None => Err(out_of_bounds!(index)),
        }
    }

    /// Returns all tiles.
    pub(crate) fn get_tiles(&self) -> &[Tile; BOARD_WIDTH * BOARD_HEIGHT] {
        &self.tiles
    }

    /// Creates an empty board surrounded by sentitnel tiles.
    pub(crate) fn new() -> Self {
        let mut board = Self {
            tiles: [tile!(.); BOARD_WIDTH * BOARD_HEIGHT]
        };

        for (index, tile) in board.tiles.iter_mut().enumerate() {
            if (index < 20) | (index >= 100) | (index % 10 == 0) | (index % 10 == 9) {
                *tile = tile!(_);
            }
        }

        board
    }

    /// Clears the board of any lingering pieces.
    pub(crate) fn empty(&mut self) {
        self.tiles = Self::new().tiles;
    }

    /// Returns the tile index for a given position.
    pub(crate) fn get_index(pos: Pos) -> usize {
        (1 + pos.file + 20 + pos.rank * 10) as usize
    }

    /// Returns the tile position for a given index.
    pub(crate) fn get_pos(index: usize) -> Pos {
        Pos { 
            rank: (index as i32 - 21) / 10,
            file: (index as i32 - 1) % 10,
        }
    }

    /// Returns the algebraic notation, in capital letters, for a given index.
    /// 
    /// Sentinel values are treated as spaces.
    pub(crate) fn index_to_string(index: usize) -> String {
        let mut string = String::new();

        let rank = match index / 10 {
            2 => '1',
            3 => '2',
            4 => '3',
            5 => '4',
            6 => '5',
            7 => '6',
            8 => '7',
            9 => '8',
            _ => ' ',
        };

        let file = match index % 10 {
            1 => 'A',
            2 => 'B',
            3 => 'C',
            4 => 'D',
            5 => 'E',
            6 => 'F',
            7 => 'G',
            8 => 'H',
            _ => ' ',
        };
        
        string.push(file);
        string.push(rank);
        
        string
    }
}

/* TESTING
 * Protection against bugs.
 * Unit tests: tests isolated parts of the codebase.
 * Integration tests: tests how components interacts.
 * End-to-end tests: tests the whole program.
 * Regression tests: ???
 * Fuzz tests: tests random input to find flaws.
 * Benchmark tests: tests performance.
 * 
 * GIT
 * Avoid committing large amount of work at once.
 * Avoid long commit messages.
 * Look up rabasing and squashes.
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board() {
        let default_tiles = [
            tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_),
            tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_),
            tile!(_), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(_),
            tile!(_), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(_),
            tile!(_), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(_),
            tile!(_), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(_),
            tile!(_), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(_),
            tile!(_), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(_),
            tile!(_), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(_),
            tile!(_), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(.), tile!(_),
            tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_),
            tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_), tile!(_),
        ];

        let board = Board::new();

        assert_eq!(board.tiles, default_tiles);

        assert_eq!(*board.get_tiles(), default_tiles);
    }
}
