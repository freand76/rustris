const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 20;

const PIECE_SIDE: usize = 4;

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
    NORTH,
    EAST,
    WEST,
    SOUTH,
}

type PieceGrid = [[u8; PIECE_SIDE]; PIECE_SIDE];

#[derive(Clone, Copy)]
struct TetrisPieceData {
    data: PieceGrid,
    color: BlockColor,
    width: usize,
    height: usize,
    y_start: i8,
}

impl TetrisPieceData {
    fn get_data(self, rotation: &PieceRotation) -> TetrisPieceData {
        let mut rotated_piece = self.clone();

        rotated_piece.width = match rotation {
            PieceRotation::NORTH => self.width,
            PieceRotation::WEST => self.height,
            PieceRotation::SOUTH => self.width,
            PieceRotation::EAST => self.height,
        };
        rotated_piece.height = match rotation {
            PieceRotation::NORTH => self.height,
            PieceRotation::WEST => self.width,
            PieceRotation::SOUTH => self.height,
            PieceRotation::EAST => self.width,
        };

        rotated_piece.data = [[0; PIECE_SIDE]; PIECE_SIDE];

        for y in 0..self.height {
            for x in 0..self.width {
                let grid_y = match rotation {
                    PieceRotation::NORTH => y as usize,
                    PieceRotation::WEST => x as usize,
                    PieceRotation::SOUTH => (self.height - y - 1) as usize,
                    PieceRotation::EAST => (self.width - x - 1) as usize,
                };
                let grid_x = match rotation {
                    PieceRotation::NORTH => x as usize,
                    PieceRotation::WEST => (self.height - y - 1) as usize,
                    PieceRotation::SOUTH => (self.width - x - 1) as usize,
                    PieceRotation::EAST => y as usize,
                };

                rotated_piece.data[grid_y][grid_x] = self.data[y][x];
            }
        }
        rotated_piece
    }
}

const IPIECE: TetrisPieceData = TetrisPieceData {
    data: [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Red,
    width: 4,
    height: 3,
    y_start: -1,
};
const LPIECE: TetrisPieceData = TetrisPieceData {
    data: [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Yellow,
    width: 3,
    height: 3,
    y_start: 0,
};
const JPIECE: TetrisPieceData = TetrisPieceData {
    data: [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Orange,
    width: 3,
    height: 3,
    y_start: 0,
};
const OPIECE: TetrisPieceData = TetrisPieceData {
    data: [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Cyan,
    width: 2,
    height: 2,
    y_start: 0,
};
const SPIECE: TetrisPieceData = TetrisPieceData {
    data: [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Green,
    width: 3,
    height: 2,
    y_start: 0,
};
const ZPIECE: TetrisPieceData = TetrisPieceData {
    data: [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Magenta,
    width: 3,
    height: 2,
    y_start: 0,
};
const TPIECE: TetrisPieceData = TetrisPieceData {
    data: [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Blue,
    width: 3,
    height: 3,
    y_start: 0,
};

const NUM_TETRISPIECES: usize = 7;
const TETRISPIECES: [TetrisPieceData; NUM_TETRISPIECES] =
    [IPIECE, LPIECE, JPIECE, OPIECE, SPIECE, ZPIECE, TPIECE];

#[derive(Clone, Copy)]
struct CurrentPiece<'a> {
    piece: &'a TetrisPieceData,
    x: i8,
    y: i8,
    rotation: PieceRotation,
}

#[derive(Clone, Copy)]
pub struct Playfield {
    data: [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT],
}

impl Playfield {
    fn draw(&mut self, current: &CurrentPiece) {
        let rotated_piece = current.piece.get_data(&current.rotation);

        for y in 0..rotated_piece.height {
            for x in 0..rotated_piece.width {
                if rotated_piece.data[y][x] == 1 {
                    let grid_x = x as i8 + current.x;
                    let grid_y = y as i8 + current.y;
                    self.data[grid_y as usize][grid_x as usize] = current.piece.color;
                }
            }
        }
    }
    fn test_row(&self, row: usize) -> bool {
        for x in 0..self.width() {
            if self.data[row][x] == BlockColor::Black {
                return false;
            }
        }
        true
    }
    fn remove_row(&mut self, row: usize) {
        if row == 0 {
            self.data[row] = [BlockColor::Black; FIELD_WIDTH];
        } else {
            for y in (0..row).rev() {
                self.data[y + 1] = self.data[y];
            }
        }
    }
    fn test_and_remove_rows(&mut self) {
        for row in (0..self.height()).rev() {
            if self.test_row(row) {
                self.remove_row(row);
            }
        }
    }
    fn place(&mut self, piece: CurrentPiece) {
        let rotated_piece = piece.piece.get_data(&piece.rotation);
        for y in 0..rotated_piece.height {
            for x in 0..rotated_piece.width {
                if rotated_piece.data[y][x] == 1 {
                    let grid_x = x as i8 + piece.x;
                    let grid_y = y as i8 + piece.y;
                    self.data[grid_y as usize][grid_x as usize] = rotated_piece.color;
                }
            }
        }
        self.test_and_remove_rows()
    }
    fn try_piece(&self, piece: CurrentPiece) -> bool {
        let rotated_piece = piece.piece.get_data(&piece.rotation);

        for y in 0..rotated_piece.height {
            for x in 0..rotated_piece.width {
                if rotated_piece.data[y][x] == 1 {
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
    pub fn get_data(&self) -> &[[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT] {
        &self.data
    }
}

fn empty_field() -> Playfield {
    Playfield {
        data: [[BlockColor::Black; FIELD_WIDTH]; FIELD_HEIGHT],
    }
}

pub struct TetrisState {
    level: u8,
    field: Playfield,
    current: CurrentPiece<'static>,
    game_over: bool,
}

impl TetrisState {
    pub fn new(level: u8) -> TetrisState {
        let mut state = TetrisState {
            level: level,
            field: empty_field(),
            current: CurrentPiece {
                piece: &IPIECE,
                x: 0,
                y: 0,
                rotation: PieceRotation::NORTH,
            },
            game_over: false,
        };
        state.new_piece();
        state
    }
    fn new_piece(&mut self) {
        let rand_val: usize = (rand::random::<u8>() as usize) % NUM_TETRISPIECES;
        let piece = &TETRISPIECES[rand_val];
        self.current = CurrentPiece {
            piece: piece,
            x: ((FIELD_WIDTH - piece.width) / 2) as i8,
            y: piece.y_start,
            rotation: PieceRotation::NORTH,
        };
        self.game_over = !self.field.try_piece(self.current);
    }
    pub fn rotate_ccw(&mut self) {
        let mut piece = self.current.clone();
        piece.rotation = match self.current.rotation {
            PieceRotation::NORTH => PieceRotation::WEST,
            PieceRotation::WEST => PieceRotation::SOUTH,
            PieceRotation::SOUTH => PieceRotation::EAST,
            PieceRotation::EAST => PieceRotation::NORTH,
        };
        if self.field.try_piece(piece) {
            self.current = piece;
        }
    }
    pub fn rotate_cw(&mut self) {
        let mut piece = self.current.clone();
        piece.rotation = match self.current.rotation {
            PieceRotation::NORTH => PieceRotation::EAST,
            PieceRotation::EAST => PieceRotation::SOUTH,
            PieceRotation::SOUTH => PieceRotation::WEST,
            PieceRotation::WEST => PieceRotation::NORTH,
        };
        if self.field.try_piece(piece) {
            self.current = piece;
        }
    }
    pub fn move_left(&mut self) {
        let mut piece = self.current.clone();
        piece.x -= 1;
        if self.field.try_piece(piece) {
            self.current = piece;
        }
    }
    pub fn move_right(&mut self) {
        let mut piece = self.current.clone();
        piece.x += 1;
        if self.field.try_piece(piece) {
            self.current = piece;
        }
    }
    fn drop_one_line(&mut self) -> bool {
        let mut piece = self.current.clone();
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
    pub fn get_field(&self) -> Playfield {
        let mut field = self.field.clone();
        field.draw(&self.current);
        return field;
    }
    pub fn get_level(&self) -> u8 {
        return self.level;
    }
    pub fn is_game_over(&self) -> bool {
        return self.game_over;
    }
}
