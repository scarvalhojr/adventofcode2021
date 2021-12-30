use std::collections::HashMap;
use std::str::FromStr;
use Instruction::*;
use Operand::*;
use Variable::*;

pub type Integer = i64;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Var(Variable),
    Num(Integer),
}

#[derive(Debug)]
pub enum Instruction {
    Inp(Variable),
    Add(Variable, Operand),
    Mul(Variable, Operand),
    Div(Variable, Operand),
    Mod(Variable, Operand),
    Eql(Variable, Operand),
}

type Registers = [Integer; 4];

fn exec(instr: &[Instruction], input: &[Integer]) -> Option<Registers> {
    let mut regs = [0; 4];
    let mut input_iter = input.iter().copied().rev();

    for instruction in instr {
        match instruction {
            Inp(var) => {
                regs[*var as usize] = input_iter.next()?;
            }
            Add(var, operand) => {
                let (operand1, operand2) = operands(&regs, *var, *operand);
                regs[*var as usize] = operand1 + operand2;
            }
            Mul(var, operand) => {
                let (operand1, operand2) = operands(&regs, *var, *operand);
                regs[*var as usize] = operand1 * operand2;
            }
            Div(var, operand) => {
                let (operand1, operand2) = operands(&regs, *var, *operand);
                if operand2 == 0 {
                    return None;
                }
                regs[*var as usize] = operand1 / operand2;
            }
            Mod(var, operand) => {
                let (operand1, operand2) = operands(&regs, *var, *operand);
                if operand1 < 0 || operand2 <= 0 {
                    return None;
                }
                regs[*var as usize] = operand1 % operand2;
            }
            Eql(var, operand) => {
                let (operand1, operand2) = operands(&regs, *var, *operand);
                let result = if operand1 == operand2 { 1 } else { 0 };
                regs[*var as usize] = result;
            }
        }
    }

    Some(regs)
}

fn operands(
    regs: &Registers,
    var: Variable,
    operand: Operand,
) -> (Integer, Integer) {
    let operand1 = regs[var as usize];
    let operand2 = match operand {
        Var(v) => regs[v as usize],
        Num(n) => n,
    };
    (operand1, operand2)
}

pub fn solve(instructions: &[Instruction]) -> Option<(i64, i64)> {
    let mut states: HashMap<Registers, (i64, i64)> =
        HashMap::from([([0; 4], (0, 0))]);
    for instruction in instructions {
        let mut next_states = HashMap::new();
        for (mut regs, (min, max)) in states.drain() {
            match instruction {
                Inp(var) => {
                    for input in 1..=9 {
                        let mut next_regs = regs;
                        next_regs[*var as usize] = input;
                        let next_min = min * 10 + input;
                        let next_max = max * 10 + input;
                        next_states
                            .entry(next_regs)
                            .and_modify(|range: &mut (i64, i64)| {
                                range.0 = range.0.min(next_min);
                                range.1 = range.1.max(next_max);
                            })
                            .or_insert((next_min, next_max));
                    }
                }
                Add(var, operand) => {
                    let (operand1, operand2) = operands(&regs, *var, *operand);
                    regs[*var as usize] = operand1 + operand2;
                    next_states
                        .entry(regs)
                        .and_modify(|range: &mut (i64, i64)| {
                            range.0 = range.0.min(min);
                            range.1 = range.1.max(max);
                        })
                        .or_insert((min, max));
                }
                Mul(var, operand) => {
                    let (operand1, operand2) = operands(&regs, *var, *operand);
                    regs[*var as usize] = operand1 * operand2;
                    next_states
                        .entry(regs)
                        .and_modify(|range: &mut (i64, i64)| {
                            range.0 = range.0.min(min);
                            range.1 = range.1.max(max);
                        })
                        .or_insert((min, max));
                }
                Div(var, operand) => {
                    let (operand1, operand2) = operands(&regs, *var, *operand);
                    if operand2 == 0 {
                        return None;
                    }
                    regs[*var as usize] = operand1 / operand2;
                    next_states
                        .entry(regs)
                        .and_modify(|range: &mut (i64, i64)| {
                            range.0 = range.0.min(min);
                            range.1 = range.1.max(max);
                        })
                        .or_insert((min, max));
                }
                Mod(var, operand) => {
                    let (operand1, operand2) = operands(&regs, *var, *operand);
                    if operand1 < 0 || operand2 <= 0 {
                        return None;
                    }
                    regs[*var as usize] = operand1 % operand2;
                    next_states
                        .entry(regs)
                        .and_modify(|range: &mut (i64, i64)| {
                            range.0 = range.0.min(min);
                            range.1 = range.1.max(max);
                        })
                        .or_insert((min, max));
                }
                Eql(var, operand) => {
                    let (operand1, operand2) = operands(&regs, *var, *operand);
                    let result = if operand1 == operand2 { 1 } else { 0 };
                    regs[*var as usize] = result;
                    next_states
                        .entry(regs)
                        .and_modify(|range: &mut (i64, i64)| {
                            range.0 = range.0.min(min);
                            range.1 = range.1.max(max);
                        })
                        .or_insert((min, max));
                }
            }
        }

        states = next_states;
    }

    let (min, max) = states
        .iter()
        .filter(|(regs, _)| regs[Z as usize] == 0)
        .map(|(_, range)| *range)
        .reduce(|(overal_min, overall_max), (state_min, state_max)| {
            (overal_min.min(state_min), overall_max.max(state_max))
        })?;
    Some((min, max))
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
