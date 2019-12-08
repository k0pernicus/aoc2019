use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    direction: Direction,
    pub distance: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iterator = s.chars();
        let direction = iterator.next().unwrap();
        let mut i = Instruction {
            direction: Direction::UP,
            distance: 0,
        };
        // Match the direction
        match direction {
            'r' | 'R' => i.direction = Direction::RIGHT,
            'l' | 'L' => i.direction = Direction::LEFT,
            'u' | 'U' => i.direction = Direction::UP,
            'd' | 'D' => i.direction = Direction::DOWN,
            _ => return Err(String::from("Direction not found")),
        }
        // Get the distance
        let distance: String = iterator.collect();
        i.distance = distance
            .parse::<i32>()
            .expect("Canno't parse distance to integer value");
        Ok(i)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }

    pub fn manhattan_distance(&self, other: Coordinate) -> u32 {
        (i32::abs(other.x - self.x) + i32::abs(other.y - self.y)) as u32
    }

    pub fn move_one(&mut self, instruction: &Instruction) -> (i32, i32) {
        if instruction.distance == 0 {
            return (0, 0);
        }
        match instruction.direction {
            Direction::UP => {
                self.y += 1;
                (0, 1)
            }
            Direction::DOWN => {
                self.y -= 1;
                (0, -1)
            }
            Direction::LEFT => {
                self.x -= 1;
                (-1, 0)
            }
            Direction::RIGHT => {
                self.x += 1;
                (1, 0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_move() {
        let mut c = Coordinate::new();
        let mut i = Instruction::from_str("U45").unwrap();
        c.move_one(&i);
        assert_eq!(c.x, 1000);
        assert_eq!(c.y, 1001);
        c = Coordinate::new();
        i = Instruction::from_str("R45").unwrap();
        c.move_one(&i);
        assert_eq!(c.x, 1001);
        assert_eq!(c.y, 1000);
        c = Coordinate::new();
        i = Instruction::from_str("D45").unwrap();
        c.move_one(&i);
        assert_eq!(c.x, 1000);
        assert_eq!(c.y, 999);
        c = Coordinate::new();
        i = Instruction::from_str("L45").unwrap();
        c.move_one(&i);
        assert_eq!(c.x, 999);
        assert_eq!(c.y, 1000);
    }
}
