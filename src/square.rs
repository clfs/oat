use crate::file::File;
use crate::rank::Rank;

#[derive(Debug, PartialEq)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    /// Return a square corresponding to the file and rank.
    pub fn new(file: File, rank: Rank) -> Square {
        Square::new_from_index((file as u8) << 3 | rank as u8)
    }

    /// Return a square by index.
    pub fn new_from_index(i: u8) -> Square {
        assert!(i < 64);
        unsafe { std::mem::transmute(i) }
    }

    pub fn file(self) -> File {
        File::new_from_index(self as u8 & 7)
    }

    pub fn rank(self) -> Rank {
        Rank::new_from_index(self as u8 >> 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::File;
    use crate::rank::Rank;

    #[test]
    fn test_new() {
        assert_eq!(Square::new(File::A, Rank::One), Square::A1);
        assert_eq!(Square::new(File::H, Rank::Eight), Square::H8);
    }

    #[test]
    fn test_new_from_index() {
        assert_eq!(Square::new_from_index(0), Square::A1);
        assert_eq!(Square::new_from_index(63), Square::H8);
    }

    #[test]
    fn test_file() {
        assert_eq!(Square::A1.file(), File::A);
        assert_eq!(Square::H8.file(), File::H);
    }

    #[test]
    fn test_rank() {
        assert_eq!(Square::A1.rank(), Rank::One);
        assert_eq!(Square::H8.rank(), Rank::Eight);
    }
}
