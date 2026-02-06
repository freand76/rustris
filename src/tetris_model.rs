const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 20;

const PIECE_SIDE: usize = 4;

#[derive(Clone, Copy)]
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
    start_diff: usize,
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
                    PieceRotation::NORTH => y,
                    PieceRotation::WEST => x,
                    PieceRotation::SOUTH => self.height - y - 1,
                    PieceRotation::EAST => self.width - x - 1,
                };
                let grid_x = match rotation {
                    PieceRotation::NORTH => x,
                    PieceRotation::WEST => self.height - y - 1,
                    PieceRotation::SOUTH => self.width - x - 1,
                    PieceRotation::EAST => y,
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
    start_diff: 0,
};
const LPIECE: TetrisPieceData = TetrisPieceData {
    data: [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Yellow,
    width: 3,
    height: 2,
    start_diff: 0,
};
const JPIECE: TetrisPieceData = TetrisPieceData {
    data: [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Orange,
    width: 3,
    height: 2,
    start_diff: 0,
};
const OPIECE: TetrisPieceData = TetrisPieceData {
    data: [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Cyan,
    width: 2,
    height: 2,
    start_diff: 0,
};
const SPIECE: TetrisPieceData = TetrisPieceData {
    data: [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Green,
    width: 3,
    height: 2,
    start_diff: 0,
};
const ZPIECE: TetrisPieceData = TetrisPieceData {
    data: [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Magenta,
    width: 3,
    height: 2,
    start_diff: 0,
};
const TPIECE: TetrisPieceData = TetrisPieceData {
    data: [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Blue,
    width: 3,
    height: 2,
    start_diff: 0,
};

const NUM_TETRISPIECES: usize = 7;
const TETRISPIECES: [TetrisPieceData; NUM_TETRISPIECES] =
    [IPIECE, LPIECE, JPIECE, OPIECE, SPIECE, ZPIECE, TPIECE];

struct CurrentPiece {
    piece: TetrisPieceData,
    x: usize,
    y: usize,
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
                    self.data[y + current.y][x + current.x] = current.piece.color;
                }
            }
        }
    }
    pub fn width(self) -> usize {
        FIELD_WIDTH
    }
    pub fn height(self) -> usize {
        FIELD_HEIGHT
    }
    pub fn get_data(self) -> [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT] {
        self.data
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
    current: CurrentPiece,
}

pub fn create_tetris_game(level: u8) -> TetrisState {
    let rand_val: usize = (rand::random::<u8>() as usize) % NUM_TETRISPIECES;

    TetrisState {
        level: level,
        field: empty_field(),
        current: CurrentPiece {
            piece: TETRISPIECES[rand_val],
            x: 5,
            y: 3,
            rotation: PieceRotation::NORTH,
        },
    }
}

impl TetrisState {
    pub fn rotate_ccw(&mut self) {
        self.current.rotation = match self.current.rotation {
            PieceRotation::NORTH => PieceRotation::WEST,
            PieceRotation::WEST => PieceRotation::SOUTH,
            PieceRotation::SOUTH => PieceRotation::EAST,
            PieceRotation::EAST => PieceRotation::NORTH,
        };
    }
    pub fn rotate_cw(&mut self) {
        self.current.rotation = match self.current.rotation {
            PieceRotation::NORTH => PieceRotation::EAST,
            PieceRotation::EAST => PieceRotation::SOUTH,
            PieceRotation::SOUTH => PieceRotation::WEST,
            PieceRotation::WEST => PieceRotation::NORTH,
        };
    }
    pub fn move_left(&mut self) {
        self.current.x -= 1;
    }
    pub fn move_right(&mut self) {
        self.current.x += 1;
    }
    pub fn drop(&mut self) {
        self.current.y += 1;
    }
    pub fn get_field(&self) -> Playfield {
        let mut field = self.field.clone();
        field.draw(&self.current);
        return field;
    }
    pub fn get_level(&self) -> u8 {
        return self.level;
    }
}
