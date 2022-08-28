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

pub struct Adapter<'a, R: BufRead, W: Write> {
    engine: &'a mut Engine,
    reader: &'a mut R,
    writer: &'a mut W,
}

impl<R: BufRead, W: Write> Adapter<'_, R, W> {
    pub fn new<'a>(
        engine: &'a mut Engine,
        reader: &'a mut R,
        writer: &'a mut W,
    ) -> Adapter<'a, R, W> {
        Adapter {
            engine,
            reader,
            writer,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        for line in self.reader.lines() {
            match parse_one(&line?) {
                UciMessage::Uci => {
                    writeln!(self.writer, "{}", id())?;
                    writeln!(self.writer, "{}", UciMessage::UciOk)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
