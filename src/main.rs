use crate::engine::Engine;

pub mod core;
pub mod engine;

fn main() {
    let engine = Engine {};
    println!("{}", engine.banner());
}
