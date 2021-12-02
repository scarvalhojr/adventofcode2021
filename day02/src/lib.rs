use std::str::FromStr;
use Command::*;

#[derive(Debug)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Default)]
struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn update(&mut self, command: &Command) {
        match command {
            Forward(units) => self.horizontal += units,
            Down(units) => self.depth += units,
            Up(units) => self.depth -= units,
        }
    }
}

pub fn part1(input: &[Command]) -> i32 {
    let final_position =
        input
            .iter()
            .fold(Position::default(), |mut position, command| {
                position.update(command);
                position
            });

    final_position.horizontal * final_position.depth
}

#[derive(Default)]
struct AimedPosition {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl AimedPosition {
    fn update(&mut self, command: &Command) {
        match command {
            Forward(units) => {
                self.horizontal += units;
                self.depth += self.aim * units;
            }
            Down(units) => self.aim += units,
            Up(units) => self.aim -= units,
        }
    }
}

pub fn part2(input: &[Command]) -> i32 {
    let final_position =
        input
            .iter()
            .fold(AimedPosition::default(), |mut position, command| {
                position.update(command);
                position
            });

    final_position.horizontal * final_position.depth
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd_str, units_str) = s
            .trim()
            .split_once(' ')
            .ok_or_else(|| format!("Incomplete command: {}", s))?;

        let units = units_str.parse().map_err(|err| {
            format!("Invalid units value '{}': {}", units_str, err)
        })?;

        match cmd_str.trim().to_lowercase().as_str() {
            "forward" => Ok(Command::Forward(units)),
            "down" => Ok(Command::Down(units)),
            "up" => Ok(Command::Up(units)),
            _ => Err(format!("Unknown command: {}", cmd_str)),
        }
    }
}
