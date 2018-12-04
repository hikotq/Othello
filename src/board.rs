use std::fmt;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn equal(color1: &Color, color2: &Color) -> bool {
        !(color1.is_black() ^ color2.is_black())
    }

    pub fn is_black(&self) -> bool {
        use self::Color;
        match *self {
            Color::Black => true,
            _ => false,
        }
    }

    pub fn is_white(&self) -> bool {
        !self.is_black()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Empty,
    Available,
    Piece(self::Color),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        use self::Cell::Empty;
        match *self {
            Empty => true,
            _ => false,
        }
    }

    pub fn is_available(&self) -> bool {
        use self::Cell::Available;
        match *self {
            Available => true,
            _ => false,
        }
    }

    pub fn is_piece(&self) -> bool {
        use self::Cell::Piece;
        match *self {
            Piece(_) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Add for Point<i32> {
    type Output = Option<Point<usize>>;

    fn add(self, other: Point<i32>) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        if x < 0 || x > 8 || y < 0 || y > 8 {
            return None;
        }
        let p = Point {
            x: x as usize,
            y: y as usize,
        };
        Some(p)
    }
}

impl Add<Point<i32>> for Point<usize> {
    type Output = Option<Point<usize>>;

    fn add(self, other: Point<i32>) -> Self::Output {
        let x = self.x as i32 + other.x;
        let y = self.y as i32 + other.y;
        if x < 0 || x > 7 || y < 0 || y > 7 {
            return None;
        }
        let p = Point {
            x: x as usize,
            y: y as usize,
        };
        Some(p)
    }
}

pub struct Board {
    cell_table: [Cell; 64],
}

impl fmt::Debug for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.cell_table[..].fmt(formatter)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

impl Move {
    fn new(x: i32, y: i32, color: Color) -> Result<Self, String> {
        if x < 0 || x > 8 || y < 0 || y > 8 {
            return Err("Invalid Point".to_string());
        }
        Ok(Move {
            x: x as usize,
            y: y as usize,
            color,
        })
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            cell_table: [self::Cell::Empty; 64],
        }
    }

    pub fn all_points() -> Vec<Point<usize>> {
        let mut v = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                v.push(Point { x, y });
            }
        }
        v
    }

    pub fn get_cell(&self, p: Point<usize>) -> Cell {
        self.cell_table[p.y * 8 + p.x]
    }

    pub fn set_cell(&mut self, p: Point<usize>, cell: Cell) {
        use self::Cell;
        self.cell_table[p.y * 8 + p.x] = cell;
    }

    pub fn show(&self) {
        for y in 0..8 {
            for x in 0..8 {
                let s = match self.get_cell(Point { x, y }) {
                    Cell::Piece(Color::Black) => "○",
                    Cell::Piece(Color::White) => "●",
                    Cell::Empty => "□",
                    Cell::Available => "×",
                };
                print!("{}", s);
            }
            println!("");
        }
    }
}
