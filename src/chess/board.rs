use crate::bitboard::Bitboard;
use crate::chess::color::Color;
use crate::chess::color::NUM_COLORS;
use crate::chess::piece::Piece;
use crate::chess::role::Role;
use crate::chess::role::NUM_ROLES;

#[derive(Default)]
pub struct Board {
    colors: [Bitboard; NUM_COLORS],
    roles: [Bitboard; NUM_ROLES],
}

impl Board {
    pub fn new() -> Self {
        Board {
            colors: [Bitboard(0); NUM_COLORS],
            roles: [Bitboard(0); NUM_ROLES],
        }
    }

    pub fn white(&self) -> Bitboard {
        self.colors[Color::White as usize]
    }

    pub fn black(&self) -> Bitboard {
        self.colors[Color::Black as usize]
    }

    pub fn by_color(&self, color: Color) -> Bitboard {
        self.colors[color as usize]
    }

    pub fn pawns(&self) -> Bitboard {
        self.roles[Role::Pawn as usize]
    }

    pub fn knights(&self) -> Bitboard {
        self.roles[Role::Knight as usize]
    }

    pub fn bishops(&self) -> Bitboard {
        self.roles[Role::Bishop as usize]
    }

    pub fn rooks(&self) -> Bitboard {
        self.roles[Role::Rook as usize]
    }

    pub fn queens(&self) -> Bitboard {
        self.roles[Role::Queen as usize]
    }

    pub fn kings(&self) -> Bitboard {
        self.roles[Role::King as usize]
    }

    pub fn by_role(&self, role: Role) -> Bitboard {
        self.roles[role as usize]
    }

    pub fn by_piece(&self, piece: Piece) -> Bitboard {
        self.by_color(piece.color) & self.by_role(piece.role)
    }

    pub fn occupied(&self) -> Bitboard {
        self.white() | self.black()
    }
}
