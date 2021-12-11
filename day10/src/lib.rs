use std::convert::TryFrom;
use std::str::FromStr;
use Bracket::*;

#[derive(Clone, Copy)]
pub enum Bracket {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    AngleOpen,
    AngleClose,
}

impl Bracket {
    fn is_close(&self) -> bool {
        matches!(self, RoundClose | SquareClose | CurlyClose | AngleClose)
    }

    fn matches(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (RoundOpen, RoundClose)
                | (SquareOpen, SquareClose)
                | (CurlyOpen, CurlyClose)
                | (AngleOpen, AngleClose)
        )
    }
}

pub struct Line(Vec<Bracket>);

impl Line {
    fn score_corrupt(&self) -> Option<u64> {
        let mut stack: Vec<Bracket> = Vec::new();

        for &bracket in self.0.iter() {
            if bracket.is_close() {
                if !stack.pop().map(|b| b.matches(&bracket)).unwrap_or(false) {
                    return match bracket {
                        RoundClose => Some(3),
                        SquareClose => Some(57),
                        CurlyClose => Some(1197),
                        AngleClose => Some(25137),
                        _ => unreachable!(),
                    };
                }
            } else {
                stack.push(bracket);
            }
        }

        None
    }

    fn score_incomplete(&self) -> Option<u64> {
        let mut stack: Vec<Bracket> = Vec::new();

        for &bracket in self.0.iter() {
            if bracket.is_close() {
                if !stack.pop().map(|b| b.matches(&bracket)).unwrap_or(false) {
                    // This line is corrupt, not incomplete
                    return None;
                }
            } else {
                stack.push(bracket);
            }
        }

        let mut score = 0;
        while let Some(bracket) = stack.pop() {
            let value = match bracket {
                RoundOpen => 1,
                SquareOpen => 2,
                CurlyOpen => 3,
                AngleOpen => 4,
                _ => unreachable!(),
            };
            score = 5 * score + value;
        }

        Some(score)
    }
}

pub fn part1(lines: &[Line]) -> u64 {
    lines.iter().filter_map(|line| line.score_corrupt()).sum()
}

pub fn part2(lines: &[Line]) -> Option<u64> {
    let mut incomplete = lines
        .iter()
        .filter_map(|line| line.score_incomplete())
        .collect::<Vec<_>>();
    incomplete.sort_unstable();
    incomplete.get(incomplete.len() / 2).copied()
}

impl TryFrom<char> for Bracket {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '(' => Ok(RoundOpen),
            ')' => Ok(RoundClose),
            '[' => Ok(SquareOpen),
            ']' => Ok(SquareClose),
            '{' => Ok(CurlyOpen),
            '}' => Ok(CurlyClose),
            '<' => Ok(AngleOpen),
            '>' => Ok(AngleClose),
            _ => Err(format!("Invalid bracket '{}'", ch)),
        }
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .chars()
            .map(Bracket::try_from)
            .collect::<Result<Vec<Bracket>, _>>()
            .map(Line)
    }
}
