use std::{collections::HashMap, collections::VecDeque, str::FromStr};

use crate::Exercise;

pub struct Day18;

impl Exercise for Day18 {
    fn part1(&self, input: &str) -> String {
        let instructions = input
            .lines()
            .map(|line| line.parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();

        let mut registers = HashMap::new();
        let mut pc: i64 = 0;
        let mut last_sound = 0;
        macro_rules! get_value {
            ($data:expr) => {
                match $data {
                    Data::Value(value) => *value,
                    Data::Register(reg) => *registers.entry(reg).or_default(),
                }
            };
        }
        loop {
            if pc < 0 || pc >= instructions.len() as i64 {
                break;
            }

            let instr = &instructions[pc as usize];
            match instr {
                Instruction::Sound(data) => last_sound = get_value!(data),
                Instruction::Set(reg, data) => {
                    let value = get_value!(data);
                    registers.insert(reg, value);
                }
                Instruction::Add(reg, data) => {
                    let value = get_value!(data);
                    *registers.entry(reg).or_default() += value;
                }
                Instruction::Multiply(reg, data) => {
                    let value = get_value!(data);
                    *registers.entry(reg).or_default() *= value;
                }
                Instruction::Modulo(reg, data) => {
                    let value = get_value!(data);
                    *registers.entry(reg).or_default() %= value;
                }
                Instruction::Recover(reg) => {
                    let value = *registers.entry(reg).or_default();
                    if value != 0 {
                        break;
                    }
                }
                Instruction::JumpIfGreaterZero(..) => {}
            }

            match instr {
                Instruction::JumpIfGreaterZero(reg, data) => {
                    let value = get_value!(reg);
                    if value > 0 {
                        let jump = get_value!(data);
                        pc += jump;
                    } else {
                        pc += 1;
                    }
                }
                _ => pc += 1,
            }
        }
        last_sound.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let instructions = input
            .lines()
            .map(|line| line.parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();

        let mut queue_0 = VecDeque::new();
        let mut queue_1 = VecDeque::new();
        let mut machine_0 = RunState::new(0);
        let mut machine_1 = RunState::new(1);
        loop {
            machine_0.step(&instructions, &mut queue_1, &mut queue_0);
            machine_1.step(&instructions, &mut queue_0, &mut queue_1);
            if (machine_0.is_blocked || machine_0.is_finished)
                && (machine_1.is_blocked || machine_1.is_finished)
            {
                break;
            }
        }
        machine_1.send_count.to_string()
    }
}

struct RunState {
    registers: HashMap<char, i64>,
    pc: i64,
    send_count: u32,
    is_blocked: bool,
    is_finished: bool,
}

impl RunState {
    fn new(machine_num: i64) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', machine_num);
        RunState {
            registers,
            pc: 0,
            send_count: 0,
            is_blocked: false,
            is_finished: false,
        }
    }

    fn step(
        &mut self,
        instructions: &[Instruction],
        send_queue: &mut VecDeque<i64>,
        recv_queue: &mut VecDeque<i64>,
    ) {
        macro_rules! get_value {
            ($data:expr) => {
                match $data {
                    Data::Value(value) => *value,
                    Data::Register(reg) => *self.registers.entry(*reg).or_default(),
                }
            };
        }
        if self.is_finished {
            return;
        }
        self.is_blocked = false;
        if self.pc < 0 || self.pc >= instructions.len() as i64 {
            self.is_finished = true;
            return;
        }

        let instr = &instructions[self.pc as usize];
        match instr {
            Instruction::Sound(data) => {
                self.send_count += 1;
                send_queue.push_back(get_value!(data));
                self.pc += 1;
            }
            Instruction::Set(reg, data) => {
                let value = get_value!(data);
                self.registers.insert(*reg, value);
                self.pc += 1;
            }
            Instruction::Add(reg, data) => {
                let value = get_value!(data);
                *self.registers.entry(*reg).or_default() += value;
                self.pc += 1;
            }
            Instruction::Multiply(reg, data) => {
                let value = get_value!(data);
                *self.registers.entry(*reg).or_default() *= value;
                self.pc += 1;
            }
            Instruction::Modulo(reg, data) => {
                let value = get_value!(data);
                *self.registers.entry(*reg).or_default() %= value;
                self.pc += 1;
            }
            Instruction::Recover(reg) => {
                if recv_queue.is_empty() {
                    self.is_blocked = true;
                    return;
                }
                let value = recv_queue.pop_front().unwrap();
                self.registers.insert(*reg, value);
                self.pc += 1;
            }
            Instruction::JumpIfGreaterZero(reg, data) => {
                let value = get_value!(reg);
                if value > 0 {
                    let jump = get_value!(data);
                    self.pc += jump;
                } else {
                    self.pc += 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Sound(Data),
    Set(char, Data),
    Add(char, Data),
    Multiply(char, Data),
    Modulo(char, Data),
    Recover(char),
    JumpIfGreaterZero(Data, Data),
}

#[derive(Debug, Clone)]
enum Data {
    Register(char),
    Value(i64),
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        Ok(match parts[0] {
            "snd" => Instruction::Sound(parts[1].parse()?),
            "set" => Instruction::Set(parts[1].chars().next().unwrap(), parts[2].parse()?),
            "add" => Instruction::Add(parts[1].chars().next().unwrap(), parts[2].parse()?),
            "mul" => Instruction::Multiply(parts[1].chars().next().unwrap(), parts[2].parse()?),
            "mod" => Instruction::Modulo(parts[1].chars().next().unwrap(), parts[2].parse()?),
            "rcv" => Instruction::Recover(parts[1].chars().next().unwrap()),
            "jgz" => Instruction::JumpIfGreaterZero(parts[1].parse()?, parts[2].parse()?),
            _ => panic!(),
        })
    }
}

impl FromStr for Data {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().next().unwrap();
        if (first >= '0' && first <= '9') || first == '-' {
            Ok(Data::Value(s.parse()?))
        } else {
            Ok(Data::Register(first))
        }
    }
}
