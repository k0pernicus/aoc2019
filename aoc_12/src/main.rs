use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod lib;
use lib::system;

fn main() {
    let mut s = system::System::default();
    let f = File::open("input.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        if l.trim().len() == 0 {
            continue;
        }
        s.add_moon(&l);
    }
    let initial_moons = s.moons.clone();
    let initial_velocities = s.velocities.clone();
    // Part 1
    for _ in 0..1000 {
        let m = s.compute_step();
        s.moons = m;
    }
    println!(
        "final sum: {}",
        s.moons.iter().enumerate().fold(0i32, |sum, (i, m)| {
            sum + (m.compute_potential_energy() * s.velocities[i].compute_kinetic_energy())
        })
    );
    // Part 2
    s.moons = initial_moons.clone();
    s.velocities = initial_velocities;
    // Look for each frequency, for X, Y, and Z, independently
    let mut frequencies: Vec<usize> = vec![0, 0, 0];
    let mut step = 2;
    let initial_x = initial_moons
        .clone()
        .iter()
        .fold(String::from(""), |sum, m| {
            String::from(sum) + &m.x.to_string()
        });
    let initial_y = initial_moons
        .clone()
        .iter()
        .fold(String::from(""), |sum, m| {
            String::from(sum) + &m.y.to_string()
        });
    let initial_z = initial_moons
        .clone()
        .iter()
        .fold(String::from(""), |sum, m| {
            String::from(sum) + &m.z.to_string()
        });
    println!("x: {}, y: {}, z: {}", initial_x, initial_y, initial_z);
    loop {
        let m = s.compute_step();
        s.moons = m.clone();
        let x_sum: String = m.iter().fold(String::from(""), |sum, m| {
            String::from(sum) + &m.x.to_string()
        });
        if frequencies[0] == 0 && x_sum == initial_x {
            frequencies[0] = step;
        }
        let y_sum: String = m.iter().fold(String::from(""), |sum, m| {
            String::from(sum) + &m.y.to_string()
        });
        if frequencies[1] == 0 && y_sum == initial_y {
            frequencies[1] = step;
        }
        let z_sum: String = m.iter().fold(String::from(""), |sum, m| {
            String::from(sum) + &m.z.to_string()
        });
        if frequencies[2] == 0 && z_sum == initial_z {
            frequencies[2] = step;
        }
        if frequencies[0] != 0 && frequencies[1] != 0 && frequencies[2] != 0 {
            break;
        }
        step += 1;
    }

    let max_f = frequencies.iter().max().unwrap();
    let mut mcm_f: usize = max_f.clone();

    loop {
        if mcm_f % frequencies[0] != 0 || mcm_f % frequencies[1] != 0 || mcm_f % frequencies[2] != 0
        {
            mcm_f += max_f;
            continue;
        }
        break;
    }

    println!("Found frequencies {:?}", mcm_f);
}
