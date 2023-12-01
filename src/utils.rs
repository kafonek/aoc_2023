use std::env;
use std::env::current_exe;
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub fn get_data_path() -> PathBuf {
    // Get the current executable path
    let exe_path = current_exe().expect("Failed to get current executable path");

    // Extract the binary name
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

pub fn read_lines(path: &Path) -> Vec<String> {
    let file = std::fs::File::open(path).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}
