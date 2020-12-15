use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    let mut memory = HashMap::new();
    let mut mask = &Mask::default();
    for instruction in instructions.iter() {
        match instruction {
            Instruction::SetMask(value) => mask = value,
            Instruction::SetMemory { address, value } => {
                memory.insert(address, mask.apply(*value));
            }
        }
    }
    let part1 = memory.values().sum::<u64>();

    let mut memory = HashMap::new();
    let mut mask = &Mask::default();
    for instruction in instructions.iter() {
        match instruction {
            Instruction::SetMask(value) => mask = value,
            Instruction::SetMemory { address, value } => {
                for address in mask.apply2(*address) {
                    memory.insert(address, *value);
                }
            }
        }
    }
    let part2 = memory.values().sum::<u64>();

    (part1, part2)
}

#[derive(Debug, Default)]
struct Mask {
    zero_mask: u64,
    one_mask: u64,
    floating_offsets: Vec<usize>,
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        value & self.zero_mask | self.one_mask
    }

    fn apply2(&self, value: u64) -> Vec<u64> {
        let value = value | self.one_mask;
        let mut results = Vec::new();
        for a in 0..(2_u64.pow(self.floating_offsets.len() as u32)) {
            let mut result = value;
            for i in 0..self.floating_offsets.len() {
                if a & (1 << i) > 0 {
                    result |= 1 << (35 - self.floating_offsets[i]);
                } else {
                    result &= !(1 << (35 - self.floating_offsets[i]));
                }
            }
            results.push(result);
        }
        results
    }
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut zero_mask = u64::MAX;
        let mut one_mask = 0;
        let mut floating_offsets = Vec::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '1' => {
                    one_mask ^= 1 << (35 - i);
                }
                '0' => {
                    zero_mask ^= 1 << (35 - i);
                }
                'X' => {
                    floating_offsets.push(i);
                }
                _ => unreachable!(),
            }
        }
        Ok(Self {
            zero_mask,
            one_mask,
            floating_offsets,
        })
    }
}

#[derive(Debug)]
enum Instruction {
    SetMask(Mask),
    SetMemory { address: u64, value: u64 },
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[..4] == "mask" {
            let mask_str = s.split(" = ").nth(1).unwrap();
            let mask = mask_str.parse::<Mask>().unwrap();
            Ok(Instruction::SetMask(mask))
        } else {
            let mut parts = s.split("] = ");
            let address = parts.next().unwrap()[4..].parse::<u64>().unwrap();
            let value = parts.next().unwrap().parse::<u64>().unwrap();
            Ok(Instruction::SetMemory { address, value })
        }
    }
}
