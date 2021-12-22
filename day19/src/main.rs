use clap::{crate_description, App, Arg};
use day19::{solve, Scanner};
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

    let input = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    match solve(&input) {
        Some((part1, part2)) => {
            println!("Part 1: {}\nPart 2: {}", part1, part2)
        }
        None => println!("Part 1: Not found\nPart 2: Not found"),
    }
}

fn read_input(filename: &str) -> Result<Vec<Scanner>, String> {
    read_to_string(filename)
        .map_err(|err| err.to_string())?
        .split("\n\n")
        .map(|scanner| scanner.parse())
        .collect::<Result<Vec<_>, _>>()
}
