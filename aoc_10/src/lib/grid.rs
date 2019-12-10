use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::iter::FromIterator;

pub type Coordinate = (usize, usize);

pub struct Grid {
    content: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            content: Vec::new(),
        }
    }

    pub fn add_line(&mut self, s: &str) {
        let mut line = Vec::new();
        for c in s.chars() {
            let c = match c {
                '#' => true,
                _ => false,
            };
            line.push(c);
        }
        self.content.push(line);
    }

    fn find_asteroids(&self) -> Vec<Coordinate> {
        let mut coordinates: Vec<Coordinate> = Vec::new();
        for (y, line) in self.content.iter().enumerate() {
            for (x, elt) in line.iter().enumerate() {
                if *elt {
                    coordinates.push((x, y));
                }
            }
        }
        coordinates
    }

    pub fn find_monitoring_station(&self) -> (Coordinate, u32) {
        let asteroids_coordinates = self.find_asteroids();
        let detection: Vec<(Coordinate, u32)> = asteroids_coordinates
            .par_iter()
            .map(|x| detect_asteroids(x, &asteroids_coordinates))
            .collect();
        *detection.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap()
    }

    pub fn vaporize(&self) {
        let asteroids_coordinates = self.find_asteroids();
        let (coordinates, _) = self.find_monitoring_station();
        let mut index = 1;
        loop {
            let (remaining_asteroids, c_index) =
                vaporize(&coordinates, &asteroids_coordinates, index);
            index = c_index;
            if remaining_asteroids.len() == 0 {
                break;
            }
        }
    }
}

fn detect_asteroids(c: &Coordinate, a: &Vec<Coordinate>) -> (Coordinate, u32) {
    let mut tree: BTreeMap<OrderedFloat<f64>, bool> = BTreeMap::new();
    for oc in a.iter() {
        if oc == c {
            continue;
        }
        let dy: f64 = (oc.1 as i64 - c.1 as i64) as f64;
        let dx: f64 = (oc.0 as i64 - c.0 as i64) as f64;
        let tan = OrderedFloat::from(dy.atan2(dx).to_degrees());
        if tree.contains_key(&tan) {
            continue;
        }
        tree.insert(tan, true);
    }
    (*c, tree.keys().len() as u32)
}

fn vaporize(c: &Coordinate, a: &Vec<Coordinate>, old_index: usize) -> (Vec<Coordinate>, usize) {
    let mut c_index = old_index;
    let mut tree: BTreeMap<OrderedFloat<f64>, &Coordinate> = BTreeMap::new();
    let mut remaining_coordinates: Vec<Coordinate> = Vec::new();
    for oc in a.iter() {
        if oc == c {
            continue;
        }
        let dy: f64 = (oc.1 as i64 - c.1 as i64) as f64;
        let dx: f64 = (oc.0 as i64 - c.0 as i64) as f64;
        let tan = OrderedFloat::from(dy.atan2(dx).to_degrees() + 180.);
        // println!("Coordinates {:?}: tan {}", oc, tan);
        tree.insert(tan, oc);
    }
    let mut v = Vec::from_iter(tree);
    v.sort_by(|&(a, _), &(b, _)| b.cmp(&a));
    v.reverse();
    println!("{:?}", v);
    // Find position with first positive value
    let zero_degree_indice = v
        .iter()
        .position(|x| x.0 >= OrderedFloat::from(90f64))
        .unwrap();
    for c in zero_degree_indice..v.len() {
        println!("Asteroid {}: {:?}", c_index, v[c]);
        c_index += 1;
    }
    for c in 0..zero_degree_indice {
        println!("Asteroid {}: {:?}", c_index, v[c]);
        c_index += 1;
    }
    (remaining_coordinates, c_index)
}
