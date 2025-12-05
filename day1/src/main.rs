fn main() {
    let input = read_input(INPUT);
    let mut zero_counts = 0;
    let mut current_dial_position: u16 = 50;
    for rotation in input {
        println!("Current position: {current_dial_position}");
        println!("Performing rotation: {rotation:?}");

        let rotation_result = apply_rotation(current_dial_position, rotation);
        current_dial_position = rotation_result.new_position;
        zero_counts += rotation_result.times_zero_passed;

        println!("Position after rotation: {current_dial_position}");

        println!();
    }
    println!("code: {zero_counts}");
}

const INPUT: &str = include_str!("../input.txt");

fn read_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .flat_map(|line| {
            let (direction, amount) = line.trim_end().split_at_checked(1)?;

            if direction == "L" {
                Some(Rotation::Left(amount.parse().unwrap()))
            } else {
                Some(Rotation::Right(amount.parse().unwrap()))
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct RotationResult {
    new_position: u16,
    times_zero_passed: u16,
}

fn apply_rotation(current_dial_position: u16, rotation: Rotation) -> RotationResult {
    const NUMBER_OF_DIAL_POINTS: u16 = 100;

    let mut times_zero_passed = 0;

    match rotation {
        Rotation::Left(left_rotation_amount) => {
            let left_rotation_amount = if left_rotation_amount >= NUMBER_OF_DIAL_POINTS {
                times_zero_passed += left_rotation_amount / NUMBER_OF_DIAL_POINTS;
                left_rotation_amount % NUMBER_OF_DIAL_POINTS
            } else {
                left_rotation_amount
            };
            if left_rotation_amount >= current_dial_position && current_dial_position != 0 {
                times_zero_passed += 1;
            }

            let equivalent_right_turn = NUMBER_OF_DIAL_POINTS - left_rotation_amount;

            let new_dial_position =
                (current_dial_position + equivalent_right_turn) % NUMBER_OF_DIAL_POINTS;

            RotationResult {
                new_position: new_dial_position,
                times_zero_passed,
            }
        }
        Rotation::Right(right_rotation_amount) => {
            let new_dial_position =
                (current_dial_position + right_rotation_amount) % NUMBER_OF_DIAL_POINTS;

            times_zero_passed +=
                (current_dial_position + right_rotation_amount) / NUMBER_OF_DIAL_POINTS;

            RotationResult {
                new_position: new_dial_position,
                times_zero_passed,
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left(u16),
    Right(u16),
}

#[test]
fn left_overflow() {
    let current_pos = 5;
    let rotation = Rotation::Left(10);
    let rotation_result = apply_rotation(current_pos, rotation);
    let new_pos = rotation_result.new_position;
    let zero_count = rotation_result.times_zero_passed;

    assert_eq!(new_pos, 95);
    assert_eq!(zero_count, 1);
}

#[test]
fn right_overflow() {
    let current_pos = 95;
    let rotation = Rotation::Right(5);

    let rotation_result = apply_rotation(current_pos, rotation);
    let new_pos = rotation_result.new_position;
    let zero_count = rotation_result.times_zero_passed;

    assert_eq!(new_pos, 0);
    assert_eq!(zero_count, 1);
}

#[test]
fn full_turns() {
    let current_pos = 95;
    let rotation = Rotation::Right(200);
    let rotation_result = apply_rotation(current_pos, rotation);
    let new_pos = rotation_result.new_position;
    let zero_count = rotation_result.times_zero_passed;

    assert_eq!(new_pos, current_pos);
    assert_eq!(zero_count, 2);

    let current_pos = 95;
    let rotation = Rotation::Left(500);
    let rotation_result = apply_rotation(current_pos, rotation);
    let new_pos = rotation_result.new_position;
    let zero_count = rotation_result.times_zero_passed;

    assert_eq!(new_pos, current_pos);
    assert_eq!(zero_count, 5);
}
#[test]
fn full_turns_and_overflow() {
    let current_pos = 95;
    let rotation = Rotation::Right(210);
    let rotation_result = apply_rotation(current_pos, rotation);
    let new_pos = rotation_result.new_position;
    let zero_count = rotation_result.times_zero_passed;

    assert_eq!(new_pos, 5);
    assert_eq!(zero_count, 3);

    let current_pos = 5;
    let rotation = Rotation::Left(515);
    let rotation_result = apply_rotation(current_pos, rotation);
    let new_pos = rotation_result.new_position;
    let zero_count = rotation_result.times_zero_passed;

    assert_eq!(new_pos, 90);
    assert_eq!(zero_count, 6);
}

#[test]
fn landing_on_zero() {
    let current_pos = 0;
    let rotation = Rotation::Right(1000);
    let rotation_result = apply_rotation(current_pos, rotation);
    let new_pos = rotation_result.new_position;
    let zero_count = rotation_result.times_zero_passed;

    assert_eq!(new_pos, 0);
    assert_eq!(zero_count, 10);

    let current_pos = 0;
    let rotation = Rotation::Left(1000);
    let rotation_result = apply_rotation(current_pos, rotation);
    let new_pos = rotation_result.new_position;
    let zero_count = rotation_result.times_zero_passed;

    assert_eq!(new_pos, 0);
    assert_eq!(zero_count, 10);
}
#[test]
fn part_2_example() {
    let turns = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
    let parsed = read_input(turns);
    let mut zero_counts = 0;
    let mut current_dial_position: u16 = 50;
    for rotation in parsed {
        println!("Current position: {current_dial_position}");
        println!("Performing rotation: {rotation:?}");

        let rotation_result = apply_rotation(current_dial_position, rotation);
        current_dial_position = rotation_result.new_position;
        zero_counts += rotation_result.times_zero_passed;

        println!("Position after rotation: {current_dial_position}");
        println!("Zero Count: {zero_counts}");

        println!();
    }
    println!("code: {zero_counts}");
}
