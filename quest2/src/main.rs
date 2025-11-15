use std::path::Path;

fn main() {
    println!("Hello, world!");
}

fn read_file(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let result = std::fs::read_to_string(path)?;
    Ok(result)
}

struct InputData {}