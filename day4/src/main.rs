use std::collections::HashMap;
use std::collections::HashSet;
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

fn run(file_path: &str) -> Result<(u64, u128), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let part_1 = calc_winning_points(&contents);

    let part_2 = calc_part_2(&contents);

    Ok((part_1, part_2))
}

fn calc_winning_points(contents: &str) -> u64 {
    contents
        .lines()
        .map(|card| calc_winning_points_for_one_card(card))
        .sum()
}

fn calc_winning_points_for_one_card(card: &str) -> u64 {
    let no_of_winnings = number_of_matching(card);
    if no_of_winnings > 0 {
        u64::pow(2, no_of_winnings as u32 - 1)
    } else {
        0
    }
}

fn number_of_matching(card: &str) -> usize {
    let mut iter = card.split(":").last().unwrap().split("|");

    let winnings = iter
        .next()
        .unwrap()
        .split(" ")
        .filter(|d| !d.is_empty())
        .map(|d| d.parse::<u64>().unwrap())
        .collect::<HashSet<u64>>();

    let havings = iter
        .next()
        .unwrap()
        .split(" ")
        .filter(|d| !d.is_empty())
        .map(|d| d.parse::<u64>().unwrap())
        .collect::<HashSet<u64>>();

    winnings.intersection(&havings).collect::<Vec<_>>().len()
}

fn calc_part_2(contents: &str) -> u128 {
    let mut total = HashMap::<usize, u128>::new();
    let max_cards = contents.lines().collect::<Vec<_>>().len();

    for (i, card) in contents.lines().enumerate() {
        let num_cards_in_mem = total.entry(i).or_insert(0);
        *num_cards_in_mem += 1;
        let n = number_of_matching(card);
        let max_size = if i + 1 + n > max_cards {
            max_cards
        } else {
            i + 1 + n
        };

        let multiple = *total.get(&i).unwrap();
        for j in i + 1..max_size {
            let total_of_a_card = total.entry(j).or_insert(0);
            *total_of_a_card += multiple; 
        }
    }

    total.iter().map(|(k, v)| v).sum()
}
