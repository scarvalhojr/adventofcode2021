use clap::{crate_description, App, Arg};
use day03::{part1, part2};
use std::fs::File;
use std::io::{BufRead, BufReader};
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

    let input = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    println!("Part 1: {}", part1(&input));
    match part2(&input) {
        Some(answer) => println!("Part 2: {}", &answer),
        None => println!("Part 2: Not found"),
    }
}

fn read_input(filename: &str) -> Result<Vec<u16>, String> {
    let input_file = File::open(filename).map_err(|err| err.to_string())?;

    BufReader::new(input_file)
        .lines()
        .zip(1..)
        .map(|(line, line_num)| {
            line.map_err(|err| (line_num, err.to_string()))
                .and_then(|bits| {
                    u16::from_str_radix(&bits, 2).map_err(
                        |err: ParseIntError| (line_num, err.to_string()),
                    )
                })
        })
        .collect::<Result<_, _>>()
        .map_err(|(line_num, err)| format!("Line {}: {}", line_num, err))
}
