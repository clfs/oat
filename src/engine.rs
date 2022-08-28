use crate::position::Position;

pub struct Engine {
    position: Position,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            position: Position::default(),
        }
    }

    pub fn new_game(&self) {}
}
