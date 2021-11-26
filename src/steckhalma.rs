#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    Invalid,
    Peg,
    NoPeg,
}

const I: Position = Position::Invalid;
const P: Position = Position::Peg;
const N: Position = Position::NoPeg;

// 24+9 = 33
#[derive(Clone)]
pub struct Board {
    positions: [Position; 49],
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

const INITIAL_BOARD: Board = Board {
    #[rustfmt::skip]
    positions: [
        I, I, P, P, P, I, I,
        I, I, P, P, P, I, I,
        P, P, P, P, P, P, P,
        P, P, P, N, P, P, P,
        P, P, P, P, P, P, P,
        I, I, P, P, P, I, I,
        I, I, P, P, P, I, I,
    ],
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MoveErr {
    InvalidFrom,
    InvalidTo,
    NoPegAtFrom,
    NotAJump,
    PegAtTo,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Pos {
    pub fn shift(&self, dir: Direction, count: usize) -> Pos {
        match dir {
            Direction::Up => Pos {
                x: self.x,
                y: self.y.wrapping_sub(count),
            },
            Direction::Down => Pos {
                x: self.x,
                y: self.y.wrapping_add(count),
            },
            Direction::Left => Pos {
                x: self.x.wrapping_sub(count),
                y: self.y,
            },
            Direction::Right => Pos {
                x: self.x.wrapping_add(count),
                y: self.y,
            },
        }
    }

    pub fn in_range(&self) -> bool {
        self.x < 7 && self.y < 7
    }
}

impl Board {
    pub fn new() -> Board {
        INITIAL_BOARD.clone()
    }

    pub fn do_move(&mut self, from: Pos, direction: Direction) -> Result<(), MoveErr> {
        match self.at(from) {
            Position::Invalid => return Err(MoveErr::InvalidFrom),
            Position::NoPeg => return Err(MoveErr::NoPegAtFrom),
            Position::Peg => (),
        }
        let jump_over = from.shift(direction, 1);
        if !jump_over.in_range() {
            return Err(MoveErr::InvalidTo);
        }
        match self.at(jump_over) {
            Position::Invalid => return Err(MoveErr::InvalidTo),
            Position::NoPeg => return Err(MoveErr::NotAJump),
            Position::Peg => (),
        }
        let jump_to = from.shift(direction, 2);
        if !jump_over.in_range() {
            return Err(MoveErr::InvalidTo);
        }
        match self.at(jump_to) {
            Position::Invalid => Err(MoveErr::InvalidTo),
            Position::NoPeg => {
                *self.mut_at(from) = Position::NoPeg;
                *self.mut_at(jump_over) = Position::NoPeg;
                *self.mut_at(jump_to) = Position::Peg;
                Ok(())
            }
            Position::Peg => Err(MoveErr::PegAtTo),
        }
    }
    pub fn at(&self, pos: Pos) -> Position {
        self.positions[pos.y * 7 + pos.x]
    }
    pub fn mut_at(&mut self, pos: Pos) -> &mut Position {
        &mut self.positions[pos.y * 7 + pos.x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let mut board = Board::new();
        let from = Pos::new(3, 1);
        assert_eq!(board.do_move(from, Direction::Down), Ok(()));
        assert_eq!(board.at(from), Position::NoPeg);
        assert_eq!(board.at(from.shift(Direction::Down, 1)), Position::NoPeg);
        assert_eq!(board.at(from.shift(Direction::Down, 2)), Position::Peg);
    }
}
