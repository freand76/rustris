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

type PieceGrid = [[u8; PIECE_SIDE]; PIECE_SIDE];

struct TetrisPieceData {
    pub data: PieceGrid,
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

enum PieceRotation {
    NORTH,
    EAST,
    WEST,
    SOUTH,
}

pub struct CurrentPiece {
    pub piece: TetrisPieceData,
    x: usize,
    y: usize,
    rotation: PieceRotation,
}

#[derive(Clone, Copy)]
pub struct Playfield {
    pub data: [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT],
}

pub trait PieceDrawer {
    fn draw(&mut self, piece: &CurrentPiece);
}

impl PieceDrawer for Playfield {
    fn draw(&mut self, piece: &CurrentPiece) {
        for y in 0..PIECE_SIDE {
            for x in 0..PIECE_SIDE {
                if piece.piece.data[y][x] == 1 {
                    self.data[y + piece.y][x + piece.x] = piece.piece.color;
                }
            }
        }
    }
}

fn empty_field() -> Playfield {
    Playfield {
        data: [[BlockColor::Black; FIELD_WIDTH]; FIELD_HEIGHT],
    }
}

pub struct TetrisState {
    level: u8,
    pub field: Playfield,
    pub current: CurrentPiece,
}

pub fn create_tetris_game(level: u8) -> TetrisState {
    TetrisState {
        level: level,
        field: empty_field(),
        current: CurrentPiece {
            piece: IPIECE,
            x: 5,
            y: 0,
            rotation: PieceRotation::NORTH,
        },
    }
}

pub trait ControlInterface {
    fn rotate_ccw(&mut self);
    fn rotate_cw(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn drop(&mut self);
}

impl ControlInterface for TetrisState {
    fn rotate_ccw(&mut self) {
        self.current.rotation = match self.current.rotation {
            PieceRotation::NORTH => PieceRotation::WEST,
            PieceRotation::WEST => PieceRotation::SOUTH,
            PieceRotation::SOUTH => PieceRotation::EAST,
            PieceRotation::EAST => PieceRotation::NORTH,
        };
    }
    fn rotate_cw(&mut self) {
        self.current.rotation = match self.current.rotation {
            PieceRotation::NORTH => PieceRotation::EAST,
            PieceRotation::EAST => PieceRotation::SOUTH,
            PieceRotation::SOUTH => PieceRotation::WEST,
            PieceRotation::WEST => PieceRotation::NORTH,
        };
    }
    fn move_left(&mut self) {
        self.current.x -= 1;
    }
    fn move_right(&mut self) {
        self.current.x += 1;
    }
    fn drop(&mut self) {
        self.current.y += 1;
    }
}
