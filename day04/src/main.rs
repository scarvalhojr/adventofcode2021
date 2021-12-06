use clap::{crate_description, App, Arg};
use day04::{part1, part2, Board, BOARD_SIZE};
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::process::exit;

fn main() {
    let args = App::new(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .help("File with puzzle input")
                .required(true)
                .index(1),
        )
        .get_matches();

    println!(crate_description!());

    let (numbers, boards) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    match part1(&numbers, &boards) {
        Some(answer) => println!("Part 1: {}", &answer),
        None => println!("Part 1: Not found"),
    }
    match part2(&numbers, &boards) {
        Some(answer) => println!("Part 2: {}", &answer),
        None => println!("Part 2: Not found"),
    }
}

fn read_input(filename: &str) -> Result<(Vec<i32>, Vec<Board>), String> {
    let contents = read_to_string(filename).map_err(|err| err.to_string())?;
    let mut lines = contents.lines().collect::<Vec<_>>();
    if lines.is_empty() {
        return Err("Empty file".to_string());
    }

    let remaining = lines.split_off(1);

    let numbers = lines
        .pop()
        .unwrap()
        .split(',')
        .map(|num| num.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err: ParseIntError| err.to_string())?;

    let boards = remaining
        .chunks(BOARD_SIZE + 1)
        .map(|chunk| chunk.join("\n").parse::<Board>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok((numbers, boards))
}
