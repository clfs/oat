use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::role::Role;

#[derive(Default)]
pub struct Board {
    white: Bitboard,
    black: Bitboard,
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        Board {
            white: Bitboard(0),
            black: Bitboard(0),
            pawns: Bitboard(0),
            knights: Bitboard(0),
            bishops: Bitboard(0),
            rooks: Bitboard(0),
            queens: Bitboard(0),
            kings: Bitboard(0),
        }
    }

    pub fn white(&self) -> Bitboard {
        self.white
    }

    pub fn black(&self) -> Bitboard {
        self.black
    }

    pub fn by_color(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.white(),
            Color::Black => self.black(),
        }
    }

    pub fn by_role(&self, role: Role) -> Bitboard {
        match role {
            Role::Pawn => self.pawns,
            Role::Knight => self.knights,
            Role::Bishop => self.bishops,
            Role::Rook => self.rooks,
            Role::Queen => self.queens,
            Role::King => self.kings,
        }
    }
}
