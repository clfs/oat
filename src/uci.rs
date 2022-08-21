use std::io::BufRead;
use std::io::Write;

use crate::engine::Engine;

pub fn run<R, W>(reader: &mut R, writer: &mut W, engine: &mut Engine)
where
    R: BufRead,
    W: Write,
{
    for line in reader.lines() {
        engine.noop();
        writeln!(writer, "echo: {}", line.unwrap()).unwrap();
    }
}
