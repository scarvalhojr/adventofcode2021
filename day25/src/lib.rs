use std::collections::HashMap;
use std::str::FromStr;
use Herd::*;

#[derive(Clone, Copy, PartialEq)]
enum Herd {
    East,
    South,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
pub struct Region {
    x_dim: i32,
    y_dim: i32,
    positions: HashMap<Position, Herd>,
}

impl Region {
    fn step(&self) -> Option<Self> {
        if let Some(next_region) = self.move_herd(&East) {
            next_region.move_herd(&South)
        } else {
            self.move_herd(&South)
        }
    }

    fn move_herd(&self, herd: &Herd) -> Option<Self> {
        let mut moved = false;
        let positions = self
            .positions
            .iter()
            .map(|(pos, sea_cucumber)| {
                if sea_cucumber == herd {
                    if let Some(next_pos) = self.move_if_free(herd, pos) {
                        moved = true;
                        (next_pos, *sea_cucumber)
                    } else {
                        (pos.clone(), *sea_cucumber)
                    }
                } else {
                    (pos.clone(), *sea_cucumber)
                }
            })
            .collect();
        if moved {
            Some(Self {
                x_dim: self.x_dim,
                y_dim: self.y_dim,
                positions,
            })
        } else {
            None
        }
    }

    fn move_if_free(&self, herd: &Herd, pos: &Position) -> Option<Position> {
        let next_pos = match herd {
            East => Position::new((pos.x + 1) % self.x_dim, pos.y),
            South => Position::new(pos.x, (pos.y + 1) % self.y_dim),
        };
        if self.positions.contains_key(&next_pos) {
            None
        } else {
            Some(next_pos)
        }
    }
}

pub fn part1(initial_region: &Region) -> u32 {
    let mut steps = 1;
    let mut region = initial_region.clone();
    while let Some(next_region) = region.step() {
        region = next_region;
        steps += 1;
    }
    steps
}

impl TryFrom<char> for Herd {
    type Error = ();

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v.to_ascii_lowercase() {
            '>' => Ok(East),
            'v' => Ok(South),
            _ => Err(()),
        }
    }
}

impl FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .lines()
            .zip(0..)
            .flat_map(|(line, y)| {
                line.chars().zip(0..).filter(|(ch, _)| *ch != '.').map(
                    move |(ch, x)| {
                        Herd::try_from(ch)
                            .map(|herd| (Position::new(x, y), herd))
                            .map_err(|_| format!("Invalid input '{}'", ch))
                    },
                )
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        let x_dim = 1 + positions.keys().map(|pos| pos.x).max().unwrap_or(0);
        let y_dim = 1 + positions.keys().map(|pos| pos.y).max().unwrap_or(0);
        Ok(Self {
            x_dim,
            y_dim,
            positions,
        })
    }
}
