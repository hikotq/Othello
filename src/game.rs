use crate::board::{Board, Cell, Color, Move, Pos};

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    turn: Color,
    dir: Vec<Point<i32>>,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new();
        board.set_cell(Pos { x: 3, y: 3 }, Cell::Piece(Color::Black));
        board.set_cell(Pos { x: 4, y: 4 }, Cell::Piece(Color::Black));
        board.set_cell(Pos { x: 3, y: 4 }, Cell::Piece(Color::White));
        board.set_cell(Pos { x: 4, y: 3 }, Cell::Piece(Color::White));

        let dir_v = vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        let dir_v = dir_v
            .into_iter()
            .map(|(x, y)| Point { x, y })
            .collect::<Vec<Point<i32>>>();
        let mut dir = [Point { x: 0, y: 0 }; 8];
        Self {
            board: board,
            turn: Color::Black,
            dir: dir_v,
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
