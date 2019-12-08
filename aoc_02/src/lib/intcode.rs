use super::opcode::compute;

const STEPS: usize = 4;

struct IntCode {
    tape: Vec<i32>,
    index: usize,
}

impl IntCode {
    fn new(v: Vec<i32>) -> IntCode {
        IntCode { tape: v, index: 0 }
    }

    fn run(&mut self) -> Result<(), String> {
        if self.tape.len() == 0 {
            return Err(String::from("tape is too small"));
        }
        loop {
            let tape_part = &self.tape[self.index..usize::min(self.tape.len(), self.index + STEPS)];
            let opcode = tape_part[0];
            if opcode == 99 {
                break;
            }
            let (left_position, right_position, override_position) =
                (tape_part[1], tape_part[2], tape_part[3]);
            let (left, right) = (
                self.tape[left_position as usize],
                self.tape[right_position as usize],
            );
            match compute(opcode, left, right) {
                Some(result) => {
                    if self.tape.len() < (override_position as usize) {
                        return Err(format!(
                            "overriding value at index {}: size of tape is {}",
                            override_position,
                            self.tape.len(),
                        ));
                    }
                    self.tape[override_position as usize] = result;
                }
                None => break,
            }
            self.index = self.index + STEPS;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestDef {
        tape: Vec<i32>,
        output: Vec<i32>,
    }

    #[test]
    fn run_intcode() {
        let test: TestDef = TestDef {
            tape: vec![
                1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 19, 10, 23, 1,
                23, 13, 27, 1, 6, 27, 31, 1, 9, 31, 35, 2, 10, 35, 39, 1, 39, 6, 43, 1, 6, 43, 47,
                2, 13, 47, 51, 1, 51, 6, 55, 2, 6, 55, 59, 2, 59, 6, 63, 2, 63, 13, 67, 1, 5, 67,
                71, 2, 9, 71, 75, 1, 5, 75, 79, 1, 5, 79, 83, 1, 83, 6, 87, 1, 87, 6, 91, 1, 91, 5,
                95, 2, 10, 95, 99, 1, 5, 99, 103, 1, 10, 103, 107, 1, 107, 9, 111, 2, 111, 10, 115,
                1, 115, 9, 119, 1, 13, 119, 123, 1, 123, 9, 127, 1, 5, 127, 131, 2, 13, 131, 135,
                1, 9, 135, 139, 1, 2, 139, 143, 1, 13, 143, 0, 99, 2, 0, 14, 0,
            ],
            output: vec![],
        };
        let mut machine = IntCode::new(test.tape.clone());
        match machine.run() {
            Ok(_) => println!("Position 0: {}", machine.tape.first().unwrap()),
            Err(err_message) => println!("Got an error running test: {:?}", err_message),
        }
    }

    #[test]
    fn search_for() {
        let test: TestDef = TestDef {
            tape: vec![
                1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 19, 10, 23, 1, 23,
                13, 27, 1, 6, 27, 31, 1, 9, 31, 35, 2, 10, 35, 39, 1, 39, 6, 43, 1, 6, 43, 47, 2,
                13, 47, 51, 1, 51, 6, 55, 2, 6, 55, 59, 2, 59, 6, 63, 2, 63, 13, 67, 1, 5, 67, 71,
                2, 9, 71, 75, 1, 5, 75, 79, 1, 5, 79, 83, 1, 83, 6, 87, 1, 87, 6, 91, 1, 91, 5, 95,
                2, 10, 95, 99, 1, 5, 99, 103, 1, 10, 103, 107, 1, 107, 9, 111, 2, 111, 10, 115, 1,
                115, 9, 119, 1, 13, 119, 123, 1, 123, 9, 127, 1, 5, 127, 131, 2, 13, 131, 135, 1,
                9, 135, 139, 1, 2, 139, 143, 1, 13, 143, 0, 99, 2, 0, 14, 0,
            ],
            output: vec![],
        };
        let mut got = false;
        for noun in 0..100 {
            if got {
                break;
            }
            for verb in 0..100 {
                if got {
                    break;
                }
                println!("{} - {}", noun, verb);
                let mut tape_clone = test.tape.clone();
                tape_clone[1] = noun;
                tape_clone[2] = verb;
                let mut machine = IntCode::new(tape_clone);
                match machine.run() {
                    Ok(_) => {
                        if *machine.tape.first().unwrap() == 19690720 {
                            println!("Found noun {} and verb {}", noun, verb);
                            got = true;
                        }
                    }
                    Err(err_message) => println!("Got an error running test: {:?}", err_message),
                }
            }
        }
    }

    #[test]
    fn test_intcode() {
        let tests: [TestDef; 5] = [
            TestDef {
                tape: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                output: vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            },
            TestDef {
                tape: vec![1, 0, 0, 0, 99],
                output: vec![2, 0, 0, 0, 99],
            },
            TestDef {
                tape: vec![2, 3, 0, 3, 99],
                output: vec![2, 3, 0, 6, 99],
            },
            TestDef {
                tape: vec![2, 4, 4, 5, 99, 0],
                output: vec![2, 4, 4, 5, 99, 9801],
            },
            TestDef {
                tape: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                output: vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            },
        ];
        for (index, t) in tests.into_iter().enumerate() {
            let mut machine = IntCode::new(t.tape.clone());
            match machine.run() {
                Ok(_) => assert_eq!(machine.tape, t.output),
                Err(err_message) => {
                    println!("Got an error running test {}: {:?}", index, err_message)
                }
            }
        }
    }
}
