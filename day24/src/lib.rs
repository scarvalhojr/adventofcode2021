use std::collections::HashMap;
use std::str::FromStr;
use Instruction::*;
use Operand::*;
use Variable::*;

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
    Num(i64),
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

type Registers = [i64; 4];
type Bounds = (i64, i64);
type States = HashMap<Registers, Bounds>;

fn operands(regs: &Registers, var: Variable, operand: Operand) -> (i64, i64) {
    let operand1 = regs[var as usize];
    let operand2 = match operand {
        Var(v) => regs[v as usize],
        Num(n) => n,
    };
    (operand1, operand2)
}

impl Instruction {
    fn evaluate(&self, regs: &mut Registers) -> Option<()> {
        let (var, result) = match self {
            Inp(_) => return None,
            Add(var, operand) => {
                let (operand1, operand2) = operands(regs, *var, *operand);
                (*var, operand1 + operand2)
            }
            Mul(var, operand) => {
                let (operand1, operand2) = operands(regs, *var, *operand);
                (*var, operand1 * operand2)
            }
            Div(var, operand) => {
                let (operand1, operand2) = operands(regs, *var, *operand);
                if operand2 == 0 {
                    return None;
                }
                (*var, operand1 / operand2)
            }
            Mod(var, operand) => {
                let (operand1, operand2) = operands(regs, *var, *operand);
                if operand1 < 0 || operand2 <= 0 {
                    return None;
                }
                (*var, operand1 % operand2)
            }
            Eql(var, operand) => {
                let (operand1, operand2) = operands(regs, *var, *operand);
                (*var, if operand1 == operand2 { 1 } else { 0 })
            }
        };

        regs[var as usize] = result;
        Some(())
    }

    fn next_states(&self, mut states: States) -> Option<States> {
        let mut next_states = HashMap::new();

        if let Inp(var) = self {
            for (regs, (min, max)) in states.drain() {
                for input in 1..=9 {
                    let mut next_regs = regs;
                    next_regs[*var as usize] = input;
                    let next_min = min * 10 + input;
                    let next_max = max * 10 + input;
                    next_states
                        .entry(next_regs)
                        .and_modify(|bounds: &mut Bounds| {
                            bounds.0 = bounds.0.min(next_min);
                            bounds.1 = bounds.1.max(next_max);
                        })
                        .or_insert((next_min, next_max));
                }
            }
        } else {
            for (mut regs, (min, max)) in states.drain() {
                self.evaluate(&mut regs)?;
                next_states
                    .entry(regs)
                    .and_modify(|bounds: &mut Bounds| {
                        bounds.0 = bounds.0.min(min);
                        bounds.1 = bounds.1.max(max);
                    })
                    .or_insert((min, max));
            }
        }

        Some(next_states)
    }
}

fn run(
    instructions: &[Instruction],
    mut input: impl Iterator<Item = i64>,
) -> Option<Registers> {
    let mut regs = [0; 4];
    for instruction in instructions {
        if let Inp(var) = instruction {
            regs[*var as usize] = input.next()?;
        } else {
            instruction.evaluate(&mut regs)?;
        }
    }
    Some(regs)
}

pub fn solve(instructions: &[Instruction]) -> Option<(i64, i64)> {
    let mut states = HashMap::from([([0; 4], (0, 0))]);
    for instruction in instructions {
        states = instruction.next_states(states)?;
    }

    let (min, max) = states
        .iter()
        .filter(|(regs, _)| regs[Z as usize] == 0)
        .map(|(_, range)| *range)
        .reduce(|(overal_min, overall_max), (state_min, state_max)| {
            (overal_min.min(state_min), overall_max.max(state_max))
        })?;

    // Make sure the solutions do result in Z being zero
    for mut solution in [min, max] {
        let mut input = Vec::new();
        while solution > 0 {
            input.push(solution % 10);
            solution /= 10;
        }
        if run(instructions, input.into_iter().rev())?[Z as usize] != 0 {
            return None;
        }
    }

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

        s.parse::<i64>()
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
