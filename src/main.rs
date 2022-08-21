use uci::ParseError;

mod engine;
mod uci;

fn main() -> Result<(), ParseError> {
    uci::run(
        &mut std::io::stdin(),
        &mut std::io::stdout(),
        &mut engine::Engine::new(),
    )
}
