mod p01;
mod p02;
mod p03;
mod p04;
mod p05;

fn main() {
    // this is my main file of the project
    // when `cargo run` is should :
    // -- run all problems sequentially
    // -- compute answer
    // -- run respective tests
    println!("Answers");
    println!("Day 01: {}", p01::solve());
    println!("Day 02: {}", p02::solve());
    println!("Day 03: {}", p03::solve());
    println!("Day 04: {}", p04::solve());
    println!("Day 05: {}", p05::solve());
}
