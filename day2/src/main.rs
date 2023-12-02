use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Set {
    blue: u32,
    green: u32,
    red: u32,
}

struct Game {
    id: u32,
    set: Set,
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

fn calc_part_1(contents: &str) -> u32 {
    contents
        .lines()
        .map(|game| parse_game_part_1(game))
        .filter(|game| game.is_some())
        .map(|game| game.unwrap().id)
        .sum()
}

fn parse_game_part_1(game: &str) -> Option<Game> {
    let mut iter = game.split(":");

    let id = iter
        .next()
        .and_then(|g| g.split(" ").nth(1))
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let valid_record = iter
        .next()
        .unwrap()
        .split(";")
        .all(|set| !is_set_invalid(set));

    if valid_record {
        return Some(Game {
            id,
            set: Set {
                blue: 0,
                green: 0,
                red: 0,
            },
        });
    }

    None
}

fn is_set_invalid(set: &str) -> bool {
    set.split(",")
        .filter(|s| {
            let mut cubes = s.trim().split(" ");
            let number = cubes.next().unwrap().parse::<u32>().unwrap();
            let color = cubes.next().unwrap();
            color == "red" && number > 12
                || color == "green" && number > 13
                || color == "blue" && number > 14
        })
        .collect::<Vec<_>>()
        .len()
        > 0
}

fn calc_part_2(contents: &str) -> u32 {
    contents
        .lines()
        .map(|game| parse_game_part_2(game))
        .filter(|game| game.is_some())
        .map(|game| {
            let set = game.unwrap().set;
            set.blue * set.green * set.red
        })
        .sum()
}

fn parse_game_part_2(game: &str) -> Option<Game> {
    let mut iter = game.split(":");

    let id = iter
        .next()
        .and_then(|g| g.split(" ").nth(1))
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let records = iter
        .next()
        .unwrap()
        .split(";")
        .map(|set| parse_set(set))
        .collect::<Vec<_>>();

    let blue = records
        .iter()
        .max_by(|f, s| f.blue.cmp(&s.blue))
        .unwrap()
        .blue;
    let green = records
        .iter()
        .max_by(|f, s| f.green.cmp(&s.green))
        .unwrap()
        .green;
    let red = records.iter().max_by(|f, s| f.red.cmp(&s.red)).unwrap().red;
    Some(Game {
        id,
        set: Set { blue, green, red },
    })
}

fn parse_set(set: &str) -> Set {
    let a_set = set
        .split(",")
        .map(|s| {
            let mut cubes = s.trim().split(" ");
            let number = cubes.next().unwrap().parse::<u32>().unwrap();
            let color = cubes.next().unwrap();
            (color, number)
        })
        .collect::<Vec<_>>();

    match_color(a_set)
}

fn match_color(input: Vec<(&str, u32)>) -> Set {
    let mut output = Set {
        blue: 0,
        green: 0,
        red: 0,
    };
    for e in input {
        if e.0 == "red" {
            output.red = e.1;
        } else if e.0 == "blue" {
            output.blue = e.1;
        } else {
            output.green = e.1;
        }
    }
    output
}

fn run(file_path: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let part_1 = calc_part_1(&contents);

    let part_2 = calc_part_2(&contents);

    Ok((part_1, part_2))
}
