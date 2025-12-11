use std::ops::Range;

fn main() {
    let (ranges, values) = parse_input(INPUT);

    let fresh_count = values
        .iter()
        .filter(|v| ranges.iter().any(|range| range.contains(v)))
        .count();

    println!("fresh count: {fresh_count}");
}

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> (Vec<Range<u64>>, Vec<u64>) {
    let mut lines = input.trim_end().lines();
    let mut ranges = vec![];

    let mut values = vec![];

    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let (start, end) = line.split_once("-").unwrap();

        let parsed_start: u64 = start.parse().unwrap();

        let parsed_end: u64 = end.parse().unwrap();

        ranges.push(parsed_start..parsed_end + 1);
    }

    for line in lines {
        let parsed: u64 = line.parse().unwrap();
        values.push(parsed);
    }
    (ranges, values)
}
