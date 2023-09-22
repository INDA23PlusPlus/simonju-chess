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

#[derive(Debug, Clone)]
pub enum BoardError {
    OutOfBounds(Pos, usize)
}

impl std::fmt::Display for BoardError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "invalid board operation")
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

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

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Board {
    tiles: [Tile; BOARD_WIDTH * BOARD_HEIGHT],
}

impl Board {
    pub fn get_tile(&self, index: usize) -> Option<&Tile> {
        self.tiles.get(index) 
    }

    pub fn set_tile(&mut self, index: usize, piece: Tile) -> Result<(), BoardError> {
        match self.tiles.get_mut(index) {
            Some(tile) => match piece {
                // Sentinel tiles cannot be set.
                Tile::Sentinel => Err(out_of_bounds!(index)),
                _ => { *tile = piece; Ok(()) },
            },
            None => Err(out_of_bounds!(index)),
        }
    }

    pub fn rem_tile(&mut self, index: usize) -> Result<Tile, BoardError> {
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

    pub fn get_tiles(&self) -> &[Tile; BOARD_WIDTH * BOARD_HEIGHT] {
        &self.tiles
    }

    pub fn new() -> Self {
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

    pub fn empty(&mut self) {
        self.tiles = Self::new().tiles;
    }

    // Returns a position as an index.
    pub(crate) fn get_index(pos: Pos) -> usize {
        (1 + pos.file + 20 + pos.rank * 10) as usize
    }

    // Returns an as a position.
    pub(crate) fn get_pos(index: usize) -> Pos {
        Pos { 
            rank: (index as i32 - 21) / 10,
            file: (index as i32 - 1) % 10,
        }
    }

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

    /* 
    pub fn new() -> Self {
        Self {
            tiles: [
                // The ranks are reversed to make sure the coordinates are correct.
                s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_),
                s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_),
                s!(_), s!(R), s!(N), s!(B), s!(Q), s!(K), s!(B), s!(N), s!(R), s!(_),
                s!(_), s!(P), s!(P), s!(P), s!(P), s!(P), s!(P), s!(P), s!(P), s!(_),
                s!(_), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(_),
                s!(_), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(_),
                s!(_), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(_),
                s!(_), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(.), s!(_),
                s!(_), s!(p), s!(p), s!(p), s!(p), s!(p), s!(p), s!(p), s!(p), s!(_),
                s!(_), s!(r), s!(n), s!(b), s!(q), s!(k), s!(b), s!(n), s!(r), s!(_),
                s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_),
                s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_), s!(_),
            ],
        }
    }
    */


    /*
    pub fn print_tiles(&self) {
        let mut str = String::new();

        for tile in self.tiles.iter().enumerate() {
            str.push_str(match tile.1 {
                Tile::WhitePawn     => "P ",
                Tile::WhiteKnight   => "N ",
                Tile::WhiteBishop   => "B ",
                Tile::WhiteRook     => "R ",
                Tile::WhiteQueen    => "Q ",
                Tile::WhiteKing     => "K ",
                Tile::BlackPawn     => "p ",
                Tile::BlackKnight   => "n ",
                Tile::BlackBishop   => "b ",
                Tile::BlackRook     => "r ",
                Tile::BlackQueen    => "q ",
                Tile::BlackKing     => "k ",
                Tile::Empty         => ". ",
                Tile::Sentinel      => "",
            });

            if tile.0 % 10 == 0 {
                str.push('@')
            }
        }


        for rank in str.rsplit('@') {
            println!("{rank}");
        }
    }

    pub fn print_tile(&self, pos: Pos) {
        let mut str = String::new();

        match self.get_tile(pos) {
            Some(tile) => match tile {
                Tile::WhitePawn     => str.push('P'),
                Tile::WhiteKnight   => str.push('N'),
                Tile::WhiteBishop   => str.push('B'),
                Tile::WhiteRook     => str.push('R'),
                Tile::WhiteQueen    => str.push('Q'),
                Tile::WhiteKing     => str.push('K'),
                Tile::BlackPawn     => str.push('p'),
                Tile::BlackKnight   => str.push('n'),
                Tile::BlackBishop   => str.push('b'),
                Tile::BlackRook     => str.push('r'),
                Tile::BlackQueen    => str.push('q'),
                Tile::BlackKing     => str.push('k'),
                Tile::Empty         => str.push('.'),
                Tile::Sentinel      => (),
            }
            None => (),
        };

        println!("{str}");
    }
    */
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
