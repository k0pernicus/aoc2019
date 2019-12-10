use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod lib;
use lib::grid;

fn main() {
    let mut g = grid::Grid::new();
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        if l.trim().len() == 0 {
            continue;
        }
        g.add_line(l.trim());
    }
    let monitoring_station: (grid::Coordinate, u32) = g.find_monitoring_station();
    println!(
        "Found monitoring station at coordinates {:?}: {} asteroids",
        monitoring_station.0, monitoring_station.1
    );
}
