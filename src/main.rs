use std::error::Error;

mod engine;
mod uci;

fn main() -> Result<(), Box<dyn Error>> {
    uci::run(
        &mut std::io::stdin(),
        &mut std::io::stdout(),
        &mut engine::Engine::new(),
    )
}
