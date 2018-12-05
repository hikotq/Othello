use crate::board::{Board, Cell, Color, Move, Pos};
use crate::rule::Rule;

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
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
        }
    }

    fn change_turn(&mut self) {
        self.turn = match self.turn {
            Color::Black => Color::White,
            _ => Color::Black,
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
            if let Cell::Piece(color) = self.board.get_cell(point) {
                if !Color::equal(&self.turn, &color) {
                    break;
                }
            }
            for d in self.dir.iter() {
                let mut dir_p = point + *d;
                match dir_p {
                    Some(p) => {
                        match self.board.get_cell(p) {
                            Cell::Piece(color) => {
                                if Color::equal(&self.turn, &color) {
                                    continue;
                                }
                            }
                            _ => continue,
                        }
                        dir_p = p + *d;
                    }
                    _ => continue,
                }
                while let Some(p) = dir_p {
                    if let Cell::Piece(color) = self.board.get_cell(p) {
                        if Color::equal(&self.turn, &color) {
                            self.board.set_cell(point, Cell::Available);
                            break;
                        }
                    } else {
                        break;
                    }
                    dir_p = p + *d;
                }
            }
        }
    }
}
