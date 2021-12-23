use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::mem::swap;
use std::str::FromStr;
use Operation::*;

pub enum Operation {
    On,
    Off,
}

#[derive(Clone, Copy)]
pub struct Range {
    start: i32,
    end: i32,
}

#[derive(Clone)]
pub struct Region {
    x_range: Range,
    y_range: Range,
    z_range: Range,
}

pub struct Step {
    operation: Operation,
    region: Region,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Range {
    fn try_from_bounds(start: i32, end: i32) -> Option<Self> {
        if end >= start {
            Some(Self { start, end })
        } else {
            None
        }
    }

    fn len(&self) -> u64 {
        u64::try_from(self.end - self.start + 1).unwrap_or(0)
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        if other.start <= self.start && other.end >= self.end {
            Some(*self)
        } else if other.start >= self.start && other.start <= self.end {
            Self::try_from_bounds(other.start, min(self.end, other.end))
        } else if other.end >= self.start && other.end <= self.end {
            Self::try_from_bounds(max(self.start, other.start), other.end)
        } else {
            None
        }
    }
}

impl Region {
    fn new(x_range: Range, y_range: Range, z_range: Range) -> Self {
        Self {
            x_range,
            y_range,
            z_range,
        }
    }

    fn init_coordinates(&self) -> impl Iterator<Item = Coord> + '_ {
        let x_start = max(-50, self.x_range.start);
        let x_end = min(50, self.x_range.end);
        let y_start = max(-50, self.y_range.start);
        let y_end = min(50, self.y_range.end);
        let z_start = max(-50, self.z_range.start);
        let z_end = min(50, self.z_range.end);
        (x_start..=x_end).flat_map(move |x| {
            (y_start..=y_end).flat_map(move |y| {
                (z_start..=z_end).map(move |z| Coord::new(x, y, z))
            })
        })
    }

    fn count_cubes(&self) -> u64 {
        self.x_range.len() * self.y_range.len() * self.z_range.len()
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        let x_range = self.x_range.overlap(&other.x_range)?;
        let y_range = self.y_range.overlap(&other.y_range)?;
        let z_range = self.z_range.overlap(&other.z_range)?;
        Some(Self {
            x_range,
            y_range,
            z_range,
        })
    }

    fn split_off(&self, sub_region: &Region) -> Vec<Self> {
        let mut remain = Vec::new();

        if let Some(x_range) = Range::try_from_bounds(
            self.x_range.start,
            sub_region.x_range.start - 1,
        ) {
            remain.push(Region::new(x_range, self.y_range, self.z_range));
        }

        if let Some(x_range) =
            Range::try_from_bounds(sub_region.x_range.end + 1, self.x_range.end)
        {
            remain.push(Region::new(x_range, self.y_range, self.z_range));
        }

        if let Some(y_range) = Range::try_from_bounds(
            self.y_range.start,
            sub_region.y_range.start - 1,
        ) {
            remain.push(Region::new(sub_region.x_range, y_range, self.z_range));
        }

        if let Some(y_range) =
            Range::try_from_bounds(sub_region.y_range.end + 1, self.y_range.end)
        {
            remain.push(Region::new(sub_region.x_range, y_range, self.z_range));
        }

        if let Some(z_range) = Range::try_from_bounds(
            self.z_range.start,
            sub_region.z_range.start - 1,
        ) {
            remain.push(Region::new(
                sub_region.x_range,
                sub_region.y_range,
                z_range,
            ));
        }

        if let Some(z_range) =
            Range::try_from_bounds(sub_region.z_range.end + 1, self.z_range.end)
        {
            remain.push(Region::new(
                sub_region.x_range,
                sub_region.y_range,
                z_range,
            ));
        }

        remain
    }
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

struct InitArea(HashSet<Coord>);

impl InitArea {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn execute(&mut self, step: &Step) {
        match step.operation {
            On => self.0.extend(step.region.init_coordinates()),
            Off => {
                for coord in step.region.init_coordinates() {
                    self.0.remove(&coord);
                }
            }
        };
    }

    fn count_cubes(&self) -> usize {
        self.0.len()
    }
}

pub fn part1(steps: &[Step]) -> usize {
    let mut init_area = InitArea::new();
    for step in steps {
        init_area.execute(step);
    }
    init_area.count_cubes()
}

struct Reactor(Vec<Region>);

impl Reactor {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn turn_on(&mut self, region: Region) {
        let mut pending = vec![region];
        while let Some(sub_region) = pending.pop() {
            if let Some(overlap) =
                self.0.iter().find_map(|r| r.overlap(&sub_region))
            {
                pending.extend(sub_region.split_off(&overlap));
            } else {
                self.0.push(sub_region);
            }
        }
    }

    fn turn_off(&mut self, region: &Region) {
        let mut regions = self
            .0
            .drain(..)
            .flat_map(|r| {
                if let Some(overlap) = r.overlap(region) {
                    r.split_off(&overlap)
                } else {
                    vec![r]
                }
            })
            .collect();
        swap(&mut self.0, &mut regions);
    }

    fn execute(&mut self, step: &Step) {
        match step.operation {
            On => self.turn_on(step.region.clone()),
            Off => self.turn_off(&step.region),
        }
    }

    fn count_cubes(&self) -> u64 {
        self.0.iter().map(|region| region.count_cubes()).sum()
    }
}

pub fn part2(steps: &[Step]) -> u64 {
    let mut reactor = Reactor::new();
    for step in steps {
        reactor.execute(step);
    }
    reactor.count_cubes()
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "on" => Ok(On),
            "off" => Ok(Off),
            _ => Err(format!("Invalid operation '{}'", s)),
        }
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split("..")
            .map(|num| {
                num.parse()
                    .map_err(|_| format!("Invalid range number '{}'", num))
            })
            .collect::<Result<Vec<i32>, _>>()
            .and_then(|vec| match *vec.as_slice() {
                [start, end] if end >= start => Ok(Range { start, end }),
                _ => Err(format!("Invalid range '{}'", s)),
            })
    }
}

impl FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(concat!(
                r"^x=(\-?\d+\.\.\-?\d+),",
                r"y=(\-?\d+\.\.\-?\d+),",
                r"z=(\-?\d+\.\.\-?\d+)$",
            ))
            .unwrap();
        }

        let captures = REGEX
            .captures(s.trim())
            .ok_or(format!("Invalid region '{}'", s))?;

        captures
            .iter()
            .skip(1)
            .map(|cap| cap.unwrap().as_str().parse())
            .collect::<Result<Vec<_>, _>>()
            .and_then(|vec| match *vec.as_slice() {
                [x_range, y_range, z_range] => Ok(Self {
                    x_range,
                    y_range,
                    z_range,
                }),
                _ => Err(format!("Invalid region '{}'", s)),
            })
    }
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (oper_str, region_str) = s
            .split_once(' ')
            .ok_or_else(|| format!("Invalid step '{}'", s))?;
        let operation = oper_str.parse()?;
        let region = region_str.parse()?;
        Ok(Self { operation, region })
    }
}
