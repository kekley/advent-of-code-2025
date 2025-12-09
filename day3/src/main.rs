fn main() {
    let input = parse_input(INPUT);

    let answer: u64 = input
        .iter()
        .map(|b| {
            b.batteries.iter().for_each(|b| print!("{b}"));
            println!();
            let a = b.largest_joltage();

            println!("largest: {a}");
            a
        })
        .sum();

    println!("Max: {answer}");
}

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<BatteryBank> {
    let lines = input.trim_end().lines();
    lines
        .map(|line| {
            let boxed: Box<[u8]> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();
            BatteryBank { batteries: boxed }
        })
        .collect()
}

#[derive(Debug)]
struct BatteryBank {
    batteries: Box<[u8]>,
}
impl BatteryBank {
    fn largest_joltage(&self) -> u64 {
        let num_batteries = self.batteries.len();

        //we use .rev() here because max will return the last value it finds that is at least max and we want the first
        //in the array
        let (index_a, joltage_a) = self.batteries[0..num_batteries - 1]
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();
        let joltage_b = self.batteries[index_a..]
            .iter()
            .enumerate()
            .map(|(_, b)| b)
            .max()
            .unwrap();

        *joltage_a as u64 * 10 + *joltage_b as u64
    }
    fn largest_joltage_12_bats(&self) -> u64 {
        let max = self.batteries.iter().max().copied().unwrap();
        let candidate_indicies = self
            .batteries
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == max);
    }
}
#[test]
fn test_input() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
    let battery_banks = parse_input(input);

    let answer: u64 = battery_banks.iter().map(|b| b.largest_joltage()).sum();

    assert_eq!(answer, 357);
}
