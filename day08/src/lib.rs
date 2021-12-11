use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::str::FromStr;
use Segment::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Segment {
    A = 1,
    B = 2,
    C = 4,
    D = 8,
    E = 16,
    F = 32,
    G = 64,
}

type SegmentMap = HashMap<Segment, Segment>;
type Signal = HashSet<Segment>;

fn map_signal(signal: &Signal, map: &SegmentMap) -> Option<Signal> {
    signal
        .iter()
        .map(|s| map.get(s).copied())
        .collect::<Option<HashSet<_>>>()
}

fn signal_to_digit<'a, I>(signal: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a Segment>,
{
    match signal.into_iter().map(|s| *s as u8).sum() {
        119 => Some(0),
        36 => Some(1),
        93 => Some(2),
        109 => Some(3),
        46 => Some(4),
        107 => Some(5),
        123 => Some(6),
        37 => Some(7),
        127 => Some(8),
        111 => Some(9),
        _ => None,
    }
}

pub struct Display {
    patterns: Vec<Signal>,
    output: Vec<Signal>,
}

impl Display {
    fn count_easy_digits(&self) -> usize {
        self.output
            .iter()
            .filter(|signal| matches!(signal.len(), 2 | 3 | 4 | 7))
            .count()
    }

    fn decode(&self) -> Option<u32> {
        // 1: {C, F}
        let pat1 = match self
            .patterns
            .iter()
            .filter(|p| p.len() == 2)
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(x),
            _ => None,
        }?;

        // 7: {A, C, F}
        let pat7 = match self
            .patterns
            .iter()
            .filter(|p| p.len() == 3)
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(x),
            _ => None,
        }?;

        // pat7 - pat1 = {A, C, F} - {C, F} => {A}
        let seg_a = match pat7.difference(pat1).collect::<Vec<_>>()[..] {
            [x] => Some(*x),
            _ => None,
        }?;

        // 4: {B, C, D, F}
        let pat4 = match self
            .patterns
            .iter()
            .filter(|p| p.len() == 4)
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(x),
            _ => None,
        }?;

        // 9: {A, B, C, D, F, G}
        // {A, B, C, D, F, G} - {B, C, D, F} = {A, G}
        let pat9 = match self
            .patterns
            .iter()
            .filter(|p| {
                p.len() == 6
                    && p.difference(pat4).collect::<HashSet<_>>().len() == 2
            })
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(x),
            _ => None,
        }?;

        // pat9 - pat4 - {A} = {A, B, C, D, F, G} - {B, C, D, F} - {A} => {G}
        let seg_g = match pat9
            .difference(pat4)
            .filter(|&&s| s != seg_a)
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(*x),
            _ => None,
        }?;

        // 3: {A, C, D, F, G}
        // {A, C, D, F, G} - {A, C, F} = {D, G}
        let pat3 = match self
            .patterns
            .iter()
            .filter(|p| {
                p.len() == 5
                    && p.difference(pat7).collect::<HashSet<_>>().len() == 2
            })
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(x),
            _ => None,
        }?;

        // pat3 - pat7 - {G} = {A, C, D, F, G} - {A, C, F} - {G} => {D}
        let seg_d = match pat3
            .difference(pat7)
            .filter(|&&s| s != seg_g)
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(*x),
            _ => None,
        }?;

        // 2: {A, C, D, E, G}
        // {A, C, D, E, G} - {A, B, C, D, F, G} = {E}
        let pat2 = match self
            .patterns
            .iter()
            .filter(|p| {
                p.len() == 5
                    && p.difference(pat9).collect::<HashSet<_>>().len() == 1
            })
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(x),
            _ => None,
        }?;

        // pat2 - pat9 = {A, C, D, E, G} - {A, B, C, D, F, G} => {E}
        let seg_e = match pat2.difference(pat9).collect::<Vec<_>>()[..] {
            [x] => Some(*x),
            _ => None,
        }?;

        // pat2 - {A, D, E, G} = {A, C, D, E, G} - {A, D, E, G} = {C}
        let seg_c = match pat2
            .iter()
            .filter(|&&s| s != seg_a && s != seg_d && s != seg_e && s != seg_g)
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(*x),
            _ => None,
        }?;

        // pat1 - {C} = {C, F} - {C} = {F}
        let seg_f = match pat1
            .iter()
            .filter(|&&s| s != seg_c)
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(*x),
            _ => None,
        }?;

        // pat4 - {C, D, F} = {B, C, D, F} - {C, D, F} = {B}
        let seg_b = match pat4
            .iter()
            .filter(|&&s| s != seg_c && s != seg_d && s != seg_f)
            .collect::<Vec<_>>()[..]
        {
            [x] => Some(*x),
            _ => None,
        }?;

        let seg_map: SegmentMap = HashMap::from([
            (seg_a, A),
            (seg_b, B),
            (seg_c, C),
            (seg_d, D),
            (seg_e, E),
            (seg_f, F),
            (seg_g, G),
        ]);

        let res = self.output
            .iter()
            .try_fold(0, |acc, signal|
                map_signal(signal, &seg_map)
                    .and_then(|signal| signal_to_digit(&signal))
                    .map(|digit| acc * 10 + digit)
            );
        res
    }
}

pub fn part1(display_entries: &[Display]) -> usize {
    display_entries
        .iter()
        .map(|display| display.count_easy_digits())
        .sum()
}

pub fn part2(display_entries: &[Display]) -> Option<u32> {
    display_entries
        .iter()
        .map(|display| display.decode())
        .collect::<Option<Vec<_>>>()
        .map(|values| values.iter().sum())
}

impl TryFrom<char> for Segment {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch.to_ascii_uppercase() {
            'A' => Ok(A),
            'B' => Ok(B),
            'C' => Ok(C),
            'D' => Ok(D),
            'E' => Ok(E),
            'F' => Ok(F),
            'G' => Ok(G),
            _ => Err(format!("Invalid segment '{}'", ch)),
        }
    }
}

impl FromStr for Display {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns_str, output_str) = s
            .split_once('|')
            .ok_or(format!("Invalid display entry '{}'", s))?;

        let patterns = patterns_str
            .split_whitespace()
            .map(|pat_str| {
                pat_str
                    .chars()
                    .map(Segment::try_from)
                    .collect::<Result<Signal, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let output = output_str
            .split_whitespace()
            .map(|out_str| {
                out_str
                    .chars()
                    .map(Segment::try_from)
                    .collect::<Result<Signal, _>>()
            })
            .collect::<Result<Vec<Signal>, _>>()?;

        Ok(Self { patterns, output })
    }
}
