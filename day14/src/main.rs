use clap::{crate_description, App, Arg};
use day14::{part1, part2, InsertionRule, Polymer, RuleMap};
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

    let (template, rules) = match read_input(args.value_of("INPUT").unwrap()) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(2);
        }
    };

    match part1(&template, &rules) {
        Some(answer) => println!("Part 1: {}", &answer),
        None => println!("Part 1: Not found"),
    }
    match part2(&template, &rules) {
        Some(answer) => println!("Part 2: {}", &answer),
        None => println!("Part 2: Not found"),
    }
}

fn read_input(filename: &str) -> Result<(Polymer, RuleMap), String> {
    let contents = read_to_string(filename).map_err(|err| err.to_string())?;
    let lines = contents.lines().zip(1..).collect::<Vec<_>>();
    let mut blocks = lines.as_slice().split(|(line, _)| line.trim().is_empty());

    let template = blocks
        .next()
        .and_then(|block| block.iter().next())
        .ok_or_else(|| "Missing template line".to_string())
        .and_then(|(line, _)| line.parse())?;

    let rule_map = blocks
        .next()
        .ok_or_else(|| "Missing rules".to_string())?
        .iter()
        .map(|(line, line_num)| {
            line.parse()
                .map_err(|err| format!("Line {}: {}", line_num, err))
        })
        .collect::<Result<Vec<InsertionRule>, _>>()
        .map(RuleMap::new)?;

    Ok((template, rule_map))
}
