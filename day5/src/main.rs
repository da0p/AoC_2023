use std::env;
use std::error::Error;
use std::fs;
use std::process;

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
    let seeds = parse_seeds_part_1(&contents);
    let mut maps = vec![];
    for i in 1..8 {
        maps.push(parse_map(&contents, i));
    }

    seeds
        .iter()
        .map(|seed| {
            let mut key = *seed;
            for map in maps.iter() {
                key = jump_forward(key, &map);
            }

            key
        })
        .min()
        .unwrap()
}

fn parse_seeds_part_1(contents: &str) -> Vec<u64> {
    contents
        .split("\n\n")
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|n| !n.is_empty())
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn jump_forward(key: u64, map: &Vec<Vec<u64>>) -> u64 {
    for entry in map {
        if key >= entry[1] && key - entry[1] < entry[2] {
            return entry[0] + key - entry[1];
        }
    }

    key
}

fn calc_part_2(contents: &str) -> u64 {
    0
}

fn parse_seeds_part_2(contents: &str) -> Vec<(u64, u64)> {
    parse_seeds_part_1(contents)
        .chunks(2)
        .map(|v| (v[0], v[0] + v[1]))
        .collect::<Vec<_>>()
}

fn parse_map(contents: &str, position: usize) -> Vec<Vec<u64>> {
    contents
        .split("\n\n")
        .nth(position)
        .unwrap()
        .split(":\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>()
}
