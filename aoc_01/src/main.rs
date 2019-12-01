use std::process;

mod lib;

use lib::file;
use lib::fuel;

fn main() {
    let r = file::read_file_masses("input");
    if let Err(error) = r {
        println!("Got an error when reading input file: {}", error);
        process::exit(1);
    }
    let v = r.unwrap();
    if v.len() == 0 {
        println!("Input file is empty");
        process::exit(0);
    }
    let s: u32 = v
        .iter()
        .map(|mass: &u32| fuel::get_fuel(*mass) as u32)
        .sum::<u32>();
    println!("1. The sum of masses is {}", s);
    let s: u32 = v
        .into_iter()
        .map(|mass: u32| fuel::get_additional_fuel(mass) as u32)
        .sum::<u32>();
    println!("2. Sum with additional fuel is {}", s);
}
