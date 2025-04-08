use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now(); // Capture the starting time

    // Simulate some work
    for _ in 0..1_000_000 {
        let _dummy = 2 * 2;
    }

    let duration = start.elapsed(); // Measure the elapsed time
    println!("Time taken: {:?}", duration);
}
