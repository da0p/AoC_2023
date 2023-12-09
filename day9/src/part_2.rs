use crate::utils;

pub fn calc_part_2(contents: &str) -> i64 {
    let numbers = utils::parse_input(&contents);
    numbers
        .iter()
        .map(|line| {
            let mut curr_numbers = line.clone();
            let mut first_numbers = vec![];
            while !curr_numbers.iter().all(|v| *v == 0) {
                first_numbers.push(curr_numbers[0]);
                curr_numbers = curr_numbers
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<_>>();
            }

            first_numbers.iter().rev().fold(0, |acc, x| x - acc)
        })
        .sum()
}
