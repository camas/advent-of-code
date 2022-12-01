#[derive(Debug, Clone)]
pub struct Machine {
    pub memory: Vec<i64>,
    pub ip: usize,
    pub relative_base: i64,
}

impl Machine {
    pub fn from_str(data: &str) -> Self {
        let memory: Vec<i64> = data.trim().split(',').map(|s| s.parse().unwrap()).collect();
        Machine {
            memory,
            ip: 0,
            relative_base: 0,
        }
    }

    pub fn run(&mut self, mut handler: Option<&mut dyn Handler>) {
        loop {
            // self.print_debug_info();

            let val = self.memory[self.ip];
            let instr = Instruction::from_i64(val);

            // Macros to reduce boilerplate for accessing memory
            macro_rules! read_arg {
                ($i:expr) => {{
                    let mode = match $i {
                        0 => instr.mode0(),
                        1 => instr.mode1(),
                        2 => instr.mode2(),
                        _ => unreachable!(),
                    };
                    extend_mem!(self.ip + $i + 1);
                    let val = self.memory[self.ip + $i + 1];
                    match mode {
                        Mode::Position => {
                            extend_mem!(val as usize);
                            self.memory[val as usize]
                        }
                        Mode::Immediate => val,
                        Mode::Relative => {
                            extend_mem!((self.relative_base + val) as usize);
                            self.memory[(self.relative_base + val) as usize]
                        }
                    }
                }};
            }

            macro_rules! write_arg {
                ($i:expr, $value:expr) => {{
                    let mode = match $i {
                        0 => instr.mode0(),
                        1 => instr.mode1(),
                        2 => instr.mode2(),
                        _ => unreachable!(),
                    };
                    match mode {
                        Mode::Position => {
                            let addr = self.memory[self.ip + $i + 1] as usize;
                            let value = $value;
                            extend_mem!(addr);
                            self.memory[addr] = value;
                        }
                        Mode::Immediate => unreachable!(),
                        Mode::Relative => {
                            let addr =
                                (self.relative_base + self.memory[self.ip + $i + 1]) as usize;
                            let value = $value;
                            extend_mem!(addr);
                            self.memory[addr] = value;
                        }
                    }
                }};
            }

            macro_rules! extend_mem {
                ($i:expr) => {{
                    let i = $i;
                    assert!(i < 1_000_000);
                    if i >= self.memory.len() {
                        self.memory.resize($i + 1, 0);
                    }
                }};
            }

            match instr.op {
                Op::Add => {
                    write_arg!(2, read_arg!(0) + read_arg!(1));
                    self.ip += 4;
                }
                Op::Multiply => {
                    write_arg!(2, read_arg!(0) * read_arg!(1));
                    self.ip += 4;
                }
                Op::Input => {
                    let handler = handler.as_mut().expect("Input opcode without handler");
                    write_arg!(0, handler.input(self));
                    self.ip += 2;
                }
                Op::Output => {
                    let handler = handler.as_mut().expect("Output opcode without handler");
                    let value = read_arg!(0);
                    handler.output(self, value);
                    self.ip += 2;
                }
                Op::JumpIfTrue => {
                    if read_arg!(0) != 0 {
                        self.ip = read_arg!(1) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                Op::JumpIfFalse => {
                    if read_arg!(0) == 0 {
                        self.ip = read_arg!(1) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                Op::LessThan => {
                    if read_arg!(0) < read_arg!(1) {
                        write_arg!(2, 1);
                    } else {
                        write_arg!(2, 0);
                    }
                    self.ip += 4;
                }
                Op::Equals => {
                    if read_arg!(0) == read_arg!(1) {
                        write_arg!(2, 1);
                    } else {
                        write_arg!(2, 0);
                    }
                    self.ip += 4;
                }
                Op::AdjustRelativeBase => {
                    self.relative_base += read_arg!(0);
                    self.ip += 2;
                }
                Op::End => {
                    break;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn print_debug_info(&self) {
        println!("ip: {}", self.ip);
        if self.ip >= self.memory.len() {
            println!("Error: ip in uninitialized memory");
        } else {
            println!("{:?}", &self.memory[self.ip..(self.ip + 10)]);
            let instruction = Instruction::from_i64(self.memory[self.ip]);
            let mut parts = vec![format!("{:?}", instruction.op)];
            for (i, t) in instruction.op.args().iter().enumerate() {
                let mode = match i {
                    0 => instruction.mode0(),
                    1 => instruction.mode1(),
                    2 => instruction.mode2(),
                    _ => unreachable!(),
                };
                match (mode, t) {
                    (Mode::Position, _) | (_, ArgType::Write) => parts.push(format!(
                        "[{}]={}",
                        self.memory[self.ip + i + 1],
                        self.memory[self.memory[self.ip + i + 1] as usize]
                    )),
                    (Mode::Immediate, _) => parts.push(format!("{}", self.memory[self.ip + i + 1])),
                    (Mode::Relative, _) => parts.push(format!(
                        "[{}+{}]={}",
                        self.relative_base,
                        self.memory[self.ip + i + 1],
                        self.memory[(self.relative_base + self.memory[self.ip + i + 1]) as usize]
                    )),
                }
            }
            println!("{}", parts.join(" "));
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    op: Op,
    mode_flags: i64,
}

impl Instruction {
    fn from_i64(value: i64) -> Self {
        Instruction {
            op: Op::from_i64(value % 100),
            mode_flags: value / 100,
        }
    }

    fn mode0(&self) -> Mode {
        Mode::from_i64(self.mode_flags % 10)
    }

    fn mode1(&self) -> Mode {
        Mode::from_i64((self.mode_flags / 10) % 10)
    }

    fn mode2(&self) -> Mode {
        Mode::from_i64((self.mode_flags / 100) % 10)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    End,
    AdjustRelativeBase,
}

impl Op {
    pub fn from_i64(opcode: i64) -> Self {
        match opcode {
            1 => Op::Add,
            2 => Op::Multiply,
            3 => Op::Input,
            4 => Op::Output,
            5 => Op::JumpIfTrue,
            6 => Op::JumpIfFalse,
            7 => Op::LessThan,
            8 => Op::Equals,
            9 => Op::AdjustRelativeBase,
            99 => Op::End,
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }

    fn args(&self) -> &'static [ArgType] {
        match self {
            Op::Add => &[ArgType::Read, ArgType::Read, ArgType::Write],
            Op::Multiply => &[ArgType::Read, ArgType::Read, ArgType::Write],
            Op::Input => &[ArgType::Write],
            Op::Output => &[ArgType::Read],
            Op::JumpIfTrue => &[ArgType::Read, ArgType::Read],
            Op::JumpIfFalse => &[ArgType::Read, ArgType::Read],
            Op::LessThan => &[ArgType::Read, ArgType::Read, ArgType::Write],
            Op::Equals => &[ArgType::Read, ArgType::Read, ArgType::Write],
            Op::AdjustRelativeBase => &[ArgType::Read],
            Op::End => &[],
        }
    }
}

pub trait Handler {
    fn input(&mut self, machine: &Machine) -> i64;
    fn output(&mut self, machine: &Machine, value: i64);
}

enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn from_i64(mode: i64) -> Self {
        match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown mode: {}", mode),
        }
    }
}

enum ArgType {
    Read,
    Write,
}
