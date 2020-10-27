use std::{collections::HashMap, str::FromStr};

use crate::Exercise;

pub struct Day7;

impl Exercise for Day7 {
    fn part1(&self, input: &str) -> String {
        let circuit = input.parse::<Circuit>().unwrap();
        let mut dest_map = HashMap::new();
        for instruction in circuit.instructions.iter() {
            let dest = match instruction {
                Instruction::Set { dest, .. }
                | Instruction::Not { dest, .. }
                | Instruction::And { dest, .. }
                | Instruction::Or { dest, .. }
                | Instruction::LShift { dest, .. }
                | Instruction::RShift { dest, .. } => dest,
            };
            dest_map.insert(dest, instruction);
        }

        let mut calculated_values = HashMap::<String, u16>::new();
        Circuit::calc_source(
            &Source::Wire("a".to_string()),
            &dest_map,
            &mut calculated_values,
        )
        .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let circuit = input.parse::<Circuit>().unwrap();
        let mut dest_map = HashMap::new();
        for instruction in circuit.instructions.iter() {
            let dest = match instruction {
                Instruction::Set { dest, .. }
                | Instruction::Not { dest, .. }
                | Instruction::And { dest, .. }
                | Instruction::Or { dest, .. }
                | Instruction::LShift { dest, .. }
                | Instruction::RShift { dest, .. } => dest,
            };
            dest_map.insert(dest, instruction);
        }

        let mut calculated_values = HashMap::<String, u16>::new();
        let a_value = Circuit::calc_source(
            &Source::Wire("a".to_string()),
            &dest_map,
            &mut calculated_values,
        );

        let mut calculated_values = HashMap::<String, u16>::new();
        calculated_values.insert("b".to_string(), a_value);
        Circuit::calc_source(
            &Source::Wire("a".to_string()),
            &dest_map,
            &mut calculated_values,
        )
        .to_string()
    }
}

struct Circuit {
    instructions: Vec<Instruction>,
}

impl Circuit {
    fn calc_source(
        source: &Source,
        dest_map: &HashMap<&String, &Instruction>,
        calculated_values: &mut HashMap<String, u16>,
    ) -> u16 {
        match source {
            Source::Value(value) => *value,
            Source::Wire(wire) => {
                if calculated_values.contains_key(wire) {
                    calculated_values[wire]
                } else {
                    let instr = dest_map[wire];
                    let value = match instr {
                        Instruction::Set { source, .. } => {
                            Self::calc_source(source, dest_map, calculated_values)
                        }
                        Instruction::Not { source, .. } => {
                            !Self::calc_source(source, dest_map, calculated_values)
                        }
                        Instruction::And {
                            source_a, source_b, ..
                        } => {
                            Self::calc_source(source_a, dest_map, calculated_values)
                                & Self::calc_source(source_b, dest_map, calculated_values)
                        }
                        Instruction::Or {
                            source_a, source_b, ..
                        } => {
                            Self::calc_source(source_a, dest_map, calculated_values)
                                | Self::calc_source(source_b, dest_map, calculated_values)
                        }
                        Instruction::LShift { source, shift, .. } => {
                            Self::calc_source(source, dest_map, calculated_values) << shift
                        }
                        Instruction::RShift { source, shift, .. } => {
                            Self::calc_source(source, dest_map, calculated_values) >> shift
                        }
                    };
                    calculated_values.insert(wire.to_string(), value);
                    value
                }
            }
        }
    }
}

impl FromStr for Circuit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|line| {
                let parts = line.split(' ').collect::<Vec<_>>();
                if parts[1] == "->" {
                    let source = parts[0].parse().unwrap();
                    let dest = parts[2].to_string();
                    Instruction::Set { source, dest }
                } else if parts[0] == "NOT" {
                    let source = parts[1].parse().unwrap();
                    let dest = parts[3].to_string();
                    Instruction::Not { source, dest }
                } else if parts[1] == "AND" {
                    let source_a = parts[0].parse().unwrap();
                    let source_b = parts[2].parse().unwrap();
                    let dest = parts[4].to_string();
                    Instruction::And {
                        source_a,
                        source_b,
                        dest,
                    }
                } else if parts[1] == "OR" {
                    let source_a = parts[0].parse().unwrap();
                    let source_b = parts[2].parse().unwrap();
                    let dest = parts[4].to_string();
                    Instruction::Or {
                        source_a,
                        source_b,
                        dest,
                    }
                } else if parts[1] == "LSHIFT" {
                    let source = parts[0].parse().unwrap();
                    let shift = parts[2].parse::<u16>().unwrap();
                    let dest = parts[4].to_string();
                    Instruction::LShift {
                        source,
                        shift,
                        dest,
                    }
                } else if parts[1] == "RSHIFT" {
                    let source = parts[0].parse().unwrap();
                    let shift = parts[2].parse::<u16>().unwrap();
                    let dest = parts[4].to_string();
                    Instruction::RShift {
                        source,
                        shift,
                        dest,
                    }
                } else {
                    unreachable!()
                }
            })
            .collect::<Vec<Instruction>>();
        Ok(Self { instructions })
    }
}

enum Instruction {
    Set {
        source: Source,
        dest: String,
    },
    Not {
        source: Source,
        dest: String,
    },
    And {
        source_a: Source,
        source_b: Source,
        dest: String,
    },
    Or {
        source_a: Source,
        source_b: Source,
        dest: String,
    },
    LShift {
        source: Source,
        shift: u16,
        dest: String,
    },
    RShift {
        source: Source,
        shift: u16,
        dest: String,
    },
}

enum Source {
    Value(u16),
    Wire(String),
}

impl FromStr for Source {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().next().unwrap();
        Ok(if first >= '0' && first <= '9' {
            Self::Value(s.parse().unwrap())
        } else {
            Self::Wire(s.to_string())
        })
    }
}
