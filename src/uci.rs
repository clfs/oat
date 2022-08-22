use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use crate::engine::Engine;

const STARTPOS: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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

pub fn parse(line: String) -> Message {
    let mut words = line.split_whitespace();
    let command = words.next().unwrap();
    match command {
        "uci" => Message::Uci,
        "isready" => Message::IsReady,
        "ucinewgame" => Message::UciNewGame,
        "stop" => Message::Stop,
        "ponderhit" => Message::PonderHit,
        "quit" => Message::Quit,
        _ => Message::Unknown(line.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse {
        ($name:ident, $input:expr, $want:expr) => {
            #[test]
            fn $name() {
                let line = $input.to_string();
                let want = $want;
                let got = parse(line);
                assert_eq!(want, got);
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
