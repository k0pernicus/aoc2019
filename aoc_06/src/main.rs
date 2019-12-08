mod lib;

use lib::graph::parse_file;

fn main() {
    let g = parse_file(include_str!("input.txt"));
    println!("Number of orbits: {}", g.get_nb_orbits());
    println!(
        "Path between YOU and SAN: {}",
        g.mesure_path_between("YOU", "SAN").unwrap()
    );
}
