pub fn main() {
    let problems = parse_input(INPUT);
    let sum = problems.solve();

    println!("Sum:{sum}");
}

const INPUT: &str = include_str!("../input.txt");

struct Problems {
    numbers: [Vec<u64>; 4],
    operations: Vec<Operation>,
}

impl Problems {
    fn solve(&self) -> u64 {
        assert!(
            self.numbers[0].len() == self.numbers[1].len()
                && self.numbers[1].len() == self.numbers[2].len()
                && self.numbers[2].len() == self.numbers[3].len()
                && self.numbers[3].len() == self.operations.len()
        );

        (0..self.numbers[0].len())
            .map(|i| {
                let num1 = self.numbers[0][i];
                let num2 = self.numbers[1][i];
                let num3 = self.numbers[2][i];
                let num4 = self.numbers[3][i];

                match self.operations[i] {
                    Operation::Add => num1 + num2 + num3 + num4,
                    Operation::Mul => num1 * num2 * num3 * num4,
                }
            })
            .sum()
    }
}
enum Operation {
    Add,
    Mul,
}
fn parse_input2(input: &str) -> Problems {
    let mut lines = input.trim_end().lines();
    let nums_1 = lines.next().unwrap().split_ascii_whitespace();
    let nums_2 = lines.next().unwrap().split_ascii_whitespace();
    let nums_3 = lines.next().unwrap().split_ascii_whitespace();
    let nums_4 = lines.next().unwrap().split_ascii_whitespace();
    let ops = lines.next().unwrap().split_ascii_whitespace();

    while let (Some(n1), Some(n2), Some(n3), Some(n4)) =
        (nums_1.next(), nums_2.next(), nums_3.next(), nums_4.next())
    {
        let chars1 = n1.as_bytes().iter().rev().collect::<Vec<_>>();

        let chars2 = n2.as_bytes().iter().rev().collect::<Vec<_>>();

        let chars3 = n3.as_bytes().iter().rev().collect::<Vec<_>>();

        let chars4 = n4.as_bytes().iter().rev().collect::<Vec<_>>();
    }

    let ops = ops
        .map(|op| {
            if op == "+" {
                Operation::Add
            } else {
                Operation::Mul
            }
        })
        .collect();

    let numbers = [nums_1, nums_2, nums_3, nums_4];

    Problems {
        numbers,
        operations: ops,
    }
}

fn parse_input(input: &str) -> Problems {
    let mut lines = input.trim_end().lines();
    let nums_1 = lines.next().unwrap();
    let nums_2 = lines.next().unwrap();
    let nums_3 = lines.next().unwrap();
    let nums_4 = lines.next().unwrap();
    let ops = lines.next().unwrap();

    let nums_1 = nums_1
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let nums_2 = nums_2
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let nums_3 = nums_3
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let nums_4 = nums_4
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let ops = ops
        .split_ascii_whitespace()
        .map(|op| {
            if op == "+" {
                Operation::Add
            } else {
                Operation::Mul
            }
        })
        .collect();

    let numbers = [nums_1, nums_2, nums_3, nums_4];

    Problems {
        numbers,
        operations: ops,
    }
}
