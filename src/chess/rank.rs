#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn new_from_index(i: u8) -> Rank {
        assert!(i < 8);
        unsafe { std::mem::transmute(i) }
    }
}
