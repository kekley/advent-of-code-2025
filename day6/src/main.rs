pub fn main() {
    let problems = parse_input(INPUT);
    let sum = problems.solve();

    println!("Sum: {sum}");

    let problems2 = parse_input2(INPUT);
    let sum = problems2.solve();

    println!("Sum2: {sum}");
}

const INPUT: &str = include_str!("../input.txt");

struct Problems {
    numbers: [Vec<u64>; 4],
    operations: Vec<Operation>,
}

struct Problems2 {
    numbers: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

impl Problems2 {
    pub fn solve(&self) -> u64 {
        self.numbers
            .iter()
            .zip(self.operations.iter())
            .map(|(nums, op)| match op {
                Operation::Add => nums.iter().sum::<u64>(),
                Operation::Mul => nums.iter().product::<u64>(),
            })
            .sum()
    }
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
fn parse_input2(input: &str) -> Problems2 {
    let mut lines = input.trim_end().lines();
    let nums_1 = lines.next().unwrap().as_bytes();
    let nums_2 = lines.next().unwrap().as_bytes();
    let nums_3 = lines.next().unwrap().as_bytes();
    let nums_4 = lines.next().unwrap().as_bytes();
    let ops = lines.next().unwrap().split_ascii_whitespace();
    let stride = nums_1.len();

    let mut numbers: Vec<Vec<u64>> = vec![];
    let operations: Vec<Operation> = ops
        .map(|op| match op {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => {
                unreachable!()
            }
        })
        .collect();

    let mut inner_vec = vec![];

    let slices = [nums_1, nums_2, nums_3, nums_4];

    for i in 0..stride {
        let mut num: u64 = 0;
        for slice in slices {
            let byte = slice[i];
            if byte != b' ' {
                num *= 10;
                num += (byte - b'0') as u64;
            }
        }
        if num == 0 && !inner_vec.is_empty() {
            numbers.push(inner_vec.clone());
            inner_vec.clear();
        } else if num != 0 {
            inner_vec.push(num);
        } else {
            unreachable!();
        }
    }
    numbers.push(inner_vec);

    dbg!(operations.len());
    dbg!(numbers.len());

    assert!(operations.len() == numbers.len());
    Problems2 {
        numbers,
        operations,
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
