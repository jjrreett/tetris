use rand::Rng;
use ratatui::prelude as tui;

#[derive(Clone, Copy)]
pub enum Colors {
    Purple, // T
    Orange, // L
    Pink,   // J
    Red,    // S
    Green,  // Z
    Blue,   // I
    Yellow, // O
}
impl Colors {
    pub fn to_tui_color(self) -> tui::Color {
        match self {
            Colors::Purple => tui::Color::Rgb(155, 89, 182),
            Colors::Orange => tui::Color::Rgb(242, 140, 40),
            Colors::Pink => tui::Color::Rgb(241, 148, 138),
            Colors::Red => tui::Color::Red,
            Colors::Green => tui::Color::Green,
            Colors::Blue => tui::Color::Cyan,
            Colors::Yellow => tui::Color::Yellow,
        }
    }
}

pub enum MoveDirection {
    Down,
    Left,
    Right,
    CCW,
    CW,
    FirmDrop,
}

#[derive(Clone, Copy)]
pub enum TetrominoRotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Clone, Copy)]
pub enum TetrominoShape {
    T,
    L,
    J,
    S,
    Z,
    I,
    O,
}

impl TetrominoShape {
    pub fn pick_random_shape() -> TetrominoShape {
        let shapes = [
            TetrominoShape::O,
            TetrominoShape::I,
            TetrominoShape::T,
            TetrominoShape::S,
            TetrominoShape::Z,
            TetrominoShape::J,
            TetrominoShape::L,
        ];

        let mut rng = rand::thread_rng();
        let random_shape = shapes[rng.gen_range(0..shapes.len())];
        random_shape
    }
}

#[derive(Clone, Copy)]
pub struct Tetromino {
    // y, x
    pub blocks: [(usize, usize); 4],
    pub color: Colors,
    pub shape: TetrominoShape,
    pub rotation: TetrominoRotation,
}
impl Tetromino {
    pub fn new(shape: TetrominoShape, rotation: TetrominoRotation) -> Tetromino {
        type S = TetrominoShape;
        type R = TetrominoRotation;
        type C = Colors;

        match (shape, rotation) {
            (S::T, R::Zero) => Tetromino {
                blocks: [(1, 0), (1, 1), (1, 2), (0, 1)],
                color: C::Purple,
                shape: S::T,
                rotation: R::Zero,
            },
            (S::T, R::Ninety) => Tetromino {
                blocks: [(0, 1), (1, 1), (2, 1), (1, 2)],
                color: C::Purple,
                shape: S::T,
                rotation: R::Ninety,
            },
            (S::T, R::OneEighty) => Tetromino {
                blocks: [(1, 0), (1, 1), (1, 2), (2, 1)],
                color: C::Purple,
                shape: S::T,
                rotation: R::OneEighty,
            },
            (S::T, R::TwoSeventy) => Tetromino {
                blocks: [(0, 1), (1, 1), (2, 1), (1, 0)],
                color: C::Purple,
                shape: S::T,
                rotation: R::TwoSeventy,
            },

            // L
            (S::L, R::Zero) => Tetromino {
                blocks: [(1, 0), (1, 1), (1, 2), (0, 2)],
                color: C::Orange,
                shape: S::L,
                rotation: R::Zero,
            },
            (S::L, R::Ninety) => Tetromino {
                blocks: [(0, 1), (1, 1), (2, 1), (2, 0)],
                color: C::Orange,
                shape: S::L,
                rotation: R::Ninety,
            },
            (S::L, R::OneEighty) => Tetromino {
                blocks: [(1, 0), (1, 1), (1, 2), (2, 0)],
                color: C::Orange,
                shape: S::L,
                rotation: R::OneEighty,
            },
            (S::L, R::TwoSeventy) => Tetromino {
                blocks: [(0, 0), (1, 0), (2, 0), (0, 1)],
                color: C::Orange,
                shape: S::L,
                rotation: R::TwoSeventy,
            },

            // J
            (S::J, R::Zero) => Tetromino {
                blocks: [(1, 0), (1, 1), (1, 2), (0, 0)],
                color: C::Pink,
                shape: S::J,
                rotation: R::Zero,
            },
            (S::J, R::Ninety) => Tetromino {
                blocks: [(0, 1), (1, 1), (2, 1), (2, 2)],
                color: C::Pink,
                shape: S::J,
                rotation: R::Ninety,
            },
            (S::J, R::OneEighty) => Tetromino {
                blocks: [(1, 0), (1, 1), (1, 2), (2, 2)],
                color: C::Pink,
                shape: S::J,
                rotation: R::OneEighty,
            },
            (S::J, R::TwoSeventy) => Tetromino {
                blocks: [(0, 2), (0, 1), (1, 1), (2, 1)],
                color: C::Pink,
                shape: S::J,
                rotation: R::TwoSeventy,
            },

            // S
            (S::S, R::Zero) => Tetromino {
                blocks: [(1, 0), (1, 1), (0, 1), (0, 2)],
                color: C::Green,
                shape: S::S,
                rotation: R::Zero,
            },
            (S::S, R::Ninety) => Tetromino {
                blocks: [(0, 1), (1, 1), (1, 2), (2, 2)],
                color: C::Green,
                shape: S::S,
                rotation: R::Ninety,
            },
            (S::S, R::OneEighty) => Tetromino {
                blocks: [(2, 0), (2, 1), (1, 1), (1, 2)],
                color: C::Green,
                shape: S::S,
                rotation: R::OneEighty,
            },
            (S::S, R::TwoSeventy) => Tetromino {
                blocks: [(0, 0), (1, 0), (1, 1), (2, 1)],
                color: C::Green,
                shape: S::S,
                rotation: R::TwoSeventy,
            },

            // Z - The red one
            (S::Z, R::Zero) => Tetromino {
                blocks: [(0, 0), (0, 1), (1, 1), (1, 2)],
                color: C::Red,
                shape: S::Z,
                rotation: R::Zero,
            },
            (S::Z, R::Ninety) => Tetromino {
                blocks: [(0, 1), (1, 1), (1, 2), (2, 1)],
                color: C::Red,
                shape: S::Z,
                rotation: R::Ninety,
            },
            (S::Z, R::OneEighty) => Tetromino {
                blocks: [(1, 0), (1, 1), (1, 1), (1, 2)],
                color: C::Red,
                shape: S::Z,
                rotation: R::OneEighty,
            },
            (S::Z, R::TwoSeventy) => Tetromino {
                blocks: [(0, 0), (1, 0), (1, 1), (2, 1)],
                color: C::Red,
                shape: S::Z,
                rotation: R::TwoSeventy,
            },

            // I
            (S::I, R::Zero) => Tetromino {
                blocks: [(1, 0), (1, 1), (1, 2), (1, 3)],
                color: C::Blue,
                shape: S::I,
                rotation: R::Zero,
            },
            (S::I, R::Ninety) => Tetromino {
                blocks: [(0, 2), (1, 2), (2, 2), (3, 2)],
                color: C::Blue,
                shape: S::I,
                rotation: R::Ninety,
            },
            (S::I, R::OneEighty) => Tetromino {
                blocks: [(2, 0), (2, 1), (2, 2), (2, 3)],
                color: C::Blue,
                shape: S::I,
                rotation: R::OneEighty,
            },
            (S::I, R::TwoSeventy) => Tetromino {
                blocks: [(0, 1), (1, 1), (2, 1), (3, 1)],
                color: C::Blue,
                shape: S::I,
                rotation: R::TwoSeventy,
            },

            // O is all the same, thank god
            (S::O, rotation) => Tetromino {
                blocks: [(0, 0), (0, 1), (1, 0), (1, 1)],
                color: C::Yellow,
                shape: S::O,
                rotation: rotation,
            },
        }
    }

    pub fn rotate(self, direction: MoveDirection) -> Tetromino {
        match (direction, self.rotation) {
            (MoveDirection::CW, TetrominoRotation::Zero) => {
                Tetromino::new(self.shape, TetrominoRotation::Ninety)
            }
            (MoveDirection::CW, TetrominoRotation::Ninety) => {
                Tetromino::new(self.shape, TetrominoRotation::OneEighty)
            }
            (MoveDirection::CW, TetrominoRotation::OneEighty) => {
                Tetromino::new(self.shape, TetrominoRotation::TwoSeventy)
            }
            (MoveDirection::CW, TetrominoRotation::TwoSeventy) => {
                Tetromino::new(self.shape, TetrominoRotation::Zero)
            }
            (MoveDirection::CCW, TetrominoRotation::Zero) => {
                Tetromino::new(self.shape, TetrominoRotation::TwoSeventy)
            }
            (MoveDirection::CCW, TetrominoRotation::Ninety) => {
                Tetromino::new(self.shape, TetrominoRotation::Zero)
            }
            (MoveDirection::CCW, TetrominoRotation::OneEighty) => {
                Tetromino::new(self.shape, TetrominoRotation::Ninety)
            }
            (MoveDirection::CCW, TetrominoRotation::TwoSeventy) => {
                Tetromino::new(self.shape, TetrominoRotation::OneEighty)
            }
            (_, _) => self,
        }
    }
}
