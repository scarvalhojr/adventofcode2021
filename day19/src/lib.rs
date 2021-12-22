use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use Plane::*;

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct Coordinates {
    x: i32,
    y: i32,
    z: i32,
}

enum Plane {
    XY,
    XZ,
    YZ,
}

/// Return an iterator of planes which rotates a 3-D set of coordinates
/// around all possible orientations; while there are 24 unique orientations,
/// this function will generate three repeated states for a total of 27
/// orientations (including the initial orientation)
fn all_rotations() -> impl Iterator<Item = Plane> {
    [
        XY, XY, XY, XZ, XY, XY, XY, XZ, XY, XY, XY, YZ, XY, XY, XY, XY, YZ, XY,
        XY, XY, YZ, XZ, XZ, XY, XY, XY,
    ]
    .into_iter()
}

impl Coordinates {
    fn distance(&self, other: &Self) -> Self {
        let x = other.x - self.x;
        let y = other.y - self.y;
        let z = other.z - self.z;
        Self { x, y, z }
    }

    fn rotate(&self, plane: &Plane) -> Self {
        let (x, y, z) = match plane {
            Plane::XY => (self.y, -self.x, self.z),
            Plane::XZ => (self.z, self.y, -self.x),
            Plane::YZ => (self.x, self.z, -self.y),
        };
        Self { x, y, z }
    }

    fn move_by(&self, shift: &Self) -> Self {
        let x = self.x + shift.x;
        let y = self.y + shift.y;
        let z = self.z + shift.z;
        Self { x, y, z }
    }

    fn manhattan_distance(&self, other: &Self) -> i32 {
        (other.x - self.x).abs()
            + (other.y - self.y).abs()
            + (other.z - self.z).abs()
    }
}

#[derive(Clone)]
pub struct Scanner {
    position: Coordinates,
    beacons: HashSet<Coordinates>,
}

impl Scanner {
    fn rotate(&self, plane: &Plane) -> Self {
        let position = self.position.rotate(plane);
        let beacons = self
            .beacons
            .iter()
            .map(|beacon| beacon.rotate(plane))
            .collect();
        Self { position, beacons }
    }

    fn move_by(&self, shift: &Coordinates) -> Self {
        let position = self.position.move_by(shift);
        let beacons = self
            .beacons
            .iter()
            .map(|beacon| beacon.move_by(shift))
            .collect();
        Self { position, beacons }
    }

    fn can_align_to(&self, other: &Self) -> Option<Coordinates> {
        let mut counter = HashMap::new();
        for distance in self.beacons.iter().flat_map(|my_beacon| {
            other
                .beacons
                .iter()
                .map(|their_beacon| my_beacon.distance(their_beacon))
        }) {
            if *counter
                .entry(distance.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1)
                >= 12
            {
                return Some(distance);
            }
        }
        None
    }

    fn align_to(&self, other: &Self) -> Option<Scanner> {
        let mut scanner = self.clone();
        let mut planes = all_rotations();
        loop {
            if let Some(shift) = scanner.can_align_to(other) {
                return Some(scanner.move_by(&shift));
            }

            if let Some(plane) = planes.next() {
                scanner = scanner.rotate(&plane);
            } else {
                break;
            }
        }
        None
    }
}

pub fn solve(scanners: &[Scanner]) -> Option<(usize, i32)> {
    let mut aligned = Vec::new();
    let mut aligning = Vec::from([scanners.first()?.clone()]);
    let mut pending = scanners.iter().skip(1).collect::<Vec<_>>();

    while let Some(aligning_scanner) = aligning.pop() {
        let mut skipped = Vec::new();
        while let Some(pending_scanner) = pending.pop() {
            if let Some(scanner) = pending_scanner.align_to(&aligning_scanner) {
                aligning.push(scanner);
            } else {
                skipped.push(pending_scanner);
            }
        }
        aligned.push(aligning_scanner);
        pending = skipped;
    }

    if !pending.is_empty() {
        // Not all scanners could be aligned
        return None;
    }

    let unique_beacons = aligned
        .iter()
        .flat_map(|scanner| scanner.beacons.iter())
        .collect::<HashSet<_>>()
        .len();

    let max_distance = aligned
        .iter()
        .zip(1..)
        .flat_map(|(scanner1, index)| {
            aligned[index..].iter().map(|scanner2| {
                scanner1.position.manhattan_distance(&scanner2.position)
            })
        })
        .max()?;

    Some((unique_beacons, max_distance))
}

impl FromStr for Coordinates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|num| {
                num.trim().parse::<i32>().map_err(|err| {
                    format!("Invalid coordinate '{}': {}", num, err)
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .and_then(|vec| match vec[..] {
                [x, y, z] => Ok(Coordinates { x, y, z }),
                _ => Err(format!("Invalid coordinates '{}'", s)),
            })
    }
}

impl FromStr for Scanner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("--- scanner") {
            return Err("Missing scanner header line".to_string());
        }

        let position = Coordinates::default();
        let beacons = s
            .lines()
            .skip(1)
            .map(|line| line.parse())
            .collect::<Result<HashSet<_>, _>>()?;

        Ok(Self { position, beacons })
    }
}
