use std::error::Error;

mod bitboard;
mod board;
mod color;
mod engine;
mod file;
mod piece;
mod position;
mod rank;
mod role;
mod square;
mod uci;

fn main() -> Result<(), Box<dyn Error>> {
    uci::run(
        &mut std::io::stdin(),
        &mut std::io::stdout(),
        &mut engine::Engine::new(),
    )
}
