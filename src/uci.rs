use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use vampirc_uci::{parse_one, UciMessage};

use crate::engine::Engine;

fn id() -> UciMessage {
    UciMessage::Id {
        name: Some("aloe-0.1.1".to_string()),
        author: Some("Calvin Figuereo-Supraner".to_string()),
    }
}

pub fn run<R, W>(reader: &mut R, writer: &mut W, engine: &mut Engine) -> Result<(), Box<dyn Error>>
where
    R: Read,
    W: Write,
{
    let buf_reader = BufReader::new(reader);

    for line in buf_reader.lines() {
        match parse_one(&line?) {
            UciMessage::Uci => {
                writeln!(writer, "{}", id())?;
                writeln!(writer, "{}", UciMessage::UciOk)?;
            }
            UciMessage::UciNewGame => {
                engine.new_game();
            }
            UciMessage::Position {
                startpos,
                fen,
                moves,
            } => {
                if startpos {
                    engine.new_game();
                } else {
                    engine.new_game_from_uci_fen(fen.unwrap());
                }
                for m in moves {
                    engine.make_uci_move(m);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
