#![allow(dead_code)]
use std::error::Error;

mod bitboard;
mod board;
mod color;
mod engine;
mod file;
mod moves;
mod piece;
mod position;
mod rank;
mod role;
mod square;
mod uci;

fn main() -> Result<(), Box<dyn Error>> {
    uci::Adapter::new(
        &mut engine::Engine::new(),
        &mut std::io::stdin().lock(),
        &mut std::io::stdout(),
    )
    .run()
}
