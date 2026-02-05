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

struct TetrisPieceData {
    data: PieceGrid,
    color: BlockColor,
    width: usize,
    heigth: usize,
    start_diff: usize,
}

const IPIECE: TetrisPieceData = TetrisPieceData {
    data: [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Red,
    width: 4,
    heigth: 3,
    start_diff: 0,
};

const TETRISPIECES: [TetrisPieceData; 1] = [IPIECE];

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
    fn draw(&mut self, current_piece: &CurrentPiece) {
        for y in 0..current_piece.piece.heigth {
            for x in 0..current_piece.piece.width {
                if current_piece.piece.data[y][x] == 1 {
                    self.data[y + current_piece.y][x + current_piece.x] = current_piece.piece.color;
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
    TetrisState {
        level: level,
        field: empty_field(),
        current: CurrentPiece {
            piece: IPIECE,
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
