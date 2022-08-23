use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;

use crate::engine::Engine;

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn run<R, W>(reader: &mut R, writer: &mut W, engine: &mut Engine) -> Result<(), Box<dyn Error>>
where
    R: Read,
    W: Write,
{
    let buf_reader = BufReader::new(reader);

    for line in buf_reader.lines() {
        match line?.parse()? {
            Message::Uci => {
                writeln!(
                    writer,
                    "id name {}-{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                )?;
                writeln!(writer, "id author {}", env!("CARGO_PKG_AUTHORS"))?;
                writeln!(writer, "uciok")?;
            }
            Message::IsReady => {
                writeln!(writer, "readyok")?;
            }
            // TODO: Implement.
            Message::SetOption { .. } => {
                writeln!(writer, "setoption not handled yet")?;
            }
            // TODO: Implement.
            Message::UciNewGame => {
                writeln!(writer, "ucinewgame not handled yet")?;
            }
            // TODO: Implement.
            Message::Position { .. } => {
                writeln!(writer, "position not handled yet")?;
            }
            // TODO: Implement.
            Message::Go { .. } => {
                writeln!(writer, "go not handled yet")?;
            }
            // TODO: Implement.
            Message::Stop => {
                writeln!(writer, "stop not handled yet")?;
            }
            // TODO: Implement.
            Message::PonderHit => {
                writeln!(writer, "ponderhit not handled yet")?;
            }
            Message::Quit => {
                return Ok(());
            }
            Message::Unknown(s) => {
                writeln!(writer, "unknown command: {}", s)?;
            }
        }
        engine.noop();
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimeControl {
    Ponder,
    Explicit {
        white_time_ms: Option<u32>,
        black_time_ms: Option<u32>,
        white_increment_ms: Option<u32>,
        black_increment_ms: Option<u32>,
        moves_to_go: Option<u32>,
    },
    MoveTime {
        time_ms: u32,
    },
    Infinite,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Uci,
    IsReady,
    SetOption {
        name: String,
        value: Option<String>,
    },
    UciNewGame,
    Position {
        fen: String,
        moves: Vec<String>,
    },
    Go {
        time_control: Option<TimeControl>,
        search_moves: Vec<String>,
        mate: Option<u32>,
        depth: Option<u32>,
        nodes: Option<u32>,
    },
    Stop,
    PonderHit,
    Quit,
    Unknown(String),
}

impl FromStr for Message {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let command = words.next().ok_or("no command")?;
        match command {
            "uci" => Ok(Message::Uci),
            "isready" => Ok(Message::IsReady),
            // TODO: Implement the "setoption" command correctly.
            "setoption" => Ok(Message::SetOption {
                name: "foo".to_string(),
                value: None,
            }),
            "ucinewgame" => Ok(Message::UciNewGame),
            // TODO: Implement the "position" command correctly.
            "position" => Ok(Message::Position {
                fen: STARTPOS.to_string(),
                moves: Vec::new(),
            }),
            // TODO: Implement the "go" command correctly.
            "go" => Ok(Message::Go {
                time_control: Some(TimeControl::Infinite),
                search_moves: Vec::new(),
                mate: None,
                depth: None,
                nodes: None,
            }),
            "stop" => Ok(Message::Stop),
            "ponderhit" => Ok(Message::PonderHit),
            "quit" => Ok(Message::Quit),
            _ => Ok(Message::Unknown(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse {
        ($name:ident, $input:expr, $want:expr) => {
            #[test]
            fn $name() {
                match Message::from_str($input) {
                    Ok(got) => assert_eq!(got, $want),
                    Err(err) => panic!("{:?}", err),
                }
            }
        };
    }

    test_parse!(test_parse_uci, "uci", Message::Uci);

    test_parse!(test_parse_isready, "isready", Message::IsReady);

    test_parse!(
        test_parse_setoption,
        "setoption name foo value bar",
        Message::SetOption {
            name: "foo".to_string(),
            value: Some("bar".to_string()),
        }
    );

    test_parse!(
        test_parse_setoption_no_value,
        "setoption name foo",
        Message::SetOption {
            name: "foo".to_string(),
            value: None,
        }
    );

    test_parse!(
        test_parse_setoption_multiword_name,
        "setoption name foo bar value baz",
        Message::SetOption {
            name: "foo bar".to_string(),
            value: Some("baz".to_string()),
        }
    );

    test_parse!(
        test_parse_setoption_multiword_name_no_value,
        "setoption name foo bar",
        Message::SetOption {
            name: "foo bar".to_string(),
            value: None,
        }
    );

    test_parse!(
        test_parse_setoption_multiword_name_and_value,
        "setoption name foo bar value baz qux",
        Message::SetOption {
            name: "foo bar".to_string(),
            value: Some("baz qux".to_string()),
        }
    );

    test_parse!(test_parse_ucinewgame, "ucinewgame", Message::UciNewGame);

    test_parse!(
        test_parse_position_startpos,
        "position startpos",
        Message::Position {
            fen: STARTPOS.to_string(),
            moves: vec![],
        }
    );

    test_parse!(
        test_parse_position_startpos_moves,
        "position startpos moves e2e4 e7e5",
        Message::Position {
            fen: STARTPOS.to_string(),
            moves: vec!["e2e4".to_string(), "e7e5".to_string()],
        }
    );

    test_parse!(
        test_parse_position_fen,
        "position fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        Message::Position {
            fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string(),
            moves: vec![],
        }
    );

    test_parse!(
        test_parse_position_fen_moves,
        "position fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 moves e7e5",
        Message::Position {
            fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string(),
            moves: vec!["e7e5".to_string()],
        }
    );

    test_parse!(
        test_parse_go_ponder,
        "go ponder",
        Message::Go {
            time_control: Some(TimeControl::Ponder),
            search_moves: vec![],
            mate: None,
            depth: None,
            nodes: None,
        }
    );

    test_parse!(
        test_parse_go_complex,
        "go wtime 100 btime 200 winc 300 binc 400 movestogo 5 mate 6 depth 7 nodes 8 searchmoves e2e4 d2d4",
        Message::Go {
            time_control: Some(TimeControl::Explicit {
                white_time_ms: Some(100),
                black_time_ms: Some(200),
                white_increment_ms: Some(300),
                black_increment_ms: Some(400),
                moves_to_go: Some(5),
            }),
            search_moves: vec!["e2e4".to_string(), "d2d4".to_string()],
            mate: Some(6),
            depth: Some(7),
            nodes: Some(8),
        }
    );

    test_parse!(test_parse_stop, "stop", Message::Stop);

    test_parse!(test_parse_ponderhit, "ponderhit", Message::PonderHit);

    test_parse!(test_parse_quit, "quit", Message::Quit);

    test_parse!(
        test_parse_unknown,
        "foo bar",
        Message::Unknown("foo bar".to_string())
    );
}
