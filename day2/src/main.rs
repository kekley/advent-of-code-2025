fn main() {
    let input = parse_input(INPUT);

    let ids = input
        .into_iter()
        .flat_map(|range| find_invalid_id_count_in_range(&range))
        .collect::<Vec<_>>();
    let sum: u64 = ids.iter().sum();

    println!("total: {sum}");
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
    let start_digits = range.start.len();
    let end_digits = range.end.len();

    if end_digits == start_digits && start_digits & 1 == 1 && end_digits & 1 == 1 {
        //A range containing only numbers with an odd number of digits can never contain an invalid
        //ID
        return vec![];
    }

    let mut ids = vec![];
    let start_num: u64 = range.start.parse().unwrap();
    let end_num: u64 = range.end.parse().unwrap();

    let mut guess = {
        let (digits, _) = range.end.split_at(end_digits.div_ceil(2));
        format!("{digits}{digits}")
    };

    //    println!("first guess: {guess}");
    while let parsed = guess.parse().unwrap()
        && start_num <= parsed
    {
        //        println!("guess: {guess}");

        //can definitely optimize here to skip a lot of iterations but lol this works
        if (start_num..end_num + 1).contains(&parsed) {
            ids.push(parsed);
        }

        guess = {
            let (half, _) = guess.split_at(guess.len().div_ceil(2));
            format!(
                "{new_half}{new_half}",
                new_half = half.parse::<u64>().unwrap() - 1
            )
        };
    }

    ids
}

#[test]
fn odd_digits_range() {
    let input = "1698522-1698528";
    let ranges = parse_input(input);

    for range in ranges {
        let num_ids = find_invalid_id_count_in_range(&range).len();
        assert_eq!(num_ids, 0);
    }
}

#[test]
fn _11_22() {
    let input = "11-22";
    let ranges = parse_input(input);

    for range in ranges {
        let num_ids = find_invalid_id_count_in_range(&range).len();
        assert_eq!(num_ids, 2);
    }
}

#[test]
fn _95_115() {
    let input = "95-115";
    let ranges = parse_input(input);

    for range in ranges {
        let num_ids = find_invalid_id_count_in_range(&range).len();
        assert_eq!(num_ids, 1);
    }
}

#[test]
fn _998_1012() {
    let input = "998-1012";
    let ranges = parse_input(input);

    for range in ranges {
        let num_ids = find_invalid_id_count_in_range(&range).len();
        assert_eq!(num_ids, 1);
    }
}

#[test]
fn _1188511880_1188511890() {
    let input = "1188511880-1188511890";
    let ranges = parse_input(input);

    for range in ranges {
        let num_ids = find_invalid_id_count_in_range(&range).len();
        assert_eq!(num_ids, 1);
    }
}

#[test]
fn example_data() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862";
    let ranges = parse_input(input);

    let ids = ranges
        .into_iter()
        .flat_map(|range| find_invalid_id_count_in_range(&range))
        .collect::<Vec<_>>();

    let num_ids = ids.len();
    let sum: u64 = ids.iter().sum();
    assert_eq!(num_ids, 8);

    assert_eq!(sum, 1227775554);
}
