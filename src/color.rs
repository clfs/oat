#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn new_from_index(i: u8) -> Color {
        assert!(i < 2);
        unsafe { std::mem::transmute(i) }
    }
}
