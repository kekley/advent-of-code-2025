use std::fmt::Write as _;

fn main() {
    let input = parse_input(INPUT);

    println!("{input:?}");
    let result = input.count_removable_paper();
    println!("count: {result}");
}

const INPUT: &str = include_str!("../input.txt");

struct Diagram {
    width: usize,
    height: usize,
    stride: usize,
    data: Vec<bool>,
}

impl Diagram {
    fn part2(&mut self) -> u64 {}
    fn count_removable_paper(&self) -> u64 {
        let mut total = 0;
        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                let index = xy_to_ind(x, y, self.stride);
                let value = self.data[index];
                if value {
                    let mut count = 0;
                    let top_left_index = xy_to_ind(x - 1, y - 1, self.stride);

                    let top_index = xy_to_ind(x, y - 1, self.stride);

                    let top_right_index = xy_to_ind(x + 1, y - 1, self.stride);

                    let left_index = xy_to_ind(x - 1, y, self.stride);

                    let right_index = xy_to_ind(x + 1, y, self.stride);

                    let bottom_left_index = xy_to_ind(x - 1, y + 1, self.stride);

                    let bottom_index = xy_to_ind(x, y + 1, self.stride);

                    let bottom_right_index = xy_to_ind(x + 1, y + 1, self.stride);

                    if self.data[top_left_index] {
                        count += 1;
                    }
                    if self.data[top_index] {
                        count += 1;
                    }
                    if self.data[top_right_index] {
                        count += 1;
                    }
                    if self.data[left_index] {
                        count += 1;
                    }
                    if self.data[right_index] {
                        count += 1;
                    }
                    if self.data[bottom_left_index] {
                        count += 1;
                    }
                    if self.data[bottom_index] {
                        count += 1;
                    }
                    if self.data[bottom_right_index] {
                        count += 1;
                    }
                    if count < 4 {
                        total += 1;
                    }
                }
            }
        }
        total
    }
}

impl std::fmt::Debug for Diagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::with_capacity(self.data.len());
        let _ = writeln!(&mut buf, "Stride: {}", self.stride);
        let chunks = self.data.chunks_exact(self.stride);
        for chunk in chunks {
            for val in chunk {
                let char = if *val { '@' } else { '.' };
                buf.push(char);
            }

            buf.push('\n');
        }
        f.write_str(&buf)
    }
}
fn parse_input(input: &str) -> Diagram {
    let lines = input.trim_end().lines();

    let height = lines.clone().count();
    let width = lines.clone().next().unwrap().len();
    let stride = width + 2;

    dbg!(width, height);

    let mut data = vec![false; (width + 2) * (height + 2)];

    for (y, line) in lines.enumerate() {
        let bytes = line.as_bytes();
        for (x, byte) in bytes.iter().enumerate() {
            let index = xy_to_ind(x + 1, y + 1, stride);
            if *byte == b'@' {
                data[index] = true;
            }
        }
    }

    Diagram {
        stride,
        data,
        width,
        height,
    }
}
#[inline]
fn ind_to_xy(index: usize, stride: usize) -> (usize, usize) {
    let y = index / stride;
    let x = index % stride;
    (x, y)
}
#[inline]
fn xy_to_ind(x: usize, y: usize, stride: usize) -> usize {
    x + (y * stride)
}
