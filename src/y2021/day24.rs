use z3::{
    ast::{Ast, BV},
    Config, Context, Optimize,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let instructions = input.lines().map(Instruction::from_str).collect::<Vec<_>>();

    // Setup Z3 and variables
    let config = Config::new();
    let ctx = Context::new(&config);
    let optimize = Optimize::new(&ctx);
    const SZ: u32 = 64;
    const INP_SZ: u32 = 8;
    let mut vars = (0..4)
        .map(|_| BV::from_u64(&ctx, 0, SZ))
        .collect::<Vec<_>>();
    let input = (0..14)
        .map(|i| BV::new_const(&ctx, format!("input{}", i), INP_SZ))
        .collect::<Vec<_>>();
    let mut input_index = 0;

    // Run each instruction
    macro_rules! var {
        ($var:expr) => {
            vars[*$var as usize].clone()
        };
    }
    macro_rules! value {
        ($var:expr) => {
            match $var {
                Value::Literal(v) => BV::from_i64(&ctx, *v, SZ),
                Value::Variable(v) => vars[*v as usize].clone(),
            }
        };
    }
    for instr in instructions.iter() {
        match instr {
            Instruction::Input(a) => {
                vars[*a as usize] = input[input_index].zero_ext(SZ - INP_SZ);
                input_index += 1;
            }
            Instruction::Add(a, b) => {
                vars[*a as usize] = var!(a) + value!(b);
            }
            Instruction::Multiply(a, b) => {
                vars[*a as usize] = var!(a) * value!(b);
            }
            Instruction::Divide(a, b) => {
                vars[*a as usize] = var!(a).bvsdiv(&value!(b));
            }
            Instruction::Modulus(a, b) => {
                vars[*a as usize] = var!(a).bvsrem(&value!(b));
            }
            Instruction::Equal(a, b) => {
                vars[*a as usize] = var!(a)
                    ._eq(&value!(b))
                    .ite(&BV::from_u64(&ctx, 1, SZ), &BV::from_u64(&ctx, 0, SZ));
            }
        }
    }

    // Assert input within range
    for inp in input.iter() {
        optimize.assert(&inp.bvule(&BV::from_u64(&ctx, 9, INP_SZ)));
        optimize.assert(&inp.bvuge(&BV::from_u64(&ctx, 1, INP_SZ)));
    }

    // Assert final Z register is zero
    optimize.assert(&vars[Variable::Z as usize]._eq(&BV::from_u64(&ctx, 0, SZ)));

    // Solve for maximum input value
    let mut input_value = BV::from_u64(&ctx, 0, SZ);
    for inp in input.iter() {
        input_value *= BV::from_u64(&ctx, 10, SZ);
        input_value += inp.zero_ext(SZ - INP_SZ);
    }
    optimize.push();
    optimize.maximize(&input_value);
    optimize.check(&[]);
    let model = optimize.get_model().unwrap();
    let mut part1 = 0;
    for inp in input.iter() {
        part1 *= 10;
        part1 += model.eval(inp, false).unwrap().as_i64().unwrap();
    }

    // Solve for minimum input value
    optimize.pop();
    optimize.minimize(&input_value);
    optimize.check(&[]);
    let model = optimize.get_model().unwrap();
    let mut part2 = 0;
    for inp in input.iter() {
        part2 *= 10;
        part2 += model.eval(inp, false).unwrap().as_i64().unwrap();
    }

    (part1, part2)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Input(Variable),
    Add(Variable, Value),
    Multiply(Variable, Value),
    Divide(Variable, Value),
    Modulus(Variable, Value),
    Equal(Variable, Value),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        macro_rules! var {
            ($i:expr) => {
                Variable::from_str(&parts[$i])
            };
        }
        macro_rules! value {
            ($i:expr) => {
                Value::from_str(&parts[$i])
            };
        }
        match parts[0] {
            "inp" => Instruction::Input(var!(1)),
            "add" => Instruction::Add(var!(1), value!(2)),
            "mul" => Instruction::Multiply(var!(1), value!(2)),
            "div" => Instruction::Divide(var!(1), value!(2)),
            "mod" => Instruction::Modulus(var!(1), value!(2)),
            "eql" => Instruction::Equal(var!(1), value!(2)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Variable(Variable),
    Literal(i64),
}

impl Value {
    fn from_str(s: &str) -> Self {
        if let Ok(v) = s.parse::<i64>() {
            Value::Literal(v)
        } else {
            Value::Variable(Variable::from_str(s))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl Variable {
    fn from_str(s: &str) -> Self {
        match s {
            "w" => Variable::W,
            "x" => Variable::X,
            "y" => Variable::Y,
            "z" => Variable::Z,
            _ => unreachable!(),
        }
    }
}
