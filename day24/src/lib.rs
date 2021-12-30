use std::collections::{HashMap, HashSet};
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

type Variables = (Integer, Integer, Integer, Integer);

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
    ExpLiteral(Box<Expr>, Integer),
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
            ExpLiteral(e, n) => write!(f, "({} ^ {})", e, n),
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
            (_, n) if n == 0 => Literal(n),
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
            (e1, e2) if e1 == e2 => ExpLiteral(Box::new(e1), 2),
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
                if e.values().iter().all(|n| *n < literal) {
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
            e => EqlLiteral(Box::new(e), literal),
        }
    }

    fn new_eql_expr(expr1: Expr, expr2: Expr) -> Self {
        match (expr1, expr2) {
            (Literal(n1), Literal(n2)) if n1 == n2 => Literal(1),
            (Literal(_), Literal(_)) => Literal(0),
            (Read(_), Literal(n)) if n < 1 || n > 9 => Literal(0),
            (Literal(n), Read(_)) if n < 1 || n > 9 => Literal(0),
            (e1, e2) => {
                let vals1 = e1.values();
                let vals2 = e2.values();
                if vals1.len() == 1 && vals2.len() == 1 && vals1 == vals2 {
                    Literal(1)
                } else if vals1.is_disjoint(&vals2) {
                    Literal(0)
                } else {
                    EqlExpr(Box::new(e1), Box::new(e2))
                }
            }
        }
    }

    fn values(&self) -> HashSet<Integer> {
        match self {
            Read(_) => HashSet::from_iter(1..=9),
            Literal(n) => HashSet::from([*n]),
            AddLiteral(expr, literal) => expr
                .values()
                .iter()
                .map(|n| *n + literal)
                .collect::<HashSet<_>>(),
            AddExpr(expr1, expr2) => expr1
                .values()
                .iter()
                .flat_map(|n1| {
                    expr2
                        .values()
                        .iter()
                        .map(move |n2| n1 + n2)
                        .collect::<HashSet<_>>()
                })
                .collect::<HashSet<_>>(),
            MulLiteral(expr, literal) => expr
                .values()
                .iter()
                .map(|n| *n * literal)
                .collect::<HashSet<_>>(),
            MulExpr(expr1, expr2) => expr1
                .values()
                .iter()
                .flat_map(|n1| {
                    expr2
                        .values()
                        .iter()
                        .map(move |n2| n1 * n2)
                        .collect::<HashSet<_>>()
                })
                .collect::<HashSet<_>>(),
            DivLiteral(expr, literal) => expr
                .values()
                .iter()
                .map(|n| *n / literal)
                .collect::<HashSet<_>>(),
            ModLiteral(expr, literal) => expr
                .values()
                .iter()
                .map(|n| *n % literal)
                .collect::<HashSet<_>>(),
            EqlLiteral(_, _) => HashSet::from_iter(0..=1),
            EqlExpr(_, _) => HashSet::from_iter(0..=1),
            _ => unimplemented!(),
        }
    }
}

fn reduce(instr: &[Instruction]) {
    let mut read_count = 0;
    let mut expression: HashMap<Variable, Expr> = [W, X, Y, Z]
        .into_iter()
        .map(|var| (var, Literal(0)))
        .collect();

    for instruction in instr.iter().take(150) {
        match instruction {
            Inp(var) => {
                expression.insert(*var, Read(read_count));
                read_count += 1;
            }
            Add(var, Num(n)) => {
                let expr = expression.remove(var).unwrap();
                expression.insert(*var, Expr::new_add_lit(expr, *n));
            }
            Add(var1, Var(var2)) => {
                let expr1 = expression.remove(var1).unwrap();
                let expr2 = expression.get(&var2).unwrap().clone();
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

        // println!("\nVariables after instruction: {:?}", instruction);
        // for var in [W, X, Y, Z] {
        //     let expr = expression.get(&var).unwrap();
        //     println!("{:?}: {} => {:?}", var, expr, expr.values());
        // }
    }

    println!("\nVariables after all instructions:");
    for var in [W, X, Y, Z] {
        println!("{:?}: {}", var, expression.get(&var).unwrap());
    }
}

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
