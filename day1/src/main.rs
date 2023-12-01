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

fn calc_sum_part_1(contents: &str) -> u32 {
    contents
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .flat_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            
            digits.first().unwrap() * 10 + digits.last().unwrap()
            }
        )
        .sum()
}

fn calc_sum_part_2(contents: &str) -> u32 {
    contents
        .lines()
        .map(|line| {
            parse_line_part_2(line)
        })
        .sum()
}

fn parse_line_part_2(line: &str) -> u32 {
    let first = 
            line.chars()
                .enumerate()
                .filter_map(|e| {
                    if e.1.is_ascii_digit() {
                        e.1.to_digit(10)
                    } else {
                        match &line[e.0..] {
                            one if one.starts_with("one") => Some(1),
                            two if two.starts_with("two") => Some(2),
                            three if three.starts_with("three") => Some(3),
                            four if four.starts_with("four") => Some(4),
                            five if five.starts_with("five") => Some(5),
                            six if six.starts_with("six") => Some(6),
                            seven if seven.starts_with("seven") => Some(7),
                            eight if eight.starts_with("eight") => Some(8),
                            nine if nine.starts_with("nine") => Some(9),
                            _ => None,
                        }
                    }
                }
            )
            .take(1)
            .next()
            .unwrap();

    let rev_line = line.chars().rev().collect::<String>();
    let last = 
            rev_line.chars()
                .enumerate()
                .filter_map(|e| {
                    if e.1.is_ascii_digit() {
                        e.1.to_digit(10)
                    } else {
                        match &rev_line[e.0..] {
                            one if one.starts_with("eno") => Some(1),
                            two if two.starts_with("owt") => Some(2),
                            three if three.starts_with("eerht") => Some(3),
                            four if four.starts_with("ruof") => Some(4),
                            five if five.starts_with("evif") => Some(5),
                            six if six.starts_with("xis") => Some(6),
                            seven if seven.starts_with("neves") => Some(7),
                            eight if eight.starts_with("thgie") => Some(8),
                            nine if nine.starts_with("enin") => Some(9),
                            _ => None,
                        }
                    }
                }
            )
            .take(1)
            .next()
            .unwrap();

    first * 10 + last
}

fn run(file_path: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let part_1 = calc_sum_part_1(&contents);

    let part_2 = calc_sum_part_2(&contents);

    Ok((part_1, part_2))
}
