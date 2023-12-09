use crate::utils;

pub fn calc_part_1(contents: &str) -> i64 {
    let numbers = utils::parse_input(&contents);

    numbers
        .iter()
        .map(|line| {
            let mut curr_numbers = line.clone();
            let mut next_value = 0;
            while !curr_numbers.iter().all(|v| *v == 0) {
                next_value += *curr_numbers.iter().last().unwrap();
                curr_numbers = curr_numbers
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<_>>();
            }
            next_value
        })
        .sum()
}
