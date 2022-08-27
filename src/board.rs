use crate::bitboard::Bitboard;

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
}
