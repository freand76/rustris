const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 22;

const PIECE_SIDE: usize = 4;

enum BlockColor {
    Black,
    Red,
    Blue,
    Yellow,
    Green,
    Purple,
    Turquoise,
    Orange,
}

type PieceField = [[u8; PIECE_SIDE]; PIECE_SIDE];

struct TetrisPieceData {
    piecefield: PieceField,
    color: BlockColor,
    width: u8,
    heigth: u8,
    start_diff: u8,
}

const Ipiece: TetrisPieceData = TetrisPieceData {
    piecefield: [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
    color: BlockColor::Red,
    width: 4,
    heigth: 3,
    start_diff: 0,
};

const TetrisPieces: [TetrisPieceData; 1] = [Ipiece];

enum PieceRotation {
    NORTH,
    EAST,
    WEST,
    SOUTH,
}

struct CurrentPiece {
    piece: TetrisPieceData,
    x: u8,
    y: u8,
    rotation: PieceRotation,
}

type Playfield = [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT];

struct TetrisState {
    level: u8,
    field: Playfield,
    current: CurrentPiece,
}

trait ControlInterface {
    fn rotatateccw(&mut self);
    fn rotatatecw(&mut self);
    fn left(&mut self);
    fn right(&mut self);
    fn drop(&mut self);
}

impl ControlInterface for TetrisState {
    fn rotatateccw(&mut self) {
        self.current.rotation = match self.current.rotation {
            PieceRotation::NORTH => PieceRotation::WEST,
            PieceRotation::WEST => PieceRotation::SOUTH,
            PieceRotation::SOUTH => PieceRotation::EAST,
            PieceRotation::EAST => PieceRotation::NORTH,
        };
    }
    fn rotatatecw(&mut self) {
        self.current.rotation = match self.current.rotation {
            PieceRotation::NORTH => PieceRotation::EAST,
            PieceRotation::EAST => PieceRotation::SOUTH,
            PieceRotation::SOUTH => PieceRotation::WEST,
            PieceRotation::WEST => PieceRotation::NORTH,
        };
    }
    fn left(&mut self) {
        self.current.x -= 1;
    }
    fn right(&mut self) {
        self.current.x += 1;
    }
    fn drop(&mut self) {
        self.current.y += 1;
    }
}
