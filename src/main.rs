use crate::engine::Engine;

pub mod core;
pub mod engine;
pub mod uci;

fn main() {
    let engine = Engine {};
    println!("{}", engine.banner());
}
