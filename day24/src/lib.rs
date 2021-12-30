use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use Expr::*;
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Expr {
    Read(usize),
    Literal(Integer),
    AddLiteral(Box<Expr>, Integer),
    AddExpr(Box<Expr>, Box<Expr>),
    MulLiteral(Box<Expr>, Integer),
    MulExpr(Box<Expr>, Box<Expr>),
    DivLiteral(Box<Expr>, Integer),
    ModLiteral(Box<Expr>, Integer),
    EqlLiteral(Box<Expr>, Integer),
    EqlExpr(Box<Expr>, Box<Expr>),
    Memory(usize, Integer, Integer),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Read(n) => write!(f, "R{}", n),
            Literal(n) => write!(f, "{}", n),
            AddLiteral(e, n) => write!(f, "({} + {})", e, n),
            AddExpr(e1, e2) => write!(f, "({} + {})", e1, e2),
            MulLiteral(e, n) => write!(f, "({} * {})", e, n),
            MulExpr(e1, e2) => write!(f, "({} * {})", e1, e2),
            DivLiteral(e, n) => write!(f, "({} / {})", e, n),
            ModLiteral(e, n) => write!(f, "({} % {})", e, n),
            EqlLiteral(e, n) => write!(f, "({} == {})", e, n),
            EqlExpr(e1, e2) => write!(f, "({} == {})", e1, e2),
            Memory(n, _, _) => write!(f, "M{}", n),
        }
    }
}

impl Expr {
    fn new_add_lit(expr: Expr, literal: Integer) -> Self {
        match (expr, literal) {
            (e, n) if n == 0 => e,
            (Literal(n1), n2) => Literal(n1 + n2),
            (e, n) => AddLiteral(Box::new(e), n),
        }
    }

    fn new_add_expr(expr1: Expr, expr2: Expr) -> Self {
        match (expr1, expr2) {
            (Literal(n), e) if n == 0 => e,
            (e, Literal(n)) if n == 0 => e,
            (Literal(n1), Literal(n2)) => Literal(n1 + n2),
            (e1, e2) => AddExpr(Box::new(e1), Box::new(e2)),
        }
    }

    fn new_mul_lit(expr: Expr, literal: Integer) -> Self {
        match (expr, literal) {
            (_, n) if n == 0 => Literal(0),
            (e, n) if n == 1 => e,
            (Literal(n1), n2) => Literal(n1 * n2),
            (e, n) => MulLiteral(Box::new(e), n),
        }
    }

    fn new_mul_expr(expr1: Expr, expr2: Expr) -> Self {
        match (expr1, expr2) {
            (Literal(n), _) if n == 0 => Literal(0),
            (_, Literal(n)) if n == 0 => Literal(0),
            (Literal(n), e) if n == 1 => e,
            (e, Literal(n)) if n == 1 => e,
            (Literal(n1), Literal(n2)) => Literal(n1 * n2),
            (e1, e2) => MulExpr(Box::new(e1), Box::new(e2)),
        }
    }

    fn new_div_lit(expr: Expr, literal: Integer) -> Self {
        match (expr, literal) {
            (e, n) if n == 1 => e,
            (Literal(n1), n2) => Literal(n1 / n2),
            (e, n) => DivLiteral(Box::new(e), n),
        }
    }

    fn new_mod_lit(expr: Expr, literal: Integer) -> Self {
        match expr {
            Literal(num) => Literal(num % literal),
            e => {
                let (_min, max) = e.range();
                if max < literal {
                    e
                } else {
                    ModLiteral(Box::new(e), literal)
                }
            }
        }
    }

    fn new_eql_lit(expr: Expr, literal: Integer) -> Self {
        match expr {
            Literal(num) if num == literal => Literal(1),
            Literal(_) => Literal(0),
            Read(_) if literal < 1 || literal > 9 => Literal(0),
            e => {
                let (min, max) = e.range();
                if literal == min && literal == max {
                    Literal(1)
                } else if literal < min || literal > max {
                    Literal(0)
                } else {
                    EqlLiteral(Box::new(e), literal)
                }
            }
        }
    }

    fn new_eql_expr(expr1: Expr, expr2: Expr) -> Self {
        match (expr1, expr2) {
            (Literal(n1), Literal(n2)) if n1 == n2 => Literal(1),
            (Literal(_), Literal(_)) => Literal(0),
            (Read(_), Literal(n)) if n < 1 || n > 9 => Literal(0),
            (Literal(n), Read(_)) if n < 1 || n > 9 => Literal(0),
            (e1, e2) => {
                let (min1, max1) = e1.range();
                let (min2, max2) = e2.range();
                if min1 == max1 && max1 == min2 && min2 == max2 {
                    Literal(1)
                } else if max1 < min2 || min1 > max2 {
                    Literal(0)
                } else {
                    EqlExpr(Box::new(e1), Box::new(e2))
                }
            }
        }
    }

    fn range(&self) -> (i64, i64) {
        match self {
            Read(_) => (1, 9),
            Literal(n) => (*n, *n),
            AddLiteral(expr, literal) => {
                let (min, max) = expr.range();
                (min + literal, max + literal)
            }
            AddExpr(expr1, expr2) => {
                let (min1, max1) = expr1.range();
                let (min2, max2) = expr2.range();
                (min1 + min2, max1 + max2)
            }
            MulLiteral(expr, literal) => {
                let (min, max) = expr.range();
                if *literal >= 0 {
                    (min * literal, max * literal)
                } else {
                    (max * literal, min * literal)
                }
            }
            MulExpr(expr1, expr2) => {
                let (min1, max1) = expr1.range();
                let (min2, max2) = expr2.range();
                let min = [min1 * min2, min1 * max2, max1 * min2, max1 * max2]
                    .into_iter()
                    .min()
                    .unwrap();
                let max = [min1 * min2, min1 * max2, max1 * min2, max1 * max2]
                    .into_iter()
                    .max()
                    .unwrap();
                (min, max)
            }
            DivLiteral(expr, literal) => {
                let (min, max) = expr.range();
                if *literal > 0 {
                    (min / literal, max / literal)
                } else {
                    (max / literal, min / literal)
                }
            }
            ModLiteral(expr, literal) => {
                let (min, max) = expr.range();
                if max < *literal {
                    (min, max)
                } else {
                    (0, literal - 1)
                }
            }
            EqlLiteral(_, _) => (0, 1),
            EqlExpr(_, _) => (0, 1),
            Memory(_, min, max) => (*min, *max),
        }
    }
}

fn reduce(instr: &[Instruction]) {
    let mut read_count = 0;
    let mut mem_count = 0;
    let mut expression: HashMap<Variable, Expr> = [W, X, Y, Z]
        .into_iter()
        .map(|var| (var, Literal(0)))
        .collect();

    for (instruction, inst_num) in instr.iter().zip(1..) {
        match instruction {
            Inp(var) => {
                expression.insert(*var, Read(read_count));
                read_count += 1;
                if !matches!(expression.get(&Z), Some(Literal(_))) {
                    let expr = expression.remove(&Z).unwrap();
                    let (min, max) = expr.range();
                    println!("M{}: {} ({}, {})", mem_count, expr, min, max);
                    expression.insert(Z, Memory(mem_count, min, max));
                    mem_count += 1;
                }
            }
            Add(var, Num(n)) => {
                let expr = expression.remove(var).unwrap();
                expression.insert(*var, Expr::new_add_lit(expr, *n));
            }
            Add(var1, Var(var2)) => {
                let expr1 = expression.remove(var1).unwrap();
                let expr2 = expression.get(var2).unwrap().clone();
                expression.insert(*var1, Expr::new_add_expr(expr1, expr2));
            }
            Mul(var1, Var(var2)) => {
                let expr1 = expression.remove(var1).unwrap();
                let expr2 = expression.get(var2).unwrap().clone();
                expression.insert(*var1, Expr::new_mul_expr(expr1, expr2));
            }
            Mul(var, Num(n)) => {
                let expr = expression.remove(var).unwrap();
                expression.insert(*var, Expr::new_mul_lit(expr, *n));
            }
            Div(var, Num(n)) => {
                let expr = expression.remove(var).unwrap();
                expression.insert(*var, Expr::new_div_lit(expr, *n));
            }
            Mod(var, Num(n)) => {
                let expr = expression.remove(var).unwrap();
                expression.insert(*var, Expr::new_mod_lit(expr, *n));
            }
            Eql(var, Num(n)) => {
                let expr = expression.remove(var).unwrap();
                expression.insert(*var, Expr::new_eql_lit(expr, *n));
            }
            Eql(var1, Var(var2)) => {
                let expr1 = expression.remove(var1).unwrap();
                let expr2 = expression.get(var2).unwrap().clone();
                expression.insert(*var1, Expr::new_eql_expr(expr1, expr2));
            }
            _ => unimplemented!(),
        }

        println!("\nInstruction {}: {:?}", inst_num, instruction);
        for var in [W, X, Y, Z] {
            let expr = expression.get(&var).unwrap();
            let (min, max) = expr.range();
            println!("{:?}: {} ({}, {})", var, expr, min, max);
        }
    }

    println!(
        "\nZ after all instructions:\n{}",
        expression.get(&Z).unwrap()
    );
}

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

pub fn solve(instructions: &[Instruction]) -> Option<(i64, i64)> {
    let mut states: HashMap<Registers, (i64, i64)> =
        HashMap::from([([0; 4], (0, 0))]);
    for instruction in instructions {
        println!("{} states", states.len());
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
    println!("Final: {} states", states.len());
    Some((0, 0))
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
        if let Some([_, _, _, z]) = exec(instructions, &input.0) {
            if z == 0 {
                // TODO: return input as a number
                return Some(z);
            }
        }
        input.decrement();
    }
}

pub fn part2(instructions: &[Instruction]) -> Option<i64> {
    reduce(instructions);
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
