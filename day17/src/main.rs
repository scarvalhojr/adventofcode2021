use clap::{crate_description, App, Arg};
use day17::{part1, part2, Target};
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

    match part1(&input) {
        Some(answer) => println!("Part 1: {}", &answer),
        None => println!("Part 1: Not found"),
    }
    println!("Part 2: {}", part2(&input));
}

fn read_input(filename: &str) -> Result<Target, String> {
    read_to_string(filename)
        .map_err(|err| err.to_string())
        .and_then(|s| s.parse())
}
