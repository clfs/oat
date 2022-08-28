pub const NUM_COLORS: usize = 2;

#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub fn new_from_index(i: u8) -> Self {
        assert!(i < 2);
        unsafe { std::mem::transmute(i) }
    }
}
