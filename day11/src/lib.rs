use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coord {
    x: i8,
    y: i8,
}

impl Coord {
    fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> impl Iterator<Item = Self> + '_ {
        (-1..=1).flat_map(move |dx| {
            (-1..=1).filter_map(move |dy| {
                if dx != 0 || dy != 0 {
                    Some(Coord::new(self.x + dx, self.y + dy))
                } else {
                    None
                }
            })
        })
    }
}

#[derive(Clone)]
pub struct EnergyMap {
    map: HashMap<Coord, u8>,
    total_flashes: u32,
}

impl EnergyMap {
    fn update(&mut self) {
        let mut flashed = Vec::new();

        for (coord, energy) in self.map.iter_mut() {
            *energy += 1;
            if *energy == 10 {
                *energy = 0;
                flashed.push(*coord);
            }
        }

        while let Some(coord) = flashed.pop() {
            self.total_flashes += 1;
            for adjacent in coord.adjacent() {
                if let Some(energy) = self.map.get_mut(&adjacent) {
                    if *energy > 0 {
                        *energy += 1;
                        if *energy == 10 {
                            *energy = 0;
                            flashed.push(adjacent);
                        }
                    }
                }
            }
        }
    }

    fn total_flashes(&self) -> u32 {
        self.total_flashes
    }

    fn all_flashed(&self) -> bool {
        self.map.values().all(|energy| *energy == 0)
    }
}

pub fn part1(start_map: &EnergyMap) -> u32 {
    let mut map = start_map.clone();
    for _step in 1..=100 {
        map.update();
    }
    map.total_flashes()
}

pub fn part2(start_map: &EnergyMap) -> u32 {
    let mut map = start_map.clone();
    for step in 1.. {
        map.update();
        if map.all_flashed() {
            return step;
        }
    }
    unreachable!()
}

impl FromStr for EnergyMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .lines()
            .zip(0..)
            .flat_map(|(line, y)| {
                line.trim().chars().zip(0..).map(move |(ch, x)| {
                    ch.to_digit(10)
                        .ok_or_else(|| format!("Invalid energy level '{}'", ch))
                        .map(|num| {
                            (Coord::new(x, y), u8::try_from(num).unwrap())
                        })
                })
            })
            .collect::<Result<HashMap<_, _>, _>>()
            .map(|map| EnergyMap {
                map,
                total_flashes: 0,
            })
    }
}
