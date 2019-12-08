type Range = (String, String);

trait Incr {
    fn incr(&mut self);
}

pub fn generate_passwords(range: Range) -> Vec<String> {
    let mut valid_passwords = Vec::new();
    let range_0 = range.0.parse::<u32>().unwrap();
    let range_1 = range.1.parse::<u32>().unwrap();
    for i in range_0..range_1 {
        let c_range = i.to_string();
        if valid_password_part2(c_range.clone()) {
            valid_passwords.push(c_range);
        }
    }
    valid_passwords
}

fn valid_password_part1(password: String) -> bool {
    let password_iter = password.chars();
    let mut found_duplicate = false;
    let mut previous_c: char = '0';
    let mut max: char = '0';
    for (i, c) in password_iter.enumerate() {
        if i == 0 {
            previous_c = c;
            max = c;
            continue;
        }
        if c == previous_c {
            found_duplicate = true;
            previous_c = c;
            continue;
        }
        previous_c = c;
        if c < max {
            return false;
        } else {
            max = c;
        }
    }
    found_duplicate
}

fn valid_password_part2(password: String) -> bool {
    let password_iter = password.chars();
    let mut adjacent_digits: u32 = 1;
    let mut previous_c = '0';
    let mut found_duplicate = false;
    let mut max: char = '0';
    for (i, c) in password_iter.enumerate() {
        if i == 0 {
            previous_c = c;
            max = c;
            continue;
        }
        if c == previous_c {
            adjacent_digits += 1;
            continue;
        }
        if adjacent_digits == 2 {
            found_duplicate = true;
        }
        adjacent_digits = 1;
        previous_c = c;
        if c < max {
            return false;
        } else {
            max = c;
        }
    }
    if adjacent_digits == 2 {
        found_duplicate = true;
    }
    found_duplicate
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_password_part1() {
        struct TestS {
            input: String,
            output: bool,
        }
        let tests: [TestS; 3] = [
            TestS {
                input: String::from("111111"),
                output: true,
            },
            TestS {
                input: String::from("223450"),
                output: false,
            },
            TestS {
                input: String::from("123789"),
                output: false,
            },
        ];
        for t in tests.iter() {
            assert_eq!(valid_password_part1(t.input.clone()), t.output);
        }
    }

    #[test]
    fn test_valid_password_part2() {
        struct TestS {
            input: String,
            output: bool,
        }
        let tests: [TestS; 3] = [
            TestS {
                input: String::from("112233"),
                output: true,
            },
            TestS {
                input: String::from("123444"),
                output: false,
            },
            TestS {
                input: String::from("111122"),
                output: true,
            },
        ];
        for t in tests.iter() {
            println!("{}", t.input.clone());
            assert_eq!(valid_password_part2(t.input.clone()), t.output);
        }
    }

    #[test]
    fn test_generate_password() {
        struct TestS {
            input: Range,
            output: u32,
        }
        let tests: [TestS; 2] = [
            TestS {
                input: (String::from("111111"), String::from("111112")),
                output: 1,
            },
            TestS {
                input: (String::from("123451"), String::from("123454")),
                output: 0,
            },
        ];
        for t in tests.iter() {
            let generated_passwords = generate_passwords(t.input.clone()).len();
            assert_eq!(generated_passwords as u32, t.output);
        }
    }

    #[test]
    fn test_part_1() {
        struct TestS {
            input: Range,
        }
        let test = TestS {
            input: (String::from("353096"), String::from("843212")),
        };
        println!(
            "Number of passwords generated: {}",
            generate_passwords(test.input).len()
        );
    }

    #[test]
    fn test_part_2() {
        struct TestS {
            input: Range,
        }
        let test = TestS {
            input: (String::from("353096"), String::from("843212")),
        };
        println!(
            "Number of passwords generated: {}",
            generate_passwords(test.input).len()
        );
    }
}
