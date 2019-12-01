/// Returns the fuel volume based on a mass parameter.
/// Formula: floor(module / 3) - 2
pub fn get_fuel(mass: u32) -> i32 {
    let module_div: i32 = (mass as f64 / 3.0) as i32;
    i32::max(module_div - 2, 0)
}

pub fn get_additional_fuel(mut mass: u32) -> u32 {
    let mut sum_fuel = 0;
    while mass > 0 {
        let fuel: u32 = get_fuel(mass) as u32;
        sum_fuel += fuel;
        mass = fuel;
    }
    sum_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDef<T> {
        mass: u32,
        ground_truth: T,
    }

    #[test]
    fn test_get_fuel() {
        let tests: [TestDef<i32>; 4] = [
            TestDef {
                mass: 12,
                ground_truth: 2,
            },
            TestDef {
                mass: 14,
                ground_truth: 2,
            },
            TestDef {
                mass: 1969,
                ground_truth: 654,
            },
            TestDef {
                mass: 100756,
                ground_truth: 33583,
            },
        ];
        for t in tests.iter() {
            assert_eq!(get_fuel(t.mass), t.ground_truth);
        }
    }

    #[test]
    fn test_get_additional_fuel() {
        let tests: [TestDef<u32>; 3] = [
            TestDef {
                mass: 14,
                ground_truth: 2,
            },
            TestDef {
                mass: 1969,
                ground_truth: 966,
            },
            TestDef {
                mass: 100756,
                ground_truth: 50346,
            },
        ];
        for t in tests.iter() {
            assert_eq!(get_additional_fuel(t.mass), t.ground_truth);
        }
    }
}
