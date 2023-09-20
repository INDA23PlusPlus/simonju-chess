macro_rules! tile {
    (P) => { Tile::WhitePawn };
    (N) => { Tile::WhiteKnight };
    (B) => { Tile::WhiteBishop };
    (R) => { Tile::WhiteRook };
    (Q) => { Tile::WhiteQueen };
    (K) => { Tile::WhiteKing };
    (p) => { Tile::BlackPawn };
    (n) => { Tile::BlackKnight };
    (b) => { Tile::BlackBishop };
    (r) => { Tile::BlackRook };
    (q) => { Tile::BlackQueen };
    (k) => { Tile::BlackKing };
    (.) => { Tile::Empty };
    (_) => { Tile::Sentinel };
}

pub(crate) use tile;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 12;

#[derive(Debug, Clone)]
pub struct BoardError;

impl std::fmt::Display for BoardError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "invalid board operation")
    }
}

#[derive(Copy, Clone)]
pub enum Tile {
    WhitePawn,
    WhiteBishop,
    WhiteKnight,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackBishop,
    BlackKnight,
    BlackRook,
    BlackQueen,
    BlackKing,
    Empty,
    Sentinel,
}

pub struct Pos {
    pub x: i32,
    pub y: i32,
}

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
                Tile::Sentinel => Err(BoardError),
                _ => { *tile = piece; Ok(()) },
            },
            None => Err(BoardError),
        }
    }

    pub fn rem_tile(&mut self, index: usize) -> Result<Tile, BoardError> {
        match self.tiles.get_mut(index) {
            Some(tile) => match tile {
                // Sentinel tiles cannot be removed.
                Tile::Sentinel => Err(BoardError),
                _ => { 
                    let val = tile.clone();
                    *tile = Tile::Empty; 
                    Ok(val)
                },
            },
            None => Err(BoardError),
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
            if (index < 20) | (index >= 100) | (index % 12 == 0) | (index % 12 == 11) {
                *tile = tile!(_);
            }
        }

        board
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
    fn board_init() {
    }
}