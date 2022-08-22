use std::error::Error;
use std::fmt;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use crate::engine::Engine;

pub fn run<R, W>(reader: &mut R, writer: &mut W, engine: &mut Engine) -> Result<(), Box<dyn Error>>
where
    R: Read,
    W: Write,
{
    let buf_reader = BufReader::new(reader);

    for line in buf_reader.lines() {
        match parse(line?) {
            Message::Unknown(_) => {
                // Do nothing.
            }
            Message::Uci => {
                writeln!(writer, "id name {}", env!("CARGO_PKG_NAME"))?;
                writeln!(writer, "id author {}", env!("CARGO_PKG_AUTHORS"))?;
                writeln!(writer, "uciok")?;
            }
            Message::IsReady => {
                writeln!(writer, "readyok")?;
            }
            Message::Quit => {
                return Ok(());
            }
            _ => {
                writeln!(writer, "not handled yet")?;
            }
        }
        engine.noop();
    }

    return Ok(());
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Message {
    Unknown(String),
    Uci,
    Debug(bool),
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
}

pub fn parse(line: String) -> Message {
    Message::Unknown(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uci() {
        let line = "uci".to_string();
        assert_eq!(parse(line), Message::Uci);
    }

    #[test]
    fn test_parse_debug_on() {
        let line = "debug on".to_string();
        assert_eq!(parse(line), Message::Debug(true));
    }

    #[test]
    fn test_parse_debug_off() {
        let line = "debug off".to_string();
        assert_eq!(parse(line), Message::Debug(false));
    }
}
