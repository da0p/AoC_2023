use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
struct Net {
    left: String,
    right: String,
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
    let (_, instructions, map) = parse_input(&contents);
    let start_node = String::from("AAA");

    let steps = navigate(start_node, &instructions, &map, |node| node == "ZZZ");

    steps
}

fn calc_part_2(contents: &str) -> u64 {
    let (start_nodes, instructions, map) = parse_input(&contents);

    // It's seriously a damn loop after the first round! If it's different, then it's hell!
    let rounds = start_nodes
        .iter()
        .map(|node| {
            navigate(node.to_owned(), &instructions, &map, |node| {
                node.ends_with("Z")
            })
        })
        .collect::<Vec<_>>();

    let steps = rounds.iter().fold(1, |x, y| lcm(x, *y));

    steps
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a.max(b);
    let mut y = a.min(b);
    loop {
        let r = x % y;
        if r == 0 {
            break;
        }
        x = y;
        y = r;
    }
    y
}

fn navigate(
    first_node: String,
    instructions: &Vec<char>,
    map: &HashMap<String, Net>,
    ending_fn: fn(node: &str) -> bool,
) -> u64 {
    let mut node = first_node;
    let mut steps: u64 = 0;

    loop {
        if ending_fn(&node) {
            break;
        }
        let index = steps as usize % instructions.len();
        let instruction = instructions[index];
        steps += 1;
        node = jump_one_step(&node, instruction, map);
    }

    steps
}

fn jump_one_step<'a>(
    current_node: &'a str,
    instruction: char,
    map: &'a HashMap<String, Net>,
) -> String {
    match instruction {
        'R' => map.get(current_node).unwrap().right.to_owned(),
        _ => map.get(current_node).unwrap().left.to_owned(),
    }
}

fn parse_input(contents: &str) -> (Vec<String>, Vec<char>, HashMap<String, Net>) {
    let mut iter = contents.split("\n\n");

    let instructions = iter.next().unwrap().chars().collect::<Vec<_>>();

    let map = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split(" = ");
            let key = iter.next().unwrap();
            let net = iter.next().unwrap();
            let left = &net[1..4];
            let right = &net[6..9];
            (
                key.to_owned(),
                Net {
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            )
        })
        .collect::<HashMap<String, Net>>();

    let iter = contents.split("\n\n");

    let nodes_ending_with_a = iter
        .skip(1)
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(" = ").next().unwrap())
        .filter_map(|line| {
            if line.ends_with("A") {
                Some(line.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    (nodes_ending_with_a, instructions, map)
}
