use vampirc_uci::{UciFen, UciMove};

pub struct Engine {}

impl Engine {
    pub fn new() -> Engine {
        Engine {}
    }

    pub fn new_game(&self) {}

    pub fn new_game_from_uci_fen(&self, fen: UciFen) {}

    pub fn make_uci_move(&self, move_uci: UciMove) {}
}
