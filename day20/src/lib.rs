use std::collections::BTreeSet;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use PixelState::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PixelState {
    Dark,
    Light,
}

impl PixelState {
    fn reverse(&self) -> Self {
        match *self {
            Light => Dark,
            Dark => Light,
        }
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pixel {
    x: i32,
    y: i32,
}

impl Pixel {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct EnhanceAlgo([PixelState; 512]);

impl EnhanceAlgo {
    fn pixel_state(&self, index: usize) -> PixelState {
        self.0[index]
    }
}

#[derive(Clone)]
pub struct Image {
    pixel_state: PixelState,
    pixels: BTreeSet<Pixel>,
}

impl Image {
    fn new(pixel_state: PixelState, pixels: BTreeSet<Pixel>) -> Self {
        Self {
            pixel_state,
            pixels,
        }
    }

    fn get_pixel_state(&self, x: i32, y: i32) -> PixelState {
        if self.pixels.contains(&Pixel::new(x, y)) {
            self.pixel_state
        } else {
            self.pixel_state.reverse()
        }
    }

    fn get_pixel_index(&self, x: i32, y: i32) -> usize {
        (y - 1..=y + 1)
            .flat_map(move |ny| {
                (x - 1..=x + 1)
                    .map(move |nx| self.get_pixel_state(nx, ny) as usize)
            })
            .fold(0, |acc, bit| acc * 2 + bit)
    }

    fn get_boundaries(&self) -> (Pixel, Pixel) {
        let min_x = self.pixels.iter().map(|pixel| pixel.x).min().unwrap_or(0);
        let max_x = self.pixels.iter().map(|pixel| pixel.x).max().unwrap_or(0);
        let min_y = self.pixels.iter().map(|pixel| pixel.y).min().unwrap_or(0);
        let max_y = self.pixels.iter().map(|pixel| pixel.y).max().unwrap_or(0);
        (
            Pixel::new(min_x - 1, min_y - 1),
            Pixel::new(max_x + 1, max_y + 1),
        )
    }

    fn enhance(&self, algo: &EnhanceAlgo) -> Self {
        let reverse = (self.pixel_state == Light
            && algo.pixel_state(0) == Light)
            || (self.pixel_state == Dark && algo.pixel_state(511) == Dark);

        let pixel_state = if reverse {
            self.pixel_state.reverse()
        } else {
            self.pixel_state
        };

        let (min, max) = self.get_boundaries();

        let pixels = (min.y..=max.y)
            .flat_map(move |y| {
                (min.x..=max.x).filter_map(move |x| {
                    if algo.pixel_state(self.get_pixel_index(x, y))
                        == pixel_state
                    {
                        Some(Pixel::new(x, y))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self {
            pixel_state,
            pixels,
        }
    }

    fn count_lit_pixels(&self) -> Option<usize> {
        if self.pixel_state == Light {
            Some(self.pixels.len())
        } else {
            None
        }
    }
}

pub fn part1(algo: &EnhanceAlgo, initial_image: &Image) -> Option<usize> {
    initial_image.enhance(algo).enhance(algo).count_lit_pixels()
}

pub fn part2(algo: &EnhanceAlgo, initial_image: &Image) -> Option<usize> {
    let mut image = initial_image.clone();
    for _ in 1..=50 {
        image = image.enhance(algo);
    }
    image.count_lit_pixels()
}

impl TryFrom<char> for PixelState {
    type Error = ();

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            '#' => Ok(Light),
            '.' => Ok(Dark),
            _ => Err(()),
        }
    }
}

impl FromStr for EnhanceAlgo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .chars()
            .map(|ch| {
                PixelState::try_from(ch).map_err(|_| {
                    format!("Invalid pixel '{}' in enhancement algorithm", ch)
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .and_then(|vec| {
                vec.try_into().map_err(|_| {
                    "Enhancement algorithm must have exactly 512 pixels"
                        .to_string()
                })
            })
            .map(Self)
    }
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .lines()
            .zip(0..)
            .flat_map(move |(line, y)| {
                line.trim().chars().zip(0..).map(move |(ch, x)| {
                    PixelState::try_from(ch)
                        .map_err(|_| format!("Invalid pixel '{}' in image", ch))
                        .map(|state| (state, Pixel::new(x, y)))
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|vec| {
                vec.into_iter()
                    .filter_map(|(state, pixel)| match state {
                        Light => Some(pixel),
                        Dark => None,
                    })
                    .collect::<BTreeSet<_>>()
            })
            .map(|pixels| Self::new(Light, pixels))
    }
}

impl Display for PixelState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Light => write!(f, "#"),
            Dark => write!(f, "."),
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.get_boundaries();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                write!(f, "{}", self.get_pixel_state(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
