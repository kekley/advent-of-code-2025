use std::{cmp::Ordering, ops::Range, vec};

fn main() {
    let (mut ranges, values) = parse_input(INPUT);

    let fresh_count = values
        .iter()
        .filter(|v| ranges.iter().any(|range| range.contains(v)))
        .count();

    println!("fresh count: {fresh_count}");
    ranges.sort_by(compare_ranges);

    let mut ranges_iter = ranges.into_iter().peekable();
    let mut merged = vec![];

    while let Some(mut test_range) = ranges_iter.by_ref().next() {
        println!("Starting range: {test_range:?}");

        while let Some(next) = ranges_iter.peek() {
            if next.start <= test_range.end {
                println!("checking range: {next:?}");
                if next.end > test_range.end {
                    test_range.end = next.end;
                    println!("merged range: {test_range:?}");
                } else {
                    println!("range contained in starting range");
                }
                let _ = ranges_iter.next();
            } else {
                break;
            }
        }

        merged.push(test_range.clone());
    }
    let total: usize = merged.iter().map(|r| r.clone().count()).sum();
    println!("total: {total}");
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

fn compare_ranges(a: &Range<u64>, b: &Range<u64>) -> Ordering {
    if a.start < b.start {
        Ordering::Less
    } else if a.start == b.start {
        if a.end < b.end {
            Ordering::Less
        } else if a.end == b.end {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    } else {
        Ordering::Greater
    }
}
