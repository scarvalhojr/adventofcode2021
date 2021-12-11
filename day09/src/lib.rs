use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct HeightMap(HashMap<(i32, i32), u8>);

impl HeightMap {
    fn low_points(&self) -> impl Iterator<Item = (i32, i32, u8)> + '_ {
        self.0.iter().filter_map(|(&(row, col), &height)| {
            if self.is_low_point(row, col, height) {
                Some((row, col, height))
            } else {
                None
            }
        })
    }

    fn is_low_point(&self, row: i32, col: i32, height: u8) -> bool {
        [
            (row - 1, col),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col),
        ]
        .iter()
        .all(|&(r, c)| self.0.get(&(r, c)).map(|&h| h > height).unwrap_or(true))
    }

    fn total_risk_level(&self) -> u32 {
        self.low_points()
            .map(|(_, _, height)| height as u32 + 1)
            .sum()
    }

    fn basin_size(&self, start_row: i32, start_col: i32) -> usize {
        let mut points = HashSet::from([(start_row, start_col)]);
        let mut queue = vec![(start_row, start_col)];

        while let Some((row, col)) = queue.pop() {
            queue.extend(
                [
                    (row - 1, col),
                    (row, col - 1),
                    (row, col + 1),
                    (row + 1, col),
                ]
                .into_iter()
                .filter(|&(r, c)| {
                    self.0.get(&(r, c)).map(|&h| h < 9).unwrap_or(false)
                        && points.insert((r, c))
                }),
            );
        }

        points.len()
    }
}

pub fn part1(height_map: &HeightMap) -> u32 {
    height_map.total_risk_level()
}

pub fn part2(height_map: &HeightMap) -> usize {
    let mut sizes = height_map
        .low_points()
        .map(|(row, col, _)| height_map.basin_size(row, col))
        .collect::<Vec<_>>();
    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .zip(0..)
            .flat_map(|(line, row)| {
                line.chars().zip(0..).map(move |(ch, col)| {
                    ch.to_digit(10)
                        .ok_or(format!("Invalid height value '{}'", ch))
                        .map(|val| ((row, col), u8::try_from(val).unwrap()))
                })
            })
            .collect::<Result<HashMap<_, _>, _>>()
            .map(HeightMap)
    }
}
