#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn new_from_index(i: u8) -> Self {
        assert!(i < 8);
        unsafe { std::mem::transmute(i) }
    }
}
