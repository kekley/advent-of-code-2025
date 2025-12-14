use std::collections::{HashMap, HashSet};

fn main() {
    let diagram = parse_input(INPUT);
    let result = diagram.beams();
    println!();

    println!("beam splits: {result}");

    let num_timelines = diagram.timelines();

    println!("Number of timelines: {num_timelines}");
}

const INPUT: &str = include_str!("../input.txt");

const START: u8 = b'S';
const EMPTY: u8 = b'.';
const SPLITTER: u8 = b'^';

const BEAM: u8 = b'|';

struct Diagram {
    data: &'static [u8],
    stride: usize,
}

impl Diagram {
    fn timelines(&self) -> usize {
        let start_x = self.data.iter().position(|b| b.eq(&START)).unwrap();

        let mut cache = HashMap::new();

        self.find_timelines_for_pos(start_x, 0, &mut cache)
    }
    fn find_timelines_for_pos(
        &self,
        x: usize,
        y: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(cached) = cache.get(&(x, y)) {
            return *cached;
        }
        let dim_y = self.data.len() / self.stride;
        if y + 1 > dim_y {
            cache.insert((x, y), 1);
            return 1;
        }

        let char = self.get_char_at(x, y);

        match char {
            EMPTY | START => {
                let below = self.find_timelines_for_pos(x, y + 1, cache);
                cache.insert((x, y), below);
                below
            }
            SPLITTER => {
                let left = self.find_timelines_for_pos(x - 1, y + 1, cache);

                let right = self.find_timelines_for_pos(x + 1, y + 1, cache);

                cache.insert((x, y), left + right);
                left + right
            }
            r => {
                dbg!(r as char);

                unreachable!()
            }
        }
    }
    fn get_char_at(&self, x: usize, y: usize) -> u8 {
        self.data[x + y * self.stride]
    }
    fn beams(&self) -> u64 {
        let mut split_count = 0;
        let mut beam_positions = HashSet::new();

        let start_pos = self.data.iter().position(|b| b.eq(&START)).unwrap();

        beam_positions.insert(start_pos);

        let mut lines = self.data.chunks(self.stride);
        lines.next();

        while let Some(line) = lines.by_ref().next() {
            let mut splitter_positions = vec![];
            for pos in beam_positions.clone() {
                match line[pos] {
                    EMPTY => {}
                    SPLITTER => {
                        splitter_positions.push(pos);
                        beam_positions.insert(pos + 1);

                        beam_positions.insert(pos - 1);
                        split_count += 2;
                    }
                    r => {
                        dbg!(r as char);
                        unreachable!()
                    }
                }
            }

            for splitter in splitter_positions {
                beam_positions.remove(&splitter);
                split_count -= 1;
            }

            let mut line_copy = line.to_owned();
            beam_positions.iter().for_each(|i| {
                if line_copy[*i] != SPLITTER {
                    line_copy[*i] = BEAM;
                }
            });
            let as_str = str::from_utf8(&line_copy).unwrap();
            print!("{as_str}");
        }

        split_count
    }
}

fn parse_input(input: &'static str) -> Diagram {
    let input_trimmed = input.trim_end();
    let stride = input_trimmed.find('\n').unwrap() + 1;

    Diagram {
        data: input_trimmed.as_bytes(),
        stride,
    }
}

#[test]
fn test() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    let diag = parse_input(input);

    let result = diag.beams();

    assert_eq!(result, 21);
}
