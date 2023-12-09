pub fn parse_input(contents: &str) -> Vec<Vec<i64>> {
    contents
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|d| d.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>()
}
