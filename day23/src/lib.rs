use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashSet};
use std::convert::TryFrom;
use std::rc::Rc;
use std::str::FromStr;
use Amphipod::*;
use Space::*;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn move_energy(&self) -> u32 {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1_000,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Space {
    Hallway,
    Door,
    Room(Amphipod),
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> impl Iterator<Item = Self> + '_ {
        [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .into_iter()
            .map(|(dx, dy)| Position::new(self.x + dx, self.y + dy))
    }
}

struct Burrow(BTreeMap<Position, Space>);

impl Burrow {
    fn new(extended: bool) -> Self {
        let depth = if extended { 5 } else { 3 };

        let spaces = [
            (vec![1, 2, 4, 6, 8, 10, 11], Hallway),
            (vec![3, 5, 7, 9], Door),
        ]
        .into_iter()
        .flat_map(|(xs, space)| {
            xs.into_iter().map(move |x| (Position::new(x, 1), space))
        })
        .chain(
            [(3, Amber), (5, Bronze), (7, Copper), (9, Desert)]
                .into_iter()
                .flat_map(|(x, amphipod)| {
                    (2..=depth)
                        .into_iter()
                        .map(move |y| (Position::new(x, y), Room(amphipod)))
                }),
        )
        .collect::<BTreeMap<_, _>>();

        Self(spaces)
    }

    fn min_energy(&self, initial_state: &BurrowState) -> Option<u32> {
        let initial_state_rc = Rc::new(initial_state.clone());
        let mut min_energy = BTreeMap::from([(initial_state_rc.clone(), 0)]);
        let mut min_heap = BinaryHeap::from([(Reverse(0), initial_state_rc)]);

        let mut min = None;
        while let Some((Reverse(state_energy), state)) = min_heap.pop() {
            for (next_state, move_energy) in self.next_states(&state) {
                let next_energy = state_energy + move_energy;
                if min_energy
                    .get(&next_state)
                    .map(|e| *e <= next_energy)
                    .unwrap_or(false)
                {
                    continue;
                }

                if self.is_organized(&next_state) {
                    if min.map(|e| next_energy < e).unwrap_or(true) {
                        min = Some(next_energy);
                    }
                    continue;
                }

                let next_state_rc = Rc::new(next_state);
                min_heap.push((Reverse(next_energy), next_state_rc.clone()));
                min_energy
                    .entry(next_state_rc)
                    .and_modify(|e| *e = next_energy)
                    .or_insert(next_energy);
            }
        }
        min
    }

    fn is_organized(&self, state: &BurrowState) -> bool {
        state.0.iter().all(|(pos, amphipod)|
            matches!(self.0.get(pos), Some(Room(a)) if a == amphipod)
        )
    }

    fn room_mixed(&self, state: &BurrowState, amphipod: Amphipod) -> bool {
        self.0
            .iter()
            .filter(|(_, space)| matches!(space, Room(a) if *a == amphipod))
            .any(|(pos, _)| {
                state.0.get(pos).map(|a| *a != amphipod).unwrap_or(false)
            })
    }

    fn next_states<'a>(
        &'a self,
        state: &'a BurrowState,
    ) -> impl Iterator<Item = (BurrowState, u32)> + 'a {
        state.0.iter().flat_map(|(position, amphipod)| {
            self.valid_moves(state, *position, *amphipod).into_iter()
        })
    }

    fn valid_moves(
        &self,
        state: &BurrowState,
        initial_position: Position,
        amphipod: Amphipod,
    ) -> Vec<(BurrowState, u32)> {
        let mut valid_moves = Vec::new();
        let from_space = match self.0.get(&initial_position) {
            Some(&space) if space != Door => space,
            _ => return valid_moves,
        };

        let next_state = state.remove(initial_position);
        let mut visited = HashSet::from([initial_position]);
        let mut stack = vec![(initial_position, 0)];

        while let Some((position, energy)) = stack.pop() {
            for next_pos in position.adjacent() {
                let to_space = self.0.get(&next_pos);
                if to_space.is_none()
                    || visited.contains(&next_pos)
                    || state.is_occupied(&next_pos)
                {
                    continue;
                }

                let next_energy = energy + amphipod.move_energy();

                let valid_move = match (from_space, *to_space.unwrap()) {
                    (Hallway, Room(a)) => {
                        a == amphipod && !self.room_mixed(&next_state, a)
                    }
                    (Room(a), Room(b)) => {
                        a != amphipod
                            && b == amphipod
                            && !self.room_mixed(&next_state, b)
                    }
                    (Room(a), Hallway) => {
                        a != amphipod || self.room_mixed(&next_state, a)
                    }
                    _ => false,
                };
                if valid_move {
                    valid_moves.push((
                        next_state.add(next_pos, amphipod),
                        next_energy,
                    ));
                }

                visited.insert(next_pos);
                stack.push((next_pos, next_energy));
            }
        }

        valid_moves
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BurrowState(BTreeMap<Position, Amphipod>);

impl BurrowState {
    fn remove(&self, position: Position) -> Self {
        let mut state = self.0.clone();
        state.remove(&position);
        Self(state)
    }

    fn add(&self, position: Position, amphipod: Amphipod) -> Self {
        let mut state = self.0.clone();
        state.insert(position, amphipod);
        Self(state)
    }

    fn is_occupied(&self, position: &Position) -> bool {
        self.0.contains_key(position)
    }
}

pub fn part1(initial_state: &BurrowState) -> Option<u32> {
    Burrow::new(false).min_energy(initial_state)
}

pub fn part2(initial_state: &BurrowState) -> Option<u32> {
    let state = initial_state
        .0
        .iter()
        .map(|(pos, amphipod)| {
            if pos.y == 3 {
                (Position::new(pos.x, 5), *amphipod)
            } else {
                (*pos, *amphipod)
            }
        })
        .chain(
            [
                // #D#C#B#A#
                // #D#B#A#C#
                (3, [Desert, Desert]),
                (5, [Copper, Bronze]),
                (7, [Bronze, Amber]),
                (9, [Amber, Copper]),
            ]
            .into_iter()
            .flat_map(|(x, amphipods)| {
                amphipods
                    .into_iter()
                    .zip(3..)
                    .map(move |(amphipod, y)| (Position::new(x, y), amphipod))
            }),
        )
        .collect::<BTreeMap<_, _>>();

    Burrow::new(true).min_energy(&BurrowState(state))
}

impl TryFrom<char> for Amphipod {
    type Error = ();

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v.to_ascii_uppercase() {
            'A' => Ok(Amber),
            'B' => Ok(Bronze),
            'C' => Ok(Copper),
            'D' => Ok(Desert),
            _ => Err(()),
        }
    }
}

impl FromStr for BurrowState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .zip(0..)
            .flat_map(|(line, y)| {
                line.chars()
                    .zip(0..)
                    .filter(|(ch, _)| *ch != '#' && *ch != '.' && *ch != ' ')
                    .map(move |(ch, x)| {
                        Amphipod::try_from(ch)
                            .map(|amphipod| (Position::new(x, y), amphipod))
                            .map_err(|_| format!("Unknown amphipod '{}'", ch))
                    })
            })
            .collect::<Result<BTreeMap<_, _>, _>>()
            .map(Self)
    }
}
