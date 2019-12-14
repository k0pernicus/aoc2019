use super::moon::{Moon, Velocity};
use std::default::Default;
use std::str::FromStr;

pub struct System {
    pub moons: Vec<Moon>,
    pub velocities: Vec<Velocity>,
}

impl Default for System {
    fn default() -> System {
        System {
            moons: Vec::new(),
            velocities: Vec::new(),
        }
    }
}

impl System {
    pub fn add_moon(&mut self, s: &str) {
        self.moons.push(Moon::from_str(s).unwrap());
        self.velocities.push(Velocity::default());
    }

    pub fn compute_step(&mut self) -> Vec<Moon> {
        let clones: Vec<Moon> = self.moons.clone();
        let mut velocities: Vec<Velocity> = Vec::with_capacity(clones.len());
        for (i, m) in self.moons.iter_mut().enumerate() {
            let mut o: Vec<Moon> = Vec::with_capacity(clones.len() - 1);
            for x in 0..i {
                o.push(clones[x].clone());
            }
            for x in i + 1..clones.len() {
                o.push(clones[x].clone());
            }
            let c_v = m.compute_velocity(o, &self.velocities[i]);
            self.velocities[i] = c_v.clone();
            velocities.push(c_v);
        }
        // Once velocities have been computed, add velocities...
        let z = self.moons.iter().zip(velocities.iter());
        z.map(|(m, v)| (*m).apply_velocity(v))
            .collect::<Vec<Moon>>()
    }
}
