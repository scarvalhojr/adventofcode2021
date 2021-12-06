use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

const RESTART_TIMER: usize = 6;
const NEW_TIMER: usize = 8;

#[derive(Clone)]
pub struct Population {
    timer_counts: VecDeque<u64>,
}

impl Population {
    fn next(&mut self) {
        let spawn = self.timer_counts.pop_front().unwrap();
        *self.timer_counts.get_mut(RESTART_TIMER).unwrap() += spawn;
        self.timer_counts.push_back(spawn);
    }

    fn count(&self) -> u64 {
        self.timer_counts.iter().sum()
    }
}

pub fn simulate(start_population: &Population, days: u32) -> u64 {
    let mut population = start_population.clone();
    for _ in 1..=days {
        population.next()
    }
    population.count()
}

impl FromStr for Population {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timers = s
            .trim()
            .split(',')
            .map(|num| {
                num.trim()
                    .parse()
                    .map_err(|err| format!("Invalid timer '{}': {}", num, err))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if timers.iter().any(|&timer| timer > NEW_TIMER) {
            return Err(format!("Timers must be less than {}", NEW_TIMER));
        }

        let totals =
            timers.iter().fold(HashMap::new(), |mut counters, timer| {
                *counters.entry(timer).or_insert(0) += 1;
                counters
            });

        let timer_counts = (0..=NEW_TIMER)
            .map(|timer| *totals.get(&timer).unwrap_or(&0))
            .collect();

        Ok(Self { timer_counts })
    }
}
