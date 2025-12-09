fn main() {
    let input = parse_input(INPUT);

    let answer: u64 = input.iter().map(|b| b.largest_joltage_12_bats()).sum();

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
        let joltage_b = self.batteries[index_a..].iter().max().unwrap();

        *joltage_a as u64 * 10 + *joltage_b as u64
    }

    fn largest_joltage_12_bats(&self) -> u64 {
        #[derive(Debug)]
        struct Candidate {
            cur_index: usize,
            cur_val: u64,
        }
        let num_batteries = self.batteries.len();
        let max = self.batteries[..num_batteries - 11].iter().max().unwrap();
        let mut candidates = self.batteries[..num_batteries - 11]
            .iter()
            .enumerate()
            .filter(|(_, v)| *v == max)
            .map(|(i, v)| Candidate {
                cur_index: i,
                cur_val: *v as u64 * 10_u64.pow(11),
            })
            .collect::<Vec<_>>();

        let mut digits = 11;

        while digits > 0 {
            let mut largest_digit_so_far = 0;
            candidates = candidates
                .into_iter()
                .filter_map(|c| {
                    let a = c.cur_index + 1;
                    let b = num_batteries - (digits - 1);

                    let slice = &self.batteries[a..b];
                    let max = slice.iter().max();
                    if let Some(max) = max {
                        if *max < largest_digit_so_far {
                            return None;
                        }

                        largest_digit_so_far = *max;

                        let new_ind = a + slice.iter().position(|v| v.eq(max)).unwrap();
                        let candidate = Candidate {
                            cur_index: new_ind,
                            cur_val: c.cur_val + *max as u64 * 10_u64.pow((digits - 1) as u32),
                        };
                        Some(candidate)
                    } else {
                        Some(Candidate {
                            cur_index: a,
                            cur_val: c.cur_val
                                + self.batteries[a] as u64 * 10_u64.pow((digits - 1) as u32),
                        })
                    }
                })
                .collect();
            digits -= 1;
        }

        candidates[0].cur_val
    }
}
#[test]
fn test_input2() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
    let battery_banks = parse_input(input);

    let answer: u64 = battery_banks
        .iter()
        .map(|b| b.largest_joltage_12_bats())
        .inspect(|v| {
            println!("{v}");
        })
        .sum();

    assert_eq!(answer, 3121910778619);
}
