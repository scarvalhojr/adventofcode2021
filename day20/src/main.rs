use clap::{crate_description, App, Arg};
use day20::{part1, part2, EnhanceAlgo, Image};
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

    let (algo, image) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    match part1(&algo, &image) {
        Some(answer) => println!("Part 1: {}", &answer),
        None => println!("Part 1: Not found"),
    }
    match part2(&algo, &image) {
        Some(answer) => println!("Part 2: {}", &answer),
        None => println!("Part 2: Not found"),
    }
}

fn read_input(filename: &str) -> Result<(EnhanceAlgo, Image), String> {
    let contents = read_to_string(filename).map_err(|err| err.to_string())?;
    let mut blocks = contents.split("\n\n");

    let algo = blocks
        .next()
        .ok_or_else(|| "Missing algorithm line".to_string())
        .and_then(|line| line.parse())?;

    let image = blocks
        .next()
        .ok_or_else(|| "Missing image".to_string())
        .and_then(|line| line.parse())?;

    Ok((algo, image))
}
