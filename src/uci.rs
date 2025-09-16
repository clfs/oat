use std::fmt;
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Id(Id),
    IsReady,
    ReadyOk,
    Quit,
    Uci,
    UciOk,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(id) => match id {
                Id::Name(s) => write!(f, "id name {s}"),
                Id::Author(s) => write!(f, "id author {s}"),
            },
            Self::IsReady => write!(f, "isready"),
            Self::ReadyOk => write!(f, "readyok"),
            Self::Quit => write!(f, "quit"),
            Self::Uci => write!(f, "uci"),
            Self::UciOk => write!(f, "uciok"),
        }
    }
}

impl str::FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Ensure there is a first line.
        let line = lines.next().ok_or(())?;

        // Ensure there isn't a second line.
        if lines.next().is_some() {
            return Err(());
        }

        let mut fields = line.split_whitespace();

        match fields.next() {
            Some("id") => {
                let b = fields.next();
                let c = fields.next();
                match (b, c) {
                    (Some("name"), Some(name)) => Ok(Self::Id(Id::Name(name.to_owned()))),
                    (Some("author"), Some(author)) => Ok(Self::Id(Id::Author(author.to_owned()))),
                    _ => Err(()),
                }
            }
            Some("isready") => Ok(Self::IsReady),
            Some("readyok") => Ok(Self::ReadyOk),
            Some("uci") => Ok(Self::Uci),
            Some("uciok") => Ok(Self::UciOk),
            Some("quit") => Ok(Self::Quit),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Id {
    Author(String),
    Name(String),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_display_isready() {
        assert_eq!(Command::IsReady.to_string(), "isready");
    }

    #[test]
    fn test_display_id() {
        assert_eq!(
            Command::Id(Id::Name("bingo".to_string())).to_string(),
            "id name bingo"
        );
        assert_eq!(
            Command::Id(Id::Author("bongo".to_string())).to_string(),
            "id author bongo"
        );
    }

    #[test]
    fn test_fromstr_isready() {
        assert_eq!("isready".parse(), Ok(Command::IsReady));
        assert_eq!("isready\n".parse(), Ok(Command::IsReady));
        assert_eq!("isready\r\n".parse(), Ok(Command::IsReady));
    }

    #[test]
    fn test_fromstr_id() {
        assert_eq!(
            "id name foo".parse(),
            Ok(Command::Id(Id::Name("foo".to_string())))
        );
        assert_eq!(
            "id author bar".parse(),
            Ok(Command::Id(Id::Author("bar".to_string())))
        );
    }

    #[test]
    fn test_fromstr_bad() {
        assert_eq!(Command::from_str("whatever"), Err(()));
    }
}
