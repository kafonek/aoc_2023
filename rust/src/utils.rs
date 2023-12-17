use std::path::Path;
use std::time::Instant;

// Helper function to time how long it takes to run a function and print out some information
// - Input file path
// - Output answer
// - Time elapsed
pub fn run_and_time(run_fn: fn(&Path) -> String, path: &Path) {
    println!("Reading data from: {:?}", &path);
    let start = Instant::now();
    let answer = run_fn(path);
    let elapsed = start.elapsed();
    println!("Answer: {}", answer);
    println!("Time: {:?}", elapsed);
}
