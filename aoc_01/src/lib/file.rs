use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

/// Read from file all masses, and cast each line as u32 value.
/// The file is given as function parameter.
/// This function returns a Result type: Vec<u32> which contains handled lines, or an io::Error.
pub fn read_file_masses(filepath: &str) -> io::Result<Vec<u32>> {
    let f = File::open(filepath)?;
    let f = BufReader::new(f);

    let mut v: Vec<u32> = Vec::new();

    for (i, line) in f.lines().enumerate() {
        match line {
            Ok(raw_line) => {
                if let Ok(mass) = raw_line.parse::<u32>() {
                    v.push(mass);
                }
            }
            Err(err) => println!("Got error when reading line {}: {:?}", i, err),
        }
    }

    Ok(v)
}
