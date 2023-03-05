#![allow(dead_code)]
#![allow(unused_variables)]

pub const BOARD_SIZE: usize = 20;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn other(&self) -> Stone {
        match *self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    stones: [Option<Stone>; BOARD_SIZE * BOARD_SIZE],
}

use std::fmt;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        // Print the board as a grid of characters
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                match self.stones[i * BOARD_SIZE + j] {
                    Some(Stone::Black) => s.push('●'),
                    Some(Stone::White) => s.push('○'),
                    None => s.push('+'),
                }

                if j < BOARD_SIZE - 1 {
                    s.push_str("---");
                }
            }

            s.push('\n');

            if i < BOARD_SIZE - 1 {
                for _ in 0..BOARD_SIZE - 1 {
                    s.push_str("│   ");
                }
                s.push_str("│\n");
            }
        }

        write!(f, "{}", s)
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            stones: [None; BOARD_SIZE * BOARD_SIZE],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Stone> {
        self.stones[y * BOARD_SIZE + x]
    }

    pub fn is_legal(&self, x: usize, y: usize) -> bool {
        x < BOARD_SIZE && y < BOARD_SIZE && self.get(x, y).is_none()
    }

    fn set(&mut self, x: usize, y: usize, stone: Stone) {
        if self.stones[y * BOARD_SIZE + x].is_some() {
            panic!("Stone already exists at ({}, {})", x, y); // TODO: remove when done
        }
        self.stones[y * BOARD_SIZE + x] = Some(stone);
    }

    pub fn make_move(&self, x: usize, y: usize, stone: Stone) -> Board {
        let mut new_board = self.clone();
        new_board.set(x, y, stone);
        new_board
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    x: usize,
    y: usize,
    stone: Stone,
}

impl Move {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn stone(&self) -> Stone {
        self.stone
    }

    pub fn new(x: usize, y: usize, stone: Stone) -> Move {
        Move { x, y, stone }
    }
}

pub struct Game {
    pub board: Board,
    pub moves: Vec<Move>,
    pub current_turn: Stone,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            moves: Vec::new(),
            current_turn: Stone::Black,
        }
    }

    pub fn current_turn(&self) -> Stone {
        self.current_turn
    }

    pub fn make_move(&mut self, x: usize, y: usize) {
        self.board = self.board.make_move(x, y, self.current_turn);
        self.moves.push(Move::new(x, y, self.current_turn));
        self.current_turn = self.current_turn.other();
    }
}
