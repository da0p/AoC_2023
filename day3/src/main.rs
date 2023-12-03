use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    start_coor: (usize, usize),
    end_coor: (usize, usize),
}

#[derive(Debug)]
struct Symbol {
    value: String,
    coor: (usize, usize),
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

fn run(file_path: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let (part_1, part_2) = calc(&contents);

    Ok((part_1, part_2))
}

fn calc(contents: &str) -> (u32, u32) {
    let (symbols_coors, numbers_coors) = parse_input(contents);
    let mut valid_numbers = vec![];
    let mut gear_ratios = vec![];

    for (i, symbols) in symbols_coors.iter() {
        for symbol in symbols {
            scan_neighbors(&symbol, &numbers_coors, &mut valid_numbers, &mut gear_ratios);
        }
    }

    let part_1 = valid_numbers.iter().map(|n| n.value).sum();
    let part_2 = gear_ratios.iter().sum();

    (part_1, part_2)
}


fn scan_neighbors(
    s: &Symbol,
    numbers_coors: &HashMap<usize, Vec<Number>>,
    valid_numbers: &mut Vec<Number>,
    gear_ratios: &mut Vec<u32>,
) {
    let mut gear_ratio = vec![];
    let min_row = if s.coor.0 > 0 { s.coor.0 - 1 } else { s.coor.0 };
    let max_row = s.coor.0 + 2;
    for search_row in min_row..max_row {
        let numbers = numbers_coors.get(&search_row);
        if numbers.is_some() {
            for number in numbers.unwrap() {
                let min_col = if number.start_coor.1 > 0 {
                    number.start_coor.1 - 1
                } else {
                    number.start_coor.1
                };
                let max_col = number.end_coor.1;
                if s.coor.1 >= min_col && s.coor.1 <= max_col {
                    valid_numbers.push(number.clone());
                    if s.value == "*" {
                        gear_ratio.push(number.value);
                        if gear_ratio.len() == 2 {
                            gear_ratios.push(gear_ratio[0] * gear_ratio[1]);
                        }
                    }
                }
            }
        }
    }
}

fn parse_input(contents: &str) -> (HashMap<usize, Vec<Symbol>>, HashMap<usize, Vec<Number>>) {
    let number_regex = Regex::new(r"\D+").unwrap();
    let numbers = contents
        .lines()
        .enumerate()
        .map(|line| {
            let parsed = split_with_regex(&number_regex, line.0, line.1)
                .filter_map(|e| {
                    if e.1.is_empty() {
                        return None;
                    }
                    let no = e.1.parse::<u32>();
                    if no.is_ok() {
                        let start_coor = (e.0 .0, e.0 .1);
                        let end_coor = (e.0 .0, e.0 .1 + e.1.len());
                        return Some(Number {
                            value: no.unwrap(),
                            start_coor,
                            end_coor,
                        });
                    }
                    None
                })
                .collect::<Vec<_>>();
            (line.0, parsed)
        })
        .filter(|e| !e.1.is_empty())
        .collect::<HashMap<usize, Vec<_>>>();

    let symbol_regex = Regex::new(r"\.|\d+").unwrap();
    let symbols = contents
        .lines()
        .enumerate()
        .map(|line| {
            let parsed = split_with_regex(&symbol_regex, line.0, line.1)
                .filter_map(|e| {
                    if e.1.is_empty() {
                        return None;
                    }
                    Some(Symbol {
                        value: e.1.to_owned(),
                        coor: (e.0 .0, e.0 .1),
                    })
                })
                .collect::<Vec<_>>();
            (line.0, parsed)
        })
        .filter(|e| !e.1.is_empty())
        .collect::<HashMap<usize, Vec<_>>>();

    (symbols, numbers)
}

fn addr_of(s: &str) -> usize {
    s.as_ptr() as usize
}

fn split_with_regex<'a>(
    pattern: &'a Regex,
    line_coor: usize,
    s: &'a str,
) -> impl Iterator<Item = ((usize, usize), &'a str)> {
    pattern
        .split(s)
        .map(move |sub| ((line_coor, addr_of(sub) - addr_of(s)), sub))
}
