mod engine;
mod uci;

fn main() {
    uci::run(
        &mut std::io::stdin().lock(),
        &mut std::io::stdout(),
        &mut engine::Engine::new(),
    );
}
