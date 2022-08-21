use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use crate::engine::Engine;

pub fn run<R, W>(reader: &mut R, writer: &mut W, engine: &mut Engine)
where
    R: Read,
    W: Write,
{
    for line in BufReader::new(reader).lines() {
        engine.noop();
        writeln!(writer, "echo: {}", line.unwrap()).unwrap();
    }
}
