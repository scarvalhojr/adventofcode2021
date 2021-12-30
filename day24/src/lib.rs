use std::str::FromStr;
use Instruction::*;
use Operand::*;
use Variable::*;

pub type Integer = i64;

#[derive(Clone, Copy)]
pub enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, Copy)]
pub enum Operand {
    Var(Variable),
    Num(Integer),
}

pub enum Instruction {
    Inp(Variable),
    Add(Variable, Operand),
    Mul(Variable, Operand),
    Div(Variable, Operand),
    Mod(Variable, Operand),
    Eql(Variable, Operand),
}

type Variables = (Integer, Integer, Integer, Integer);

fn exec(instr: &[Instruction], input: &[Integer]) -> Result<Variables, usize> {
    let mut vars = (0, 0, 0, 0);
    let mut input_iter = input.iter().copied().rev();
    let mut input_counter = 0;

    for instruction in instr {
        match instruction {
            Inp(var) => {
                let number = input_iter.next().ok_or(input_counter)?;
                input_counter += 1;
                store(&mut vars, *var, number);
            }
            Add(var, operand) => {
                let (operand1, operand2) = operands(&vars, *var, *operand);
                store(&mut vars, *var, operand1 + operand2);
            }
            Mul(var, operand) => {
                let (operand1, operand2) = operands(&vars, *var, *operand);
                store(&mut vars, *var, operand1 * operand2);
            }
            Div(var, operand) => {
                let (operand1, operand2) = operands(&vars, *var, *operand);
                if operand2 == 0 {
                    return Err(input_counter);
                }
                store(&mut vars, *var, operand1 / operand2);
            }
            Mod(var, operand) => {
                let (operand1, operand2) = operands(&vars, *var, *operand);
                if operand1 < 0 || operand2 <= 0 {
                    return Err(input_counter);
                }
                store(&mut vars, *var, operand1 % operand2);
            }
            Eql(var, operand) => {
                let (operand1, operand2) = operands(&vars, *var, *operand);
                let result = if operand1 == operand2 { 1 } else { 0 };
                store(&mut vars, *var, result);
            }
        }
    }

    Ok(vars)
}

fn operands(
    vars: &Variables,
    var: Variable,
    operand: Operand,
) -> (Integer, Integer) {
    let operand1 = fetch(vars, var);
    let operand2 = match operand {
        Var(v) => fetch(vars, v),
        Num(n) => n,
    };
    (operand1, operand2)
}

fn fetch(vars: &Variables, var: Variable) -> Integer {
    match var {
        W => vars.0,
        X => vars.1,
        Y => vars.2,
        Z => vars.3,
    }
}

fn store(vars: &mut Variables, var: Variable, num: Integer) {
    match var {
        W => vars.0 = num,
        X => vars.1 = num,
        Y => vars.2 = num,
        Z => vars.3 = num,
    }
}

#[derive(Debug)]
struct Input(Vec<i64>);

impl Input {
    fn new() -> Self {
        Self(vec![9; 14])
    }

    fn decrement(&mut self) {
        for digit in self.0.iter_mut().rev() {
            if *digit > 1 {
                *digit -= 1;
                break;
            }
            *digit = 9;
        }
    }
}

pub fn part1(instructions: &[Instruction]) -> Option<i64> {
    let mut input = Input::new();
    let mut report = 0;
    loop {
        if report == 1_000_000 {
            println!("Trying {:?}", input);
            report = 0;
        }
        report += 1;
        match exec(instructions, &input.0) {
            Ok((_w, _x, _y, z)) => {
                // println!("OK: w = {}, x = {}, y = {}, z = {}", w, x, y, z);
                if z == 0 {
                    return Some(z);
                }
            }
            Err(counter) => {
                println!("ERR: {} input numbers read", counter);
            }
        };
        input.decrement();
    }
}

pub fn part2(_instructions: &[Instruction]) -> Option<i32> {
    None
}

impl FromStr for Variable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "w" => Ok(W),
            "x" => Ok(X),
            "y" => Ok(Y),
            "z" => Ok(Z),
            _ => Err(format!("Invalid variable '{}'", s)),
        }
    }
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(var) = s.parse::<Variable>() {
            return Ok(Var(var));
        }

        s.parse::<Integer>()
            .map_err(|_| format!("Invalid operand '{}'", s))
            .map(Num)
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace().rev().collect::<Vec<_>>();
        if tokens.len() < 2 || tokens.len() > 3 {
            return Err(format!("Invalid instruction '{}'", s));
        }

        let instruction = tokens.pop().unwrap().to_lowercase();
        let variable = tokens.pop().unwrap().parse::<Variable>()?;

        if instruction == "inp" {
            return Ok(Inp(variable));
        }

        let operand = tokens
            .pop()
            .ok_or_else(|| format!("Missing operand for '{}'", instruction))?
            .parse::<Operand>()?;

        match instruction.as_str() {
            "add" => Ok(Add(variable, operand)),
            "mul" => Ok(Mul(variable, operand)),
            "div" => Ok(Div(variable, operand)),
            "mod" => Ok(Mod(variable, operand)),
            "eql" => Ok(Eql(variable, operand)),
            _ => Err(format!("Unknown instruction '{}'", instruction)),
        }
    }
}
