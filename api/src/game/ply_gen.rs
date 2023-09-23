use crate::board::*;

use super::Game;

#[derive(Debug)]
pub enum PlyError {
    InvalidPly,
    Unknown,
}

impl std::fmt::Display for PlyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "invalid ply")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ply {
    pub origin: usize,
    pub destination: usize,
}

impl std::fmt::Display for Ply {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();

        let origin_string = Board::index_to_string(self.origin);
        let destination_string = Board::index_to_string(self.destination);

        string.push_str(origin_string.as_str());
        string.push(' ');
        string.push_str(destination_string.as_str());

        write!(f, "{}", string)
    }
}

impl Ply {
    pub(crate) fn to_string(&self) -> String {
        let mut string = String::new();

        let origin_string = Board::index_to_string(self.origin);
        let destination_string = Board::index_to_string(self.destination);

        string.push_str(origin_string.as_str());
        string.push(' ');
        string.push_str(destination_string.as_str());

        string
    }
}

impl Game {
    pub(crate) fn gen_plys(&mut self) {
        let mut plys: Vec<Ply> = Vec::new();

        // Pseudo-legal ply generation.
        match self.player {
            Color::White => for (index, tile) in self.board.get_tiles().iter().enumerate() {
                match tile {
                    tile!(P) => plys.append(&mut self.gen_pawn_plys(index)),
                    tile!(N) => plys.append(&mut self.gen_knight_plys(index)),
                    tile!(B) => plys.append(&mut self.gen_bishop_plys(index)),
                    tile!(R) => plys.append(&mut self.gen_rook_plys(index)),
                    tile!(Q) => plys.append(&mut self.gen_queen_plys(index)),
                    tile!(K) => plys.append(&mut self.gen_king_plys(index)),
                    _ => (),
                }
            }
            Color::Black => for (index, tile) in self.board.get_tiles().iter().enumerate() {
                match tile {
                    tile!(p) => plys.append(&mut self.gen_pawn_plys(index)),
                    tile!(n) => plys.append(&mut self.gen_knight_plys(index)),
                    tile!(b) => plys.append(&mut self.gen_bishop_plys(index)),
                    tile!(r) => plys.append(&mut self.gen_rook_plys(index)),
                    tile!(q) => plys.append(&mut self.gen_queen_plys(index)),
                    tile!(k) => plys.append(&mut self.gen_king_plys(index)),
                    _ => (),
                }
            }
        }

        // Legal ply generation through pruning illegal plys.
        self.plys.clear();
        self.plys.append(&mut plys);
    }

    fn gen_pawn_plys(&self, origin: usize) -> Vec<Ply> {
        let mut plys: Vec<Ply> = Vec::new();

        let can_double_push = match self.player {
            Color::White => Board::get_pos(origin).rank == 1,
            Color::Black => Board::get_pos(origin).rank == 6,
        };

        let mut destination = match self.player {
            Color::White => origin + 10,
            Color::Black => origin - 10,
        };

        match self.board.get_tile(destination) {
            Some(tile) => match tile {
                tile!(.) => {
                    plys.push(Ply{ origin, destination });
                    if can_double_push {
                        match self.player {
                            Color::White => destination += 10,
                            Color::Black => destination -= 10,
                        };
                        match self.board.get_tile(destination) {
                            Some(tile) => match tile {
                                tile!(.) => {
                                    plys.push(Ply{ origin, destination })
                                },
                                _ => (),
                            } 
                            _ => (),
                        }
                    }
                },
                _ => (),
            } 
            _ => (),
        }

        destination = match self.player {
            Color::White => origin + 9,
            Color::Black => origin - 11,
        };

        match self.board.get_tile(destination) {
            Some(tile) => match tile {
                tile!(white) => if self.player == Color::Black {
                    plys.push(Ply{ origin, destination })
                }
                tile!(black) => if self.player == Color::White {
                    plys.push(Ply{ origin, destination })
                }
                _ => (),
            },
            None => (),
        }

        destination = match self.player {
            Color::White => origin + 11,
            Color::Black => origin - 9,
        };

        match self.board.get_tile(destination) {
            Some(tile) => match tile {
                tile!(white) => if self.player == Color::Black {
                    plys.push(Ply{ origin, destination })
                }
                tile!(black) => if self.player == Color::White {
                    plys.push(Ply{ origin, destination })
                }
                _ => (),
            },
            None => (),
        }
        
        // Push X
        // Double Push X
        // Capture X
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

        plys
    }

    fn gen_bishop_plys(&self, origin: usize) -> Vec<Ply> {
        let bishop_delta = [-11, -9, 9, 11];
        /* +9  . +11
         *  .  O  .
         * -11 . -9
         */

        let mut plys: Vec<Ply> = Vec::new();

        for direction in bishop_delta {
            let mut i = 1;
            'slide: loop {
                let destination = (direction * i + origin as isize) as usize;
                match self.board.get_tile(destination) {
                    Some(tile) => match tile {
                        tile!(white) => match self.player {
                            Color::White => break 'slide,
                            Color::Black =>  {
                                plys.push(Ply{ origin, destination });
                                break 'slide
                            },
                        },
                        tile!(black) => match self.player {
                            Color::White => {
                                plys.push(Ply{ origin, destination });
                                break 'slide
                            },
                            Color::Black => break 'slide,
                        },
                        tile!(.) => plys.push(Ply{ origin, destination }),
                        tile!(_) => break 'slide,
                    } 
                    _ => (),
                }
                i += 1;
            }
        }

        plys
    }

    fn gen_knight_plys(&self, origin: usize) -> Vec<Ply> {
        let knight_delta = [-21, -19, -12, -8, 8, 12, 19, 21];
        /*  . +19 . +21 .
         * +8  .  .  . +12
         *  .  .  O  .  .
         * -12 .  .  . -8
         *  . -21 . -19 .
         */

        let mut plys: Vec<Ply> = Vec::new();

        for destination in knight_delta.iter().map(|i| (i + origin as isize) as usize) {
            match self.board.get_tile(destination) {
                Some(tile) => match tile {
                    tile!(white) => match self.player {
                        Color::White => (),
                        Color::Black => plys.push(Ply{ origin, destination }),
                    },
                    tile!(black) => match self.player {
                        Color::White => plys.push(Ply{ origin, destination }),
                        Color::Black => (),
                    },
                    tile!(.) => plys.push(Ply{ origin, destination }),
                    tile!(_) => (),
                } 
                _ => (),
            }
        }

        plys
    }

    fn gen_rook_plys(&self, origin: usize) -> Vec<Ply> {
        let rook_delta = [-10, -1, 1, 10];
        /*  . +10 .
         * -1  O +1
         *  . -10 .
         */

        let mut plys: Vec<Ply> = Vec::new();

        for direction in rook_delta {
            let mut i = 1;
            'slide: loop {
                let destination = (direction * i + origin as isize) as usize;
                match self.board.get_tile(destination) {
                    Some(tile) => match tile {
                        tile!(white) => match self.player {
                            Color::White => break 'slide,
                            Color::Black => {
                                plys.push(Ply{ origin, destination });
                                break 'slide
                            },
                        },
                        tile!(black) => match self.player {
                            Color::White => {
                                plys.push(Ply{ origin, destination });
                                break 'slide
                            },
                            Color::Black => break 'slide,
                        },
                        tile!(.) => plys.push(Ply{ origin, destination }),
                        tile!(_) => break 'slide,
                    } 
                    _ => (),
                }
                i += 1;
            }
        }

        plys
    }

    fn gen_queen_plys(&self, origin: usize) -> Vec<Ply> {
        let mut plys: Vec<Ply> = Vec::new();

        plys.append(&mut self.gen_bishop_plys(origin));
        plys.append(&mut self.gen_rook_plys(origin));

        plys
    }

    fn gen_king_plys(&self, origin: usize) -> Vec<Ply> {
        let king_delta = [-11, -10, -9, -1, 1, 9, 10, 11];

        let mut plys: Vec<Ply> = Vec::new();

        for destination in king_delta.iter().map(|i| (i + origin as isize) as usize) {
            match self.board.get_tile(destination) {
                Some(tile) => match tile {
                    tile!(white) => match self.player {
                        Color::White => (),
                        Color::Black => plys.push(Ply{ origin, destination }),
                    },
                    tile!(black) => match self.player {
                        Color::White => plys.push(Ply{ origin, destination }),
                        Color::Black => (),
                    },
                    tile!(.) => plys.push(Ply{ origin, destination }),
                    tile!(_) => (),
                } 
                _ => (),
            }
        }

        // Step
        // Castling
        /* +9 +10+11
         * -1  O +1
         * -11-10-9
         */

        plys
    }
}