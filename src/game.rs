use crate::board::{Board, Cell, Color, Move, Pos};

type Winner = Option<Color>;

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub is_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new();
        board.set_cell(Pos { x: 3, y: 3 }, Cell::Piece(Color::Black));
        board.set_cell(Pos { x: 4, y: 4 }, Cell::Piece(Color::Black));
        board.set_cell(Pos { x: 3, y: 4 }, Cell::Piece(Color::White));
        board.set_cell(Pos { x: 4, y: 3 }, Cell::Piece(Color::White));

        Self {
            board: board,
            turn: Color::Black,
            is_over: false,
        }
    }

    pub fn change_turn(&mut self) {
        self.turn = match self.turn {
            Color::Black => Color::White,
            _ => Color::Black,
        }
    }

    pub fn winner(&self) -> Winner {
        let (black, white) = self.board.count_piece();
        if black > white {
            Some(Color::Black)
        } else if white > black {
            Some(Color::White)
        } else {
            None
        }
    }

    pub fn put_piece(&mut self, m: Move) -> Result<(), String> {
        if !self.board.get_cell(Pos { x: m.x, y: m.y }).is_available() {
            return Err("Not Available Cell".to_string());
        }
        let pos = Pos { x: m.x, y: m.y };
        self.board.set_cell(pos, Cell::Piece(m.color));
        self.flip(pos);
        Ok(())
    }

    pub fn update_available_cell(&mut self) {
        for pos in Board::all_pos()
            .into_iter()
            .filter(|&p| !self.board.get_cell(p).is_piece())
            .collect::<Vec<Pos<usize>>>()
        {
            if Rule::can_put(&self.board, pos, self.turn) {
                self.board.set_cell(pos, Cell::Available);
            } else if self.board.get_cell(pos).is_available() {
                self.board.set_cell(pos, Cell::Empty);
            }
        }
    }

    pub fn flip(&mut self, pos: Pos<usize>) {
        for d in &self.board.dir.clone() {
            let p = pos + *d;
            if p.is_none() {
                continue;
            }
            if let Cell::Piece(color) = self.board.get_cell(p.unwrap()) {
                if Color::equal(&self.turn, &color) {
                    continue;
                }
            }
            self.flip_recursive(p, *d).ok();
        }
    }

    pub fn flip_recursive(&mut self, pos: Option<Pos<usize>>, d: Pos<i32>) -> Result<(), String> {
        if let Some(pos) = pos {
            if let Cell::Piece(color) = self.board.get_cell(pos) {
                if Color::equal(&self.turn, &color) {
                    return Ok(());
                } else {
                    if self.flip_recursive(pos + d, d).is_ok() {
                        self.board.set_cell(pos, Cell::Piece(self.turn));
                        return Ok(());
                    }
                }
            }
        }
        Err("out of board".to_string())
    }
}
