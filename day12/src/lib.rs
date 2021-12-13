use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type CaveName = String;

const START_CAVE: &str = "start";
const END_CAVE: &str = "end";

fn is_small(cave: &str) -> bool {
    cave.chars().all(|ch| ch.is_lowercase())
}

pub struct CaveSystem(HashMap<CaveName, Vec<CaveName>>);

impl CaveSystem {
    fn get_connections(&self, cave: &str) -> Option<&Vec<CaveName>> {
        self.0.get(cave)
    }

    fn count_all_paths(&self, allow_small_reentrance: bool) -> Option<i32> {
        let start = Path::new(START_CAVE, self.get_connections(START_CAVE)?);
        let mut stack = vec![start];
        let mut count = 0;

        while let Some(mut path) = stack.pop() {
            if let Some(current) = path.next_cave(allow_small_reentrance) {
                stack.push(path);

                if current == END_CAVE {
                    count += 1;
                    continue;
                }

                if let Some(connections) = self.get_connections(&current) {
                    let next_path =
                        stack.last().unwrap().next_path(current, connections);
                    stack.push(next_path);
                }
            }
        }

        Some(count)
    }
}

#[derive(Debug)]
struct Path {
    visited: HashSet<CaveName>,
    current: CaveName,
    connections: Vec<CaveName>,
    small_reentered: bool,
}

impl Path {
    fn new(start: &str, connections: &[CaveName]) -> Self {
        Self {
            visited: HashSet::new(),
            current: start.to_string(),
            connections: connections.iter().map(|s| s.to_string()).collect(),
            small_reentered: false,
        }
    }

    fn next_cave(&mut self, allow_small_reentrance: bool) -> Option<CaveName> {
        while let Some(cave) = self.connections.pop() {
            if self.visited.contains(&cave)
                && (!allow_small_reentrance || self.small_reentered)
            {
                continue;
            }
            return Some(cave);
        }
        None
    }

    fn next_path(&self, current: CaveName, connections: &[CaveName]) -> Self {
        let mut visited = self.visited.clone();
        if is_small(&self.current) {
            // Only keep track of visited caves when they're small
            visited.insert(self.current.clone());
        }

        let connections = connections
            .iter()
            .filter(|c| c.as_str() != START_CAVE)
            .map(|c| c.to_string())
            .collect();

        let small_reentered =
            self.small_reentered || visited.contains(&current);

        Self {
            visited,
            current,
            connections,
            small_reentered,
        }
    }
}

pub fn part1(caves: &CaveSystem) -> Option<i32> {
    caves.count_all_paths(false)
}

pub fn part2(caves: &CaveSystem) -> Option<i32> {
    caves.count_all_paths(true)
}

impl FromStr for CaveSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut connections = HashMap::new();
        for line in s.trim().lines() {
            let (cave1, cave2) = line
                .trim()
                .split_once('-')
                .ok_or_else(|| format!("Invalid connection '{}'", line))?;
            connections
                .entry(cave1.to_string())
                .and_modify(|v: &mut Vec<_>| v.push(cave2.to_string()))
                .or_insert_with(|| vec![cave2.to_string()]);
            connections
                .entry(cave2.to_string())
                .and_modify(|v: &mut Vec<_>| v.push(cave1.to_string()))
                .or_insert_with(|| vec![cave1.to_string()]);
        }
        Ok(CaveSystem(connections))
    }
}
