use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug, Clone, Copy)]
struct Range {
    begin: u64,
    end: u64,
}

struct Line {
    destination: u64,
    range: Range,
}

struct Map {
    lines: Vec<Line>,
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
    let seeds = parse_input(&contents);

    let maps = (1..8)
        .map(|i| parse_lines(&contents, i))
        .collect::<Vec<_>>();

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

fn jump_forward(key: u64, map: &Vec<Line>) -> u64 {
    for entry in map {
        if key >= entry.range.begin && key < entry.range.end {
            return entry.destination + key - entry.range.begin;
        }
    }
    key
}

fn calc_part_2(contents: &str) -> u64 {
    let seeds = parse_seeds_part_2(&contents);
    let maps = (1..8)
        .map(|i| {
            let mut lines = parse_lines(&contents, i);
            lines.sort_by(|first, second| first.range.begin.cmp(&second.range.begin));
            Map { lines }
        })
        .collect::<Vec<_>>();

    let min_location = seeds
        .iter()
        .map(|seed| {
            let mut ranges = vec![*seed];
            for map in maps.iter() {
                let mut sub_ranges = vec![];
                for range in ranges.iter() {
                    sub_ranges.push(map_range(range, map));
                }
                ranges = sub_ranges.into_iter().flatten().collect::<Vec<Range>>();
            }
            ranges
        })
        .flatten()
        .min_by(|first, second| {
            first.begin.cmp(&second.begin)
        })
        .unwrap();

    min_location.begin

}

fn map_range(range: &Range, map: &Map) -> Vec<Range> {
    let mut points = map
        .lines
        .iter()
        .map(|line| line.range.begin)
        .collect::<HashSet<_>>();
    let last_point = map.lines.last().unwrap();
    points.insert(last_point.range.end);
    points.insert(range.begin);
    points.insert(range.end);

    let mut valid_points = points
        .iter()
        .filter(|p| *p >= &range.begin && *p <= &range.end)
        .collect::<Vec<_>>();
    if valid_points.is_empty() {
        return vec![*range];
    }
    valid_points.sort();
    let ranges = valid_points
        .windows(2)
        .map(|r| Range {
            begin: *r[0],
            end: *r[1],
        })
        .collect::<Vec<_>>();
    let mut sub_ranges = vec![];
    let mut already_mapped = vec![];
    for line in map.lines.iter() {
        for (i, range) in ranges.iter().enumerate() {
            if range.begin >= line.range.begin && range.end <= line.range.end {
                sub_ranges.push(Range {
                    begin: line.destination + range.begin - line.range.begin,
                    end: line.destination + range.end - line.range.begin,
                });
                already_mapped.push(i);
            } 
        }
    }
    
    ranges.iter().enumerate().filter(|(i, _)| {
        !already_mapped.contains(i)
    }).for_each(|(_, r)| {
        sub_ranges.push(*r);
    });

    sub_ranges
}

fn parse_seeds_part_2(contents: &str) -> Vec<Range> {
    parse_input(contents)
        .chunks(2)
        .map(|v| Range {
            begin: v[0],
            end: v[0] + v[1],
        })
        .collect::<Vec<_>>()
}

fn parse_input(contents: &str) -> Vec<u64> {
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

fn parse_lines(contents: &str, position: usize) -> Vec<Line> {
    contents
        .split("\n\n")
        .nth(position)
        .unwrap()
        .split(":\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            let parsed_line = line
                .split(" ")
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            Line {
                destination: parsed_line[0],
                range: Range {
                    begin: parsed_line[1],
                    end: parsed_line[1] + parsed_line[2],
                },
            }
        })
        .collect::<Vec<_>>()
}
