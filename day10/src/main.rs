use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
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

fn run(file_path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let part_1 = calc_part_1(&contents);

    let part_2 = calc_part_2(&contents);

    Ok((part_1, part_2))
}

fn calc_part_1(contents: &str) -> usize {
    let map = parse_input(&contents);
    find_boundary(&map).len() / 2
}

fn calc_part_2(contents: &str) -> usize {
    let map = parse_input(&contents);
    let boundary = find_boundary(&map);
    let tiles = collect_tiles(&map, &boundary);

    tiles
        .iter()
        .filter(|tile| is_tile_inside(tile, &boundary))
        .count() as usize
}

fn is_tile_inside(tile: &Point, boundary: &Vec<Point>) -> bool {
    // ray-casting algorithm
    let mut inside = false;
    for i in 0..boundary.len() {
        let j = (i + 1) % boundary.len();
        let x_i = boundary[i].x as i64;
        let y_i = boundary[i].y as i64;
        let x_j = boundary[j].x as i64;
        let y_j = boundary[j].y as i64;

        let intersect = ((x_i > (tile.x as i64)) != (x_j > (tile.x as i64)))
            && ((tile.y as i64) < ((tile.x as i64) - x_i) * (y_j - y_i) / (x_j - x_i) + y_i);
        if intersect {
            inside = !inside;
        }
    }
    inside
}

fn collect_tiles(map: &Vec<Vec<char>>, boundary: &Vec<Point>) -> Vec<Point> {
    let mut tiles = vec![];
    for (x, x_c) in map.iter().enumerate() {
        for (y, y_c) in x_c.iter().enumerate() {
            let check_point = Point { x, y };
            if !boundary.contains(&check_point) {
                tiles.push(Point { x, y });
            }
        }
    }
    tiles
}

fn find_boundary(map: &Vec<Vec<char>>) -> Vec<Point> {
    let mut boundary = vec![];
    let mut prev = find_starting_point(&map);
    let mut cur = first_step(&prev, &map);
    boundary.push(prev.clone());
    boundary.push(cur.clone());

    while map[cur.x][cur.y] != 'S' {
        let next = next_step(&cur, &prev, &map);
        boundary.push(next.clone());
        prev = cur;
        cur = next;
    }
    boundary
}

fn next_step(cur: &Point, prev: &Point, map: &Vec<Vec<char>>) -> Point {
    match map[cur.x][cur.y] {
        '-' => {
            if prev.x == cur.x && prev.y == cur.y - 1 {
                return Point {
                    x: cur.x,
                    y: cur.y + 1,
                };
            }
            Point {
                x: cur.x,
                y: cur.y - 1,
            }
        }
        '|' => {
            if prev.x == cur.x - 1 && prev.y == cur.y {
                return Point {
                    x: cur.x + 1,
                    y: cur.y,
                };
            }
            Point {
                x: cur.x - 1,
                y: cur.y,
            }
        }
        'J' => {
            if prev.x == cur.x && prev.y == cur.y - 1 {
                return Point {
                    x: cur.x - 1,
                    y: cur.y,
                };
            }
            Point {
                x: cur.x,
                y: cur.y - 1,
            }
        }
        'L' => {
            if prev.x == cur.x && prev.y == cur.y + 1 {
                return Point {
                    x: cur.x - 1,
                    y: cur.y,
                };
            }
            Point {
                x: cur.x,
                y: cur.y + 1,
            }
        }
        'F' => {
            if prev.x == cur.x + 1 && prev.y == cur.y {
                return Point {
                    x: cur.x,
                    y: cur.y + 1,
                };
            }
            Point {
                x: cur.x + 1,
                y: cur.y,
            }
        }
        '7' => {
            if prev.x == cur.x && prev.y == cur.y - 1 {
                return Point {
                    x: cur.x + 1,
                    y: cur.y,
                };
            }
            Point {
                x: cur.x,
                y: cur.y - 1,
            }
        }
        _ => unreachable!(),
    }
}

fn first_step(start: &Point, map: &Vec<Vec<char>>) -> Point {
    let max_row = map.len();
    let max_col = map[0].len();
    if start.y + 1 < max_col && ['-', 'J', '7'].contains(&map[start.x][start.y + 1]) {
        return Point {
            x: start.x,
            y: start.y + 1,
        };
    } else if start.x > 0 && ['|', '7', 'F'].contains(&map[start.x - 1][start.y]) {
        return Point {
            x: start.x - 1,
            y: start.y,
        };
    } else if start.x > 0 && ['-', 'L', 'F'].contains(&map[start.x][start.y - 1]) {
        return Point {
            x: start.x,
            y: start.y - 1,
        };
    } else if start.x + 1 < max_row && ['|', 'L', 'J'].contains(&map[start.x + 1][start.y]) {
        return Point {
            x: start.x + 1,
            y: start.y,
        };
    } else {
        unreachable!()
    }
}

fn find_starting_point(map: &Vec<Vec<char>>) -> Point {
    let mut s_coor = Point { x: 0, y: 0 };
    for (x, x_c) in map.iter().enumerate() {
        for (y, y_c) in x_c.iter().enumerate() {
            if *y_c == 'S' {
                s_coor = Point { x, y };
            }
        }
    }
    s_coor
}

fn parse_input(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.chars().map(|c| c).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_correct_starting_point() {
        let contents = String::from("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...");
        let map = parse_input(&contents);
        assert_eq!(find_starting_point(&map), Point { x: 2, y: 0 });
    }

    #[test]
    fn find_first_step() {
        let contents = String::from("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...");
        let map = parse_input(&contents);
        let start = find_starting_point(&map);
        assert_eq!(first_step(&start, &map), Point { x: 2, y: 1 });
    }

    #[test]
    fn find_further_point() {
        let contents = String::from("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...");
        assert_eq!(calc_part_1(&contents), 8);
    }

    #[test]
    fn test_tile_outside() {
        let contents = String::from("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...");
        let map = parse_input(&contents);
        let boundary = find_boundary(&map);
        assert!(!is_tile_inside(&Point { x: 3, y: 3 }, &boundary));
    }

    #[test]
    fn test_tile_inside() {
        let contents = String::from("...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........");
        let map = parse_input(&contents);
        let boundary = find_boundary(&map);
        assert!(is_tile_inside(&Point { x: 6, y: 2 }, &boundary));
    }
}
