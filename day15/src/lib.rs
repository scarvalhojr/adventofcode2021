use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> impl Iterator<Item = Self> + '_ {
        [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .into_iter()
            .map(|(dx, dy)| Coord::new(self.x + dx, self.y + dy))
    }

    fn add(&self, x_add: i32, y_add: i32) -> Self {
        Self {
            x: self.x + x_add,
            y: self.y + y_add,
        }
    }
}

pub struct RiskMap(HashMap<Coord, i32>);

impl RiskMap {
    fn get_risk(&self, coord: &Coord) -> Option<i32> {
        self.0.get(coord).copied()
    }

    fn lowest_total_risk(&self) -> Option<i32> {
        let target = self.0.keys().max()?;

        let mut lowest = HashMap::from([(Coord::default(), 0)]);
        let mut heap = BinaryHeap::from([Reverse((0, Coord::default()))]);

        while let Some(Reverse((curr_risk, coord))) = heap.pop() {
            for adjacent in coord.adjacent() {
                if let Some(adj_risk) = self.get_risk(&adjacent) {
                    if lowest
                        .get(&adjacent)
                        .map(|&r| r > curr_risk + adj_risk)
                        .unwrap_or(true)
                    {
                        let new_risk = curr_risk + adj_risk;
                        lowest.insert(adjacent.clone(), new_risk);
                        heap.push(Reverse((new_risk, adjacent)));
                    }
                }
            }
        }

        lowest.remove(target)
    }

    fn enlarge(&self, x_mult: i32, y_mult: i32) -> Self {
        let wrap = |num| (num - 1) % 9 + 1;
        let x_dim = 1 + self.0.keys().map(|coord| coord.x).max().unwrap_or(-1);
        let y_dim = 1 + self.0.keys().map(|coord| coord.y).max().unwrap_or(-1);
        let map = self
            .0
            .iter()
            .flat_map(|(coord, risk)| {
                (0..x_mult).flat_map(move |mx| {
                    (0..y_mult).map(move |my| {
                        (
                            coord.add(mx * x_dim, my * y_dim),
                            wrap(risk + mx + my),
                        )
                    })
                })
            })
            .collect();
        Self(map)
    }
}

pub fn part1(risk_map: &RiskMap) -> Option<i32> {
    risk_map.lowest_total_risk()
}

pub fn part2(risk_map: &RiskMap) -> Option<i32> {
    risk_map.enlarge(5, 5).lowest_total_risk()
}

impl FromStr for RiskMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .zip(0..)
            .flat_map(|(line, y)| {
                line.chars().zip(0..).map(move |(ch, x)| {
                    ch.to_digit(10)
                        .map(|num| {
                            (Coord::new(x, y), i32::try_from(num).unwrap())
                        })
                        .ok_or_else(|| format!("Invalid risk level '{}'", ch))
                })
            })
            .collect::<Result<HashMap<_, _>, _>>()
            .map(Self)
    }
}
