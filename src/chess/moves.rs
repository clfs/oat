use crate::chess::piece::Piece;
use crate::chess::square::Square;

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
}
