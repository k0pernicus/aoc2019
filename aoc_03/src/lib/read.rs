use super::coordinates::Instruction;
use std::str::FromStr;

pub fn get_instructions(wire: &str) -> Vec<Instruction> {
    wire.split(",")
        .map(|s: &str| Instruction::from_str(s).unwrap())
        .collect::<Vec<Instruction>>()
}
