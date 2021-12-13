use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use Fold::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Dot {
    x: i32,
    y: i32,
}

impl Dot {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn fold(&self, fold: &Fold) -> Self {
        match fold {
            Left(line) => {
                let x = if self.x > *line {
                    2 * line - self.x
                } else {
                    self.x
                };
                let y = self.y;
                Self::new(x, y)
            }
            Up(line) => {
                let x = self.x;
                let y = if self.y > *line {
                    2 * line - self.y
                } else {
                    self.y
                };
                Self::new(x, y)
            }
        }
    }
}

pub enum Fold {
    Left(i32),
    Up(i32),
}

struct Paper(HashSet<Dot>);

impl Paper {
    fn new(dots: &[Dot]) -> Self {
        Self(dots.iter().copied().collect())
    }

    fn fold(&self, fold: &Fold) -> Self {
        let dots = self.0.iter().map(|dot| dot.fold(fold)).collect();
        Self(dots)
    }

    fn count_dots(&self) -> usize {
        self.0.len()
    }
}

pub fn part1(dots: &[Dot], folds: &[Fold]) -> usize {
    let mut paper = Paper::new(dots);
    if let Some(fold) = folds.iter().next() {
        paper = paper.fold(fold);
    }
    paper.count_dots()
}

pub fn part2(dots: &[Dot], folds: &[Fold]) {
    let paper = folds
        .iter()
        .fold(Paper::new(dots), |paper, fold| paper.fold(fold));
    println!("{}", paper);
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let min_x = self.0.iter().map(|dot| dot.x).min().unwrap_or(0);
        let max_x = self.0.iter().map(|dot| dot.x).max().unwrap_or(0);
        let min_y = self.0.iter().map(|dot| dot.y).min().unwrap_or(0);
        let max_y = self.0.iter().map(|dot| dot.y).max().unwrap_or(0);
        for y in min_y..=max_y {
            let line = (min_x..=max_x)
                .map(|x| {
                    if self.0.contains(&Dot::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl FromStr for Dot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s
            .trim()
            .split_once(',')
            .ok_or_else(|| format!("Invalid dot '{}'", s))?;
        let x = x_str
            .parse()
            .map_err(|_| format!("Invalid x coordinate '{}'", x_str))?;
        let y = y_str
            .parse()
            .map_err(|_| format!("Invalid y coordinate '{}'", y_str))?;
        Ok(Self { x, y })
    }
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fold = s.trim().to_ascii_lowercase().replace("fold along ", "");
        let (axis, val_str) = fold
            .split_once('=')
            .ok_or_else(|| format!("Invalid operation '{}'", s))?;
        let value = val_str
            .trim()
            .parse()
            .map_err(|_| format!("Invalid fold value '{}'", val_str))?;
        match axis.trim() {
            "y" => Ok(Up(value)),
            "x" => Ok(Left(value)),
            _ => Err(format!("Invalid fold axis '{}", axis)),
        }
    }
}
