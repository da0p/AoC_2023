use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub mod part_1;
pub mod part_2;
pub mod utils;

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

fn run(file_path: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let part_1 = part_1::calc_part_1(&contents);

    let part_2 = part_2::calc_part_2(&contents);

    Ok((part_1, part_2))
}

fn read_input(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();

    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file path"),
    };

    Ok(file_path)
}
