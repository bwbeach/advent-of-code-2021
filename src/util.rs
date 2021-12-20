// File: util.rs

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

/// Read the contents of a file as a Vec<String>
pub fn lines_in_file(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();
    lines.collect()
}
