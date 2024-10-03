use std::time::{Duration, Instant};
fn main() {
    let start = Instant::now();

    println!("time elapsed: {:?}", start.elapsed());
}
