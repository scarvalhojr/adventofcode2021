use regex::Regex;
use std::str::FromStr;

pub struct Target {
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
}

#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn update(&mut self, velocity: &Velocity) {
        self.x += velocity.delta_x;
        self.y += velocity.delta_y;
    }

    fn in_target(&self, target: &Target) -> bool {
        self.x >= target.start_x
            && self.x <= target.end_x
            && self.y >= target.start_y
            && self.y <= target.end_y
    }

    fn past_target(&self, target: &Target) -> bool {
        (self.x > 0 && self.x > target.end_x)
            || (self.x < 0 && self.x < target.start_x)
            || self.y < target.start_y
    }
}

fn viable_velocities(target: &Target) -> impl Iterator<Item = Velocity> + '_ {
    let dx_range = if target.start_x <= 0 && target.end_x >= 0 {
        target.start_x..=target.end_x
    } else if target.start_x > 0 {
        1..=target.end_x
    } else {
        // target.end_x < 0
        target.start_x..=-1
    };
    let dy_range = if target.start_y <= 0 && target.end_y >= 0 {
        target.start_y..=target.end_y.max(-target.start_y - 1)
    } else if target.start_y > 0 {
        1..=target.end_y
    } else {
        // target.end_y < 0
        target.start_y..=-target.start_y - 1
    };

    dx_range.flat_map(move |delta_x| {
        dy_range.clone().filter_map(move |delta_y| {
            let velocity = Velocity::new(delta_x, delta_y);
            if velocity.hits_target(target) {
                Some(velocity)
            } else {
                None
            }
        })
    })
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Velocity {
    delta_x: i32,
    delta_y: i32,
}

impl Velocity {
    fn new(delta_x: i32, delta_y: i32) -> Self {
        Self { delta_x, delta_y }
    }

    fn update(&mut self) {
        self.delta_x -= self.delta_x.signum();
        self.delta_y -= 1;
    }

    fn max_height(&self) -> i32 {
        if self.delta_y > 0 {
            (self.delta_y * (self.delta_y + 1)) / 2
        } else {
            0
        }
    }

    fn hits_target(&self, target: &Target) -> bool {
        let mut velocity = self.clone();
        let mut position = Position::default();

        loop {
            if position.in_target(target) {
                return true;
            }
            position.update(&velocity);
            if position.past_target(target) {
                break;
            }
            velocity.update();
        }

        false
    }
}

pub fn part1(target: &Target) -> Option<i32> {
    viable_velocities(target)
        .map(|velocity| velocity.max_height())
        .max()
}

pub fn part2(target: &Target) -> usize {
    viable_velocities(target).count()
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Regex::new(concat!(
            r"^target area:\s*x=(\-?\d+)\.\.(\-?\d+),\s*",
            r"y=(\-?\d+)\.\.(\-?\d+)$",
        ))
        .unwrap()
        .captures(s.trim())
        .ok_or_else(|| "Invalid target format".to_string())?
        .iter()
        .skip(1)
        .map(|cap| {
            cap.unwrap().as_str().parse().map_err(|err| {
                format!("Invalid number in target range: {}", err)
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|vec| match *vec.as_slice() {
            [start_x, end_x, start_y, end_y] => Self {
                start_x,
                end_x,
                start_y,
                end_y,
            },
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_sample() {
        let target = Target {
            start_x: 20,
            end_x: 30,
            start_y: -10,
            end_y: -5,
        };
        let velocity = Velocity::new(6, 9);
        assert_eq!(velocity.max_height(), 45);
        assert_eq!(part1(&target), Some(45));
        assert_eq!(part2(&target), 112);
    }

    #[test]
    fn solves_negative_x_range() {
        let target = Target {
            start_x: -30,
            end_x: -20,
            start_y: -10,
            end_y: -5,
        };
        let velocity = Velocity::new(-6, 9);
        assert_eq!(velocity.max_height(), 45);
        assert_eq!(part1(&target), Some(45));
        assert_eq!(part2(&target), 112);
    }

    #[test]
    fn solves_input1() {
        let target = Target {
            start_x: 269,
            end_x: 292,
            start_y: -68,
            end_y: -44,
        };
        let velocity = Velocity::new(23, 67);
        assert_eq!(velocity.max_height(), 2278);
        assert_eq!(part1(&target), Some(2278));
        assert_eq!(part2(&target), 996);
    }

    #[test]
    fn solves_input2() {
        let target = Target {
            start_x: 288,
            end_x: 330,
            start_y: -96,
            end_y: -50,
        };
        let velocity = Velocity::new(25, 95);
        assert_eq!(velocity.max_height(), 4560);
        assert_eq!(part1(&target), Some(4560));
        assert_eq!(part2(&target), 3344);
    }
}
