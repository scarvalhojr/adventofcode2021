use clap::{crate_description, App, Arg};
use day21::{part1, part2};
use std::fs::read_to_string;
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

    let (player1, player2) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    println!("Part 1: {}", part1(player1, player2));
    println!("Part 2: {}", part2(player1, player2));
}

fn read_input(filename: &str) -> Result<(u64, u64), String> {
    read_to_string(filename)
        .map_err(|err| err.to_string())?
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_once(':')
                .ok_or_else(|| format!("Invalid input line '{}'", line))
                .and_then(|(_, num)| {
                    num.trim()
                        .parse()
                        .map_err(|_| format!("Invalid number '{}'", num))
                })
        })
        .collect::<Result<Vec<u64>, _>>()
        .and_then(|vec| {
            vec.try_into()
                .map_err(|_| "Input must have exactly two lines".to_string())
        })
        .map(|numbers: [u64; 2]| (numbers[0], numbers[1]))
}
