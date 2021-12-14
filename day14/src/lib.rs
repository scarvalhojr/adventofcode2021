use std::collections::HashMap;
use std::str::FromStr;

pub struct InsertionRule {
    pair: [char; 2],
    insert: char,
}

pub struct RuleMap(HashMap<[char; 2], char>);

impl RuleMap {
    pub fn new(rules: Vec<InsertionRule>) -> Self {
        Self(rules.iter().map(|r| (r.pair, r.insert)).collect())
    }

    fn get_insertion(&self, pair: &[char; 2]) -> Option<char> {
        self.0.get(pair).copied()
    }
}

#[derive(Clone)]
pub struct Polymer {
    pair_count: HashMap<[char; 2], u64>,
    end: char,
}

impl Polymer {
    fn grow(&self, rules: &RuleMap) -> Option<Self> {
        let mut pair_count = HashMap::new();
        for (pair, &count) in self.pair_count.iter() {
            let insert = rules.get_insertion(pair)?;
            for new_pair in [[pair[0], insert], [insert, pair[1]]] {
                pair_count
                    .entry(new_pair)
                    .and_modify(|total| *total += count)
                    .or_insert(count);
            }
        }

        Some(Self {
            pair_count,
            end: self.end,
        })
    }

    fn frequency_delta(&self) -> u64 {
        let mut freq = [(self.end, 1)].into_iter().collect::<HashMap<_, _>>();
        for (pair, &count) in self.pair_count.iter() {
            freq.entry(pair[0])
                .and_modify(|total| *total += count)
                .or_insert(count);
        }

        let max = freq.values().max().unwrap_or(&0);
        let min = freq.values().min().unwrap_or(&0);
        max - min
    }
}

fn solve(template: &Polymer, rules: &RuleMap, steps: u32) -> Option<u64> {
    let mut polymer = template.clone();
    for _ in 1..=steps {
        polymer = polymer.grow(rules)?;
    }
    Some(polymer.frequency_delta())
}

pub fn part1(template: &Polymer, rules: &RuleMap) -> Option<u64> {
    solve(template, rules, 10)
}

pub fn part2(template: &Polymer, rules: &RuleMap) -> Option<u64> {
    solve(template, rules, 40)
}

impl FromStr for Polymer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pair_count = HashMap::new();
        for pair in s
            .chars()
            .collect::<Vec<_>>()
            .as_slice()
            .windows(2)
            .flat_map(<&[char; 2]>::try_from)
        {
            pair_count
                .entry(*pair)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        if pair_count.is_empty() {
            return Err("Invalid polymer template length".to_string());
        }

        let end = s.chars().last().unwrap();

        Ok(Self { pair_count, end })
    }
}

impl FromStr for InsertionRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pair_str, ch_str) = s
            .split_once("->")
            .ok_or_else(|| format!("Invalid insertion rule '{}'", s))?;

        let pair = <[char; 2]>::try_from(
            pair_str.trim().chars().collect::<Vec<_>>().as_slice(),
        )
        .map_err(|_| format!("Invalid pair '{}'", pair_str))?;

        let insert = <[char; 1]>::try_from(
            ch_str.trim().chars().collect::<Vec<_>>().as_slice(),
        )
        .map_err(|_| format!("Invalid element '{}'", ch_str))?[0];

        Ok(Self { pair, insert })
    }
}
