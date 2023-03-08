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
    white_board: [bool; BOARD_SIZE * BOARD_SIZE],
    black_board: [bool; BOARD_SIZE * BOARD_SIZE],
    current_player: Stone,
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
            white_board: [false; BOARD_SIZE * BOARD_SIZE],
            black_board: [false; BOARD_SIZE * BOARD_SIZE],
            current_player: Stone::Black,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Stone> {
        self.stones[y * BOARD_SIZE + x]
    }

    pub fn is_legal(&self, x: usize, y: usize) -> bool {
        x < BOARD_SIZE && y < BOARD_SIZE && self.get(x, y).is_none()
    }

    fn set(&mut self, x: usize, y: usize, stone: Option<Stone>) {
        let index = y * BOARD_SIZE + x;
        if self.stones[index].is_some() {
            panic!("Stone already exists at ({}, {})", x, y); // TODO: remove when done
        }
        self.stones[index] = stone;
        match stone {
            Some(Stone::Black) => self.black_board[index] = true,
            Some(Stone::White) => self.white_board[index] = true,
            None => {
                // To remove a stone
                self.black_board[index] = false;
                self.white_board[index] = false;
            }
        }
    }

    pub fn make_move(&self, x: usize, y: usize, stone: Option<Stone>) -> Board {
        let mut new_board = self.clone();
        new_board.set(x, y, stone);
        new_board
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GoMove {
    pub x: usize,
    pub y: usize,
    pub stone: Stone,
}

impl minimax::Move for GoMove {
    fn apply(&self, board: &mut Board) {
        board.make_move(self.x, self.y, Some(self.stone));
    }

    type G = GoGame;

    fn undo(&self, state: &mut <Self::G as minimax::Game>::S) {
        state.make_move(self.x, self.y, None);
    }
}

impl GoMove {
    pub fn new(x: usize, y: usize, stone: Stone) -> GoMove {
        GoMove { x, y, stone }
    }
}

pub struct GoGame {
    pub board: Board,
    pub moves: Vec<GoMove>,
    pub current_turn: Stone,
}

impl minimax::Game for GoGame {
    type S = Board;

    type M = GoMove;

    fn generate_moves(state: &Self::S, moves: &mut Vec<Self::M>) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if state.get(i, j).is_none() {
                    moves.push(GoMove::new(i, j, state.current_player));
                }
            }
        }
    }

    fn get_winner(state: &Self::S) -> Option<minimax::Winner> {
        match state.current_player {
            Stone::Black => {
                if is_won(&state.black_board) {
                    Some(minimax::Winner::PlayerJustMoved) // TODO: check if this is correct
                } else {
                    None
                }
            }
            Stone::White => {
                if is_won(&state.white_board) {
                    Some(minimax::Winner::PlayerJustMoved) // TODO: check if this is correct
                } else {
                    None
                }
            }
        }
    }
}

impl GoGame {
    pub fn new() -> GoGame {
        GoGame {
            board: Board::new(),
            moves: Vec::new(),
            current_turn: Stone::Black,
        }
    }

    pub fn current_turn(&self) -> Stone {
        self.current_turn
    }

    pub fn make_move(&mut self, x: usize, y: usize) {
        self.board = self.board.make_move(x, y, Some(self.current_turn));
        self.moves.push(GoMove::new(x, y, self.current_turn));
        self.current_turn = self.current_turn.other();
    }
}

fn is_won(board: &[bool; BOARD_SIZE * BOARD_SIZE]) -> bool {
    todo!("Implement is_won function");
}
