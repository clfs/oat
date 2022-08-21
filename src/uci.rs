use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use crate::engine::Engine;

pub fn run<R, W>(reader: &mut R, writer: &mut W, engine: &mut Engine) -> Result<(), ParseError>
where
    R: Read,
    W: Write,
{
    for line in BufReader::new(reader).lines() {
        match parse(line.unwrap()) {
            Ok(message) => match message {
                Message::IsReady => {
                    writeln!(writer, "readyok").unwrap();
                }
                Message::Quit => {
                    return Ok(());
                }
                _ => {
                    writeln!(writer, "not yet handled!").unwrap();
                }
            },
            Err(err) => {
                return Err(err);
            }
        }
        engine.noop();
    }
    return Ok(());
}

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

pub enum Message {
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

#[derive(Debug)]
pub struct ParseError {
    pub line: String,
}

pub fn parse(line: String) -> Result<Message, ParseError> {
    Err(ParseError { line })
}
