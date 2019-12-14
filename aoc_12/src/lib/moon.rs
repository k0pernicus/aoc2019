use std::char::ParseCharError;
use std::cmp::Ordering;
use std::default::Default;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity { x: 0, y: 0, z: 0 }
    }
}

impl Velocity {
    pub fn compute_kinetic_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Moon {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

fn compare(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

impl Moon {
    pub fn compute_velocity(&self, moons: Vec<Moon>, v: &Velocity) -> Velocity {
        let mut o = Velocity::default();
        o.x = moons.iter().fold(v.x, |sum, s| sum + compare(self.x, s.x));
        o.y = moons.iter().fold(v.y, |sum, s| sum + compare(self.y, s.y));
        o.z = moons.iter().fold(v.z, |sum, s| sum + compare(self.z, s.z));
        o
    }

    pub fn apply_velocity(&self, v: &Velocity) -> Self {
        Moon {
            x: self.x + (*v).x,
            y: self.y + (*v).y,
            z: self.z + (*v).z,
        }
    }

    pub fn compute_potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Default for Moon {
    fn default() -> Moon {
        Moon { x: 0, y: 0, z: 0 }
    }
}

impl FromStr for Moon {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = Moon::default();
        // First, remove the '<' and '>' characters
        let mut sanitized_s = s.trim();
        sanitized_s = &sanitized_s[1..s.len() - 1];
        let coordinates = sanitized_s.split(',');
        for coordinate in coordinates {
            let sanitized_coordinate = coordinate.trim();
            let parsed_coordinates: Vec<&str> = sanitized_coordinate.split('=').collect();
            match parsed_coordinates[0] {
                "x" => m.x = parsed_coordinates[1].parse::<i32>().unwrap(),
                "y" => m.y = parsed_coordinates[1].parse::<i32>().unwrap(),
                "z" => m.z = parsed_coordinates[1].parse::<i32>().unwrap(),
                e => panic!("Received unknown key '{}'", e),
            }
        }
        Ok(m)
    }
}
