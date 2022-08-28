#![allow(dead_code)]
use std::error::Error;

mod bitboard;
mod chess {
    pub mod board;
    pub mod color;
    pub mod file;
    pub mod moves;
    pub mod piece;
    pub mod position;
    pub mod rank;
    pub mod role;
    pub mod square;
}
mod engine;

mod uci;

fn main() -> Result<(), Box<dyn Error>> {
    uci::Adapter::new(
        &mut engine::Engine::new(),
        &mut std::io::stdin().lock(),
        &mut std::io::stdout(),
    )
    .run()
}
