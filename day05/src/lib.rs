use std::collections::HashMap;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct Line {
    point1: Point,
    point2: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.point1.y == self.point2.y
    }

    fn is_vertical(&self) -> bool {
        self.point1.x == self.point2.x
    }

    fn coord_deltas(&self) -> (i32, i32) {
        let delta_x = self.point2.x - self.point1.x;
        let delta_y = self.point2.y - self.point1.y;
        (delta_x.signum(), delta_y.signum())
    }
}

pub fn count_overlaps<'a, I>(lines: I) -> usize
where
    I: IntoIterator<Item = &'a Line>,
{
    let mut counter = HashMap::new();
    for line in lines.into_iter() {
        let (delta_x, delta_y) = line.coord_deltas();
        let mut point = line.point1;
        loop {
            counter
                .entry(point)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            if point == line.point2 {
                break;
            }
            point.x += delta_x;
            point.y += delta_y;
        }
    }
    counter.values().filter(|&count| *count > 1).count()
}

pub fn part1(lines: &[Line]) -> usize {
    let non_diagonals = lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical());
    count_overlaps(non_diagonals)
}

pub fn part2(lines: &[Line]) -> usize {
    count_overlaps(lines)
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split(',')
            .map(|num| {
                num.trim().parse::<i32>().map_err(|err| {
                    format!("Invalid coordinate '{}': {}", num, err)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        if numbers.len() != 2 {
            return Err(format!("Invalid point '{}'", s));
        }

        Ok(Point {
            x: numbers[0],
            y: numbers[1],
        })
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points: Vec<Point> = s
            .split("->")
            .map(|point| point.trim().parse())
            .collect::<Result<Vec<_>, _>>()?;

        if points.len() != 2 {
            return Err(format!("Invalid line '{}'", s));
        }

        let point2 = points.pop().unwrap();
        let point1 = points.pop().unwrap();

        let delta_x = point2.x - point1.x;
        let delta_y = point2.y - point1.y;
        if delta_x != 0 && delta_y != 0 && delta_x.abs() != delta_y.abs() {
            return Err(format!(
                "Invalid line '{}': Lines must be horizontal, vertical, or \
                45-degree diagonal",
                s
            ));
        }

        Ok(Line { point1, point2 })
    }
}
