fn main() {
    let input = parse_input(INPUT);
    let mut sum: u64 = 0;
    for range in input {
        println!("{range:?}");
        let ids = find_invalid_id_count_in_range(&range);
        sum += ids
            .iter()
            .inspect(|id| {
                println!("{id}");
            })
            .sum::<u64>();
        println!();
    }
    println!("{sum}");
}

#[derive(Debug, Clone, Copy)]
struct IDRange {
    start: &'static str,
    end: &'static str,
}
const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &'static str) -> Vec<IDRange> {
    input
        .trim()
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            IDRange { start, end }
        })
        .collect()
}

fn find_invalid_id_count_in_range(range: &IDRange) -> Vec<u64> {
    let end_digits = range.end.len();

    let mut ids = vec![];
    let start_num: u64 = range.start.parse().unwrap();
    let end_num: u64 = range.end.parse().unwrap();

    let mut repeating_part = 1.to_string();

    let mut guess_str = String::with_capacity(end_digits);

    while repeating_part.len() <= end_digits / 2 {
        guess_str.clear();
        while end_digits - guess_str.len() >= repeating_part.len() {
            guess_str.push_str(&repeating_part);
            let parsed: u64 = guess_str.parse().unwrap();

            if guess_str.len() > 1 && (start_num..end_num + 1).contains(&parsed) {
                ids.push(parsed);
            }
        }

        repeating_part = (repeating_part.parse::<u64>().unwrap() + 1).to_string();
    }
    ids.sort();
    ids.dedup();
    ids
}

#[test]
fn example_data() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let ranges = parse_input(input);

    let ids = ranges
        .into_iter()
        .flat_map(|range| find_invalid_id_count_in_range(&range))
        .collect::<Vec<_>>();

    let num_ids = ids.len();
    let sum: u64 = ids.iter().sum();
    dbg!(ids);
    assert_eq!(num_ids, 13);

    assert_eq!(sum, 4174379265);
}
