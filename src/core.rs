/// The color of a piece, player, or square.
pub enum Color {
    White,
    Black,
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(n: u8) -> Result<Color, Self::Error> {
        match n {
            0 => Ok(Color::White),
            1 => Ok(Color::Black),
            _ => Err(()),
        }
    }
}

/// The role of a piece.
pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl TryFrom<u8> for Role {
    type Error = ();

    fn try_from(n: u8) -> Result<Role, Self::Error> {
        match n {
            0 => Ok(Role::Pawn),
            1 => Ok(Role::Knight),
            2 => Ok(Role::Bishop),
            3 => Ok(Role::Rook),
            4 => Ok(Role::Queen),
            5 => Ok(Role::King),
            _ => Err(()),
        }
    }
}

/// A file on the board.
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

impl TryFrom<u8> for File {
    type Error = ();

    fn try_from(n: u8) -> Result<File, Self::Error> {
        match n {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err(()),
        }
    }
}

/// A rank on the board.
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl TryFrom<u8> for Rank {
    type Error = ();

    fn try_from(n: u8) -> Result<Rank, Self::Error> {
        match n {
            0 => Ok(Rank::First),
            1 => Ok(Rank::Second),
            2 => Ok(Rank::Third),
            3 => Ok(Rank::Fourth),
            4 => Ok(Rank::Fifth),
            5 => Ok(Rank::Sixth),
            6 => Ok(Rank::Seventh),
            7 => Ok(Rank::Eighth),
            _ => Err(()),
        }
    }
}

/// A square on the board.
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    /// Return the square at the file and rank.
    pub fn new(file: File, rank: Rank) -> Square {
        let f = file as u8;
        let r = rank as u8;
        Square::try_from(f << 3 | r).unwrap()
    }

    /// Return the file that the square is on.
    pub fn file(self) -> File {
        let s = self as u8;
        File::try_from(s & 7).unwrap()
    }

    /// Return the rank that the square is on.
    pub fn rank(self) -> Rank {
        let s = self as u8;
        Rank::try_from(s >> 3).unwrap()
    }
}

impl TryFrom<u8> for Square {
    type Error = ();

    fn try_from(n: u8) -> Result<Square, Self::Error> {
        match n {
            0 => Ok(Square::A1),
            1 => Ok(Square::B1),
            2 => Ok(Square::C1),
            3 => Ok(Square::D1),
            4 => Ok(Square::E1),
            5 => Ok(Square::F1),
            6 => Ok(Square::G1),
            7 => Ok(Square::H1),
            8 => Ok(Square::A2),
            9 => Ok(Square::B2),
            10 => Ok(Square::C2),
            11 => Ok(Square::D2),
            12 => Ok(Square::E2),
            13 => Ok(Square::F2),
            14 => Ok(Square::G2),
            15 => Ok(Square::H2),
            16 => Ok(Square::A3),
            17 => Ok(Square::B3),
            18 => Ok(Square::C3),
            19 => Ok(Square::D3),
            20 => Ok(Square::E3),
            21 => Ok(Square::F3),
            22 => Ok(Square::G3),
            23 => Ok(Square::H3),
            24 => Ok(Square::A4),
            25 => Ok(Square::B4),
            26 => Ok(Square::C4),
            27 => Ok(Square::D4),
            28 => Ok(Square::E4),
            29 => Ok(Square::F4),
            30 => Ok(Square::G4),
            31 => Ok(Square::H4),
            32 => Ok(Square::A5),
            33 => Ok(Square::B5),
            34 => Ok(Square::C5),
            35 => Ok(Square::D5),
            36 => Ok(Square::E5),
            37 => Ok(Square::F5),
            38 => Ok(Square::G5),
            39 => Ok(Square::H5),
            40 => Ok(Square::A6),
            41 => Ok(Square::B6),
            42 => Ok(Square::C6),
            43 => Ok(Square::D6),
            44 => Ok(Square::E6),
            45 => Ok(Square::F6),
            46 => Ok(Square::G6),
            47 => Ok(Square::H6),
            48 => Ok(Square::A7),
            49 => Ok(Square::B7),
            50 => Ok(Square::C7),
            51 => Ok(Square::D7),
            52 => Ok(Square::E7),
            53 => Ok(Square::F7),
            54 => Ok(Square::G7),
            55 => Ok(Square::H7),
            56 => Ok(Square::A8),
            57 => Ok(Square::B8),
            58 => Ok(Square::C8),
            59 => Ok(Square::D8),
            60 => Ok(Square::E8),
            61 => Ok(Square::F8),
            62 => Ok(Square::G8),
            63 => Ok(Square::H8),
            _ => Err(()),
        }
    }
}
