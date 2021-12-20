use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::str::FromStr;
use SnailfishNumber::*;
use StackElement::*;

#[derive(Clone)]
pub enum SnailfishNumber {
    Regular(u8),
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self, other).reduce()
    }
}

impl SnailfishNumber {
    fn new(left: SnailfishNumber, right: SnailfishNumber) -> Self {
        Pair(Box::new(left), Box::new(right))
    }

    fn reduce(self) -> Self {
        let mut number = self;
        loop {
            let (new_number, exploded, _, _) = number.explode(0);
            number = new_number;
            if !exploded {
                let (new_number, split) = number.split();
                number = new_number;
                if !split {
                    return number;
                }
            }
        }
    }

    fn explode(self, level: u8) -> (Self, bool, Option<u8>, Option<u8>) {
        assert!(level <= 4);
        match self {
            Regular(number) => (Regular(number), false, None, None),
            Pair(left, right) if level == 4 => match (*left, *right) {
                (Regular(left_number), Regular(right_number)) => {
                    (Regular(0), true, Some(left_number), Some(right_number))
                }
                _ => panic!("Pair found nested inside more than four pairs"),
            },
            Pair(left, right) => {
                let (new_left, exploded, left_remain, right_remain) =
                    left.explode(level + 1);
                if let Some(number) = right_remain {
                    return (
                        Self::new(new_left, right.add_leftmost(number)),
                        exploded,
                        left_remain,
                        None,
                    );
                } else if exploded {
                    return (
                        Self::new(new_left, *right),
                        exploded,
                        left_remain,
                        None,
                    );
                }

                let (new_right, exploded, left_remain, right_remain) =
                    right.explode(level + 1);
                if let Some(number) = left_remain {
                    (
                        Self::new(new_left.add_rightmost(number), new_right),
                        exploded,
                        None,
                        right_remain,
                    )
                } else {
                    (
                        Self::new(new_left, new_right),
                        exploded,
                        None,
                        right_remain,
                    )
                }
            }
        }
    }

    fn add_leftmost(self, number: u8) -> Self {
        match self {
            Regular(n) => Regular(n + number),
            Pair(left, right) => Self::new(left.add_leftmost(number), *right),
        }
    }

    fn add_rightmost(self, number: u8) -> Self {
        match self {
            Regular(n) => Regular(n + number),
            Pair(left, right) => Self::new(*left, right.add_rightmost(number)),
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            Regular(number) if number >= 10 => {
                let left = number / 2;
                let right = number - left;
                (Self::new(Regular(left), Regular(right)), true)
            }
            Regular(number) => (Regular(number), false),
            Pair(left, right) => {
                let (new_left, split) = left.split();
                if split {
                    (Self::new(new_left, *right), split)
                } else {
                    let (new_right, split) = right.split();
                    (Self::new(new_left, new_right), split)
                }
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Regular(num) => *num as u32,
            Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

pub fn part1(numbers: &[SnailfishNumber]) -> Option<u32> {
    numbers
        .iter()
        .cloned()
        .reduce(|result, number| result + number)
        .map(|result| result.magnitude())
}

pub fn part2(numbers: &[SnailfishNumber]) -> Option<u32> {
    numbers
        .iter()
        .enumerate()
        .flat_map(|(idx1, num1)| {
            numbers
                .iter()
                .cloned()
                .enumerate()
                .filter(move |(idx2, _)| idx1 != *idx2)
                .map(move |(_, num2)| (num1.clone() + num2).magnitude())
        })
        .max()
}

enum StackElement {
    OpenBracket,
    Comma,
    Number(SnailfishNumber),
}

impl FromStr for SnailfishNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for (ch, pos) in s.trim().chars().zip(1..) {
            match ch {
                '[' => stack.push(OpenBracket),
                ',' => stack.push(Comma),
                d if d.is_digit(10) => {
                    let number = u8::try_from(d.to_digit(10).unwrap()).unwrap();
                    stack.push(Number(Regular(number)));
                }
                ']' => {
                    match (stack.pop(), stack.pop(), stack.pop(), stack.pop()) {
                        (
                            Some(Number(r)),
                            Some(Comma),
                            Some(Number(l)),
                            Some(OpenBracket),
                        ) => {
                            stack.push(Number(SnailfishNumber::new(l, r)));
                        }
                        _ => return Err("Invalid snailfish number".to_string()),
                    }
                }
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at position {}",
                        ch, pos
                    ))
                }
            }
        }

        match (stack.pop(), stack.pop()) {
            (Some(Number(number)), None) => Ok(number),
            _ => Err("Invalid snailfish number".to_string()),
        }
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Regular(num) => write!(f, "{}", num),
            Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}
