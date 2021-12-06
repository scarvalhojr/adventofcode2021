use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub const BOARD_SIZE: usize = 5;

#[derive(Clone)]
pub struct Board {
    numbers: HashMap<i32, (usize, usize)>,
    row_marks: Vec<usize>,
    col_marks: Vec<usize>,
    complete: bool,
}

impl Board {
    fn mark(&mut self, number: i32) {
        if let Some((row, col)) = self.numbers.remove(&number) {
            self.row_marks[row] += 1;
            self.col_marks[col] += 1;
            if self.row_marks[row] == BOARD_SIZE
                || self.col_marks[col] == BOARD_SIZE
            {
                self.complete = true;
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    fn sum_remaining(&self) -> i32 {
        self.numbers.keys().sum()
    }
}

pub fn part1(numbers: &[i32], boards: &[Board]) -> Option<i32> {
    let mut boards = boards.to_vec();
    for &number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.is_complete() {
                return Some(number * board.sum_remaining());
            }
        }
    }
    None
}

pub fn part2(numbers: &[i32], boards: &[Board]) -> Option<i32> {
    let mut boards = boards.to_vec();
    let mut last_win = None;
    for &number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.is_complete() {
                last_win = Some(number * board.sum_remaining())
            }
        }
        boards.retain(|board| !board.is_complete());
    }
    last_win
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: HashMap<i32, (usize, usize)> = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.split_whitespace().enumerate().map(move |(col, num)| {
                    num.parse::<i32>()
                        .map(|number| (number, (row, col)))
                        .map_err(|err: ParseIntError| err.to_string())
                })
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect();

        if numbers.len() != BOARD_SIZE * BOARD_SIZE
            || numbers
                .values()
                .any(|(r, c)| *r >= BOARD_SIZE || *c >= BOARD_SIZE)
        {
            return Err(format!("Invalid board dimension: {}", s));
        }

        let row_marks = vec![0; BOARD_SIZE];
        let col_marks = vec![0; BOARD_SIZE];
        let complete = false;

        Ok(Board {
            numbers,
            row_marks,
            col_marks,
            complete,
        })
    }
}
