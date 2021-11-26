#![no_std]

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EndGameState {
    PlayerWon,
    PlayerLost,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Resetting,
    MovingCursor,
    MovingPeg,
    GameOver(EndGameState),
}

impl GameState {

    pub fn space_pressed(&mut self) {
        match self {
            GameState::MovingCursor => *self = GameState::MovingPeg,
            GameState::MovingPeg => *self = GameState::MovingCursor,
            GameState::GameOver(..) => *self = GameState::Resetting,
            _other => {},
        };
    }
}

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

const INITIAL_BOARD_EASY: Board = Board {
    #[rustfmt::skip]
    positions: [
        I, I, N, N, N, I, I,
        I, I, N, N, N, I, I,
        N, N, N, N, N, N, N,
        N, N, N, N, N, N, N,
        N, P, P, N, P, N, N,
        I, I, N, N, N, I, I,
        I, I, N, N, N, I, I,
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
        INITIAL_BOARD_EASY.clone()
    }

    fn check_move(&self, from: Pos, direction: Direction) -> Result<(Pos, Pos), MoveErr> {

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
        if !jump_to.in_range() {
            return Err(MoveErr::InvalidTo);
        }

        match self.at(jump_to) {
            Position::Invalid => Err(MoveErr::InvalidTo),
            Position::NoPeg => Ok((jump_over, jump_to)),
            Position::Peg => Err(MoveErr::PegAtTo),
        }
    }

    pub fn do_move(&mut self, from: Pos, direction: Direction) -> Result<(Pos, Pos), MoveErr> {
        let (jump_over, jump_to) = self.check_move(from, direction)?;

        *self.mut_at(from) = Position::NoPeg;
        *self.mut_at(jump_over) = Position::NoPeg;
        *self.mut_at(jump_to) = Position::Peg;

        Ok((jump_over, jump_to))
    }

    pub fn at(&self, pos: Pos) -> Position {
        self.positions[pos.y * 7 + pos.x]
    }

    pub fn mut_at(&mut self, pos: Pos) -> &mut Position {
        &mut self.positions[pos.y * 7 + pos.x]
    }

    pub fn check_game_over(&self) -> Option<EndGameState> {

        let mut peg_counter = 0;

        for x in 0..7 {
            for y in 0..7 {
                let position = Pos::new(x, y);

                if self.at(position) == Position::Peg {
                    peg_counter += 1;

                    for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                        if self.check_move(position, direction).is_ok() {
                            return None;
                        }
                    }
                }
            }
        }

        match peg_counter {
            1 => Some(EndGameState::PlayerWon),
            _more => Some(EndGameState::PlayerLost),
        }
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
