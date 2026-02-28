const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 20;

const PIECE_SIDE: usize = 4;

const X: bool = true;
const O: bool = false;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockColor {
    Black,
    Red,
    Blue,
    Yellow,
    Green,
    Magenta,
    Cyan,
    Orange,
}

#[derive(Clone, Copy)]
enum PieceRotation {
    North,
    East,
    West,
    South,
}

type PieceGrid = [[bool; PIECE_SIDE]; PIECE_SIDE];

#[derive(Clone, Copy)]
struct TetrisPieceData {
    data: PieceGrid,
    color: BlockColor,
    width: usize,
    height: usize,
    y_start: i8,
}

impl TetrisPieceData {
    fn get_data(&self, rotation: PieceRotation) -> TetrisPieceData {
        let mut rotated_piece = *self;

        rotated_piece.width = match rotation {
            PieceRotation::North => self.width,
            PieceRotation::West => self.height,
            PieceRotation::South => self.width,
            PieceRotation::East => self.height,
        };
        rotated_piece.height = match rotation {
            PieceRotation::North => self.height,
            PieceRotation::West => self.width,
            PieceRotation::South => self.height,
            PieceRotation::East => self.width,
        };

        rotated_piece.data = [[O; PIECE_SIDE]; PIECE_SIDE];

        for y in 0..self.height {
            for x in 0..self.width {
                let grid_y = match rotation {
                    PieceRotation::North => y,
                    PieceRotation::West => x,
                    PieceRotation::South => self.height - y - 1,
                    PieceRotation::East => self.width - x - 1,
                };
                let grid_x = match rotation {
                    PieceRotation::North => x,
                    PieceRotation::West => self.height - y - 1,
                    PieceRotation::South => self.width - x - 1,
                    PieceRotation::East => y,
                };

                rotated_piece.data[grid_y][grid_x] = self.data[y][x];
            }
        }
        rotated_piece
    }
}

#[rustfmt::skip]
const IPIECE: TetrisPieceData = TetrisPieceData {
    data: [
    [O, O, O, O],
    [X, X, X, X],
    [O, O, O, O],
    [O, O, O, O]
    ],
    color: BlockColor::Red,
    width: 4,
    height: 3,
    y_start: -1,
};
#[rustfmt::skip]
const LPIECE: TetrisPieceData = TetrisPieceData {
    data: [
    [O, O, X, O],
    [X, X, X, O],
    [O, O, O, O],
    [O, O, O, O]
    ],
    color: BlockColor::Yellow,
    width: 3,
    height: 3,
    y_start: 0,
};
#[rustfmt::skip]
const JPIECE: TetrisPieceData = TetrisPieceData {
    data: [
    [X, O, O, O],
    [X, X, X, O],
    [O, O, O, O],
    [O, O, O, O]
    ],
    color: BlockColor::Orange,
    width: 3,
    height: 3,
    y_start: 0,
};
#[rustfmt::skip]
const OPIECE: TetrisPieceData = TetrisPieceData {
    data: [
    [X, X, O, O],
    [X, X, O, O],
    [O, O, O, O],
    [O, O, O, O]
    ],
    color: BlockColor::Cyan,
    width: 2,
    height: 2,
    y_start: 0,
};
#[rustfmt::skip]
const SPIECE: TetrisPieceData = TetrisPieceData {
    data: [
    [O, X, X, O],
    [X, X, O, O],
    [O, O, O, O],
    [O, O, O, O]
    ],
    color: BlockColor::Green,
    width: 3,
    height: 2,
    y_start: 0,
};
#[rustfmt::skip]
const ZPIECE: TetrisPieceData = TetrisPieceData {
    data: [
    [X, X, O, O],
    [O, X, X, O],
    [O, O, O, O],
    [O, O, O, O]
    ],
    color: BlockColor::Magenta,
    width: 3,
    height: 2,
    y_start: 0,
};
#[rustfmt::skip]
const TPIECE: TetrisPieceData = TetrisPieceData {
    data: [
    [O, X, O, O],
    [X, X, X, O],
    [O, O, O, O],
    [O, O, O, O]
    ],
    color: BlockColor::Blue,
    width: 3,
    height: 3,
    y_start: 0,
};

const TETRISPIECES: [TetrisPieceData; 7] = [IPIECE, LPIECE, JPIECE, OPIECE, SPIECE, ZPIECE, TPIECE];

#[derive(Clone, Copy)]
struct CurrentPiece<'a> {
    piece: &'a TetrisPieceData,
    x: i8,
    y: i8,
    rotation: PieceRotation,
}

impl<'a> Default for CurrentPiece<'a> {
    fn default() -> Self {
        CurrentPiece {
            piece: &IPIECE,
            x: 0,
            y: 0,
            rotation: PieceRotation::North,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Playfield {
    data: [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT],
}

impl Playfield {
    fn draw(&mut self, current: &CurrentPiece) {
        let rotated_piece = current.piece.get_data(current.rotation);

        for y in 0..rotated_piece.height {
            for x in 0..rotated_piece.width {
                if rotated_piece.data[y][x] {
                    let grid_x = x as i8 + current.x;
                    let grid_y = y as i8 + current.y;
                    self.data[grid_y as usize][grid_x as usize] = current.piece.color;
                }
            }
        }
    }
    fn test_row(&self, row: usize) -> bool {
        self.data[row]
            .iter()
            .all(|&color| color != BlockColor::Black)
    }
    fn remove_row(&mut self, row: usize) {
        for y in (1..=row).rev() {
            self.data[y] = self.data[y - 1];
        }
        self.data[0] = [BlockColor::Black; FIELD_WIDTH];
    }
    fn test_and_remove_rows(&mut self) {
        let mut row = self.height() - 1;
        loop {
            if self.test_row(row) {
                self.remove_row(row);
            } else {
                if row == 0 {
                    break;
                }
                row -= 1;
            }
        }
    }
    fn place(&mut self, piece: CurrentPiece) {
        let rotated_piece = piece.piece.get_data(piece.rotation);
        for y in 0..rotated_piece.height {
            for x in 0..rotated_piece.width {
                if rotated_piece.data[y][x] {
                    let grid_x = x as i8 + piece.x;
                    let grid_y = y as i8 + piece.y;
                    self.data[grid_y as usize][grid_x as usize] = rotated_piece.color;
                }
            }
        }
        self.test_and_remove_rows()
    }
    fn try_piece(&self, piece: CurrentPiece) -> bool {
        let rotated_piece = piece.piece.get_data(piece.rotation);

        for y in 0..rotated_piece.height {
            for x in 0..rotated_piece.width {
                if rotated_piece.data[y][x] {
                    let grid_x = x as i8 + piece.x;
                    let grid_y = y as i8 + piece.y;
                    if grid_x < 0 || grid_x >= self.width() as i8 {
                        return false;
                    }
                    if grid_y < 0 || grid_y >= self.height() as i8 {
                        return false;
                    }
                    if self.data[grid_y as usize][grid_x as usize] != BlockColor::Black {
                        return false;
                    }
                }
            }
        }

        true
    }
    pub fn width(&self) -> usize {
        FIELD_WIDTH
    }
    pub fn height(&self) -> usize {
        FIELD_HEIGHT
    }
    pub fn data(&self) -> &[[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT] {
        &self.data
    }
}

impl Default for Playfield {
    fn default() -> Self {
        Playfield {
            data: [[BlockColor::Black; FIELD_WIDTH]; FIELD_HEIGHT],
        }
    }
}

#[derive(Default)]
pub struct TetrisState {
    level: u8,
    field: Playfield,
    current: CurrentPiece<'static>,
    game_over: bool,
}

impl TetrisState {
    pub fn new(level: u8) -> TetrisState {
        let mut state = TetrisState::default();
        state.restart(level);
        state
    }
    pub fn restart(&mut self, level: u8) {
        self.game_over = false;
        self.level = level;
        self.field = Playfield::default();
        self.new_piece();
    }
    fn new_piece(&mut self) {
        let rand_val: usize = (rand::random::<u8>() as usize) % TETRISPIECES.len();
        let piece = &TETRISPIECES[rand_val];
        self.current = CurrentPiece {
            piece,
            x: ((FIELD_WIDTH - piece.width) / 2) as i8,
            y: piece.y_start,
            rotation: PieceRotation::North,
        };
        self.game_over = !self.field.try_piece(self.current);
    }
    pub fn rotate_ccw(&mut self) {
        let mut piece = self.current;
        piece.rotation = match self.current.rotation {
            PieceRotation::North => PieceRotation::West,
            PieceRotation::West => PieceRotation::South,
            PieceRotation::South => PieceRotation::East,
            PieceRotation::East => PieceRotation::North,
        };
        if self.field.try_piece(piece) {
            self.current = piece;
        }
    }
    pub fn rotate_cw(&mut self) {
        let mut piece = self.current;
        piece.rotation = match self.current.rotation {
            PieceRotation::North => PieceRotation::East,
            PieceRotation::East => PieceRotation::South,
            PieceRotation::South => PieceRotation::West,
            PieceRotation::West => PieceRotation::North,
        };
        if self.field.try_piece(piece) {
            self.current = piece;
        }
    }
    pub fn move_left(&mut self) {
        let mut piece = self.current;
        piece.x -= 1;
        if self.field.try_piece(piece) {
            self.current = piece;
        }
    }
    pub fn move_right(&mut self) {
        let mut piece = self.current;
        piece.x += 1;
        if self.field.try_piece(piece) {
            self.current = piece;
        }
    }
    fn drop_one_line(&mut self) -> bool {
        let mut piece = self.current;
        piece.y += 1;
        if self.field.try_piece(piece) {
            self.current = piece;
            return true;
        }
        false
    }
    pub fn tick(&mut self) {
        if !self.drop_one_line() {
            self.field.place(self.current);
            self.new_piece();
        }
    }
    pub fn drop(&mut self) {
        while self.drop_one_line() {}
        self.field.place(self.current);
        self.new_piece();
    }
    pub fn field(&self) -> Playfield {
        let mut field = self.field;
        field.draw(&self.current);
        field
    }
    pub fn level(&self) -> u8 {
        self.level
    }
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}
