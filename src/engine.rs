use crate::chess::position::Position;

pub struct Engine {
    position: Position,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            position: Position::default(),
        }
    }
}
