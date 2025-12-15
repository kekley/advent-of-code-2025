fn main() {
    let input = parse_input(INPUT);

    let a = part1(input, 1000);
}

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("x:{}, y:{}, z:{}", self.x, self.y, self.z))
    }
}

impl Position {
    fn distance_from(&self, other: &Position) -> f64 {
        let x_dist = (self.x - other.x).pow(2);

        let y_dist = (self.y - other.y).pow(2);

        let z_dist = (self.z - other.z).pow(2);

        let sum = x_dist + y_dist + z_dist;

        (sum as f64).sqrt()
    }
}

fn part1(mut positions: Vec<Position>, num_connections: usize) -> usize {
    todo!()
}

fn parse_input(input: &str) -> Vec<Position> {
    let lines = input.trim_end().lines();

    lines
        .map(|line| {
            let mut split = line.split(",");

            let [x, y, z]: [i64; 3] =
                std::array::from_fn(|_| split.next().unwrap().parse().unwrap());

            Position { x, y, z }
        })
        .collect()
}

#[test]
fn small_input() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
    let positions = parse_input(input);
    let a = part1(positions, 10);
}
