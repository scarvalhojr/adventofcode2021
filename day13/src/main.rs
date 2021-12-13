use clap::{crate_description, App, Arg};
use day13::{part1, part2, Dot, Fold};
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

    let (dots, folds) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    println!("Part 1: {}", part1(&dots, &folds));
    println!("Part 2:");
    part2(&dots, &folds);
}

fn read_input(filename: &str) -> Result<(Vec<Dot>, Vec<Fold>), String> {
    let contents = read_to_string(filename).map_err(|err| err.to_string())?;
    let lines = contents.lines().zip(1..).collect::<Vec<_>>();
    let mut blocks = lines.as_slice().split(|(line, _)| line.trim().is_empty());

    let dots = blocks
        .next()
        .ok_or_else(|| "Missing dot lines".to_string())?
        .iter()
        .map(|(line, line_num)| {
            line.parse()
                .map_err(|err| format!("Line {}: {}", line_num, err))
        })
        .collect::<Result<_, _>>()?;

    let folds: Vec<Fold> = blocks
        .next()
        .ok_or_else(|| "Missing fold lines".to_string())?
        .iter()
        .map(|(line, line_num)| {
            line.parse()
                .map_err(|err| format!("Line {}: {}", line_num, err))
        })
        .collect::<Result<_, _>>()?;

    Ok((dots, folds))
}
