use std::env;
use std::env::current_exe;
use std::io::BufRead;
use std::path::{Path, PathBuf};

// Get the data path that a specific binary should use, along with whether it's running with PROD
// env set or not (cargo run --bin day01-1 vs PROD=1 cargo run --bin day01-1)
pub fn get_data_path() -> PathBuf {
    // exe_path will look like ~/aoc_2023/target/debug/day01-1
    let exe_path = current_exe().expect("Failed to get current executable path");

    // Extract the binary name, e.g. day01-1
    let binary_name = exe_path
        .file_stem()
        .expect("Failed to get the binary name")
        .to_str()
        .expect("Failed to convert OsStr to String");

    // Check the PROD environment variable
    let prod = env::var("PROD").unwrap_or_else(|_| "0".to_string());

    // Construct the path based on the binary name and the environment variable
    let folder = if prod == "1" { "prod" } else { "train" };
    let file_name = format!("{}.txt", binary_name);

    Path::new(folder).join(file_name)
}

// Similar to Python open(fname).readlines()
pub fn read_lines(path: &Path) -> Vec<String> {
    let file = std::fs::File::open(path).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}
