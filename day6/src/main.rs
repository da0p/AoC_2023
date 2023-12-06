use std::env;
use std::error::Error;
use std::ffi::FromVecWithNulError;
use std::fs;
use std::iter::zip;
use std::process;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    let digits = read_input(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let total = run(&digits).unwrap_or_else(|err| {
        println!("Calculation error: {err}");
        process::exit(1);
    });

    println!("total = {:#?}", total);
}

fn read_input(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();

    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path"),
    };

    Ok(file_path)
}

fn run(file_path: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let part_1 = calc_part_1(&contents);

    let part_2 = calc_part_2(&contents);

    Ok((part_1, part_2))
}

fn calc_part_1(contents: &str) -> u64 {
    let races = parse_input_part_1(&contents);
    let mut products = vec![];

    for race in races.iter() {
        products.push(race.time + 1 - 2 * calc_loses(race));
    }

    products.iter().fold(1, |first, second| first * second)
}

fn parse_input_part_1(contents: &str) -> Vec<Race> {
    let mut lines = contents.lines();
    let time = parse_line_part_1(lines.next().unwrap());
    let distance = parse_line_part_1(lines.next().unwrap());

    zip(time, distance)
        .map(|r| Race {
            time: r.0,
            distance: r.1,
        })
        .collect::<Vec<Race>>()
}

fn parse_line_part_1(line: &str) -> Vec<u64> {
    line.split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|t| !t.is_empty())
        .map(|t| t.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn calc_part_2(contents: &str) -> u64 {
    let race = parse_input_part_2(&contents);

    race.time + 1 - calc_loses(&race) * 2
}

fn parse_input_part_2(contents: &str) -> Race {
    let mut lines = contents.lines();
    let time = parse_line_part_2(lines.next().unwrap());

    let distance = parse_line_part_2(lines.next().unwrap());

    Race { time, distance }
}

fn parse_line_part_2(line: &str) -> u64 {
    line.split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn calc_loses(race: &Race) -> u64 {
    (0..race.time / 2 + 1)
        .filter(|velocity| velocity * (race.time - velocity) <= race.distance)
        .count() as u64
}
