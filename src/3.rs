use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "inputs/3.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut index = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let trimmed = line.trim();

        let mut need_lines = trimmed.to_owned();
        if trimmed.len() < (index + 1) {
            let multiplicator = (index + 1 / trimmed.len()) as usize;
            for _i in 1..multiplicator {
                need_lines.push_str(trimmed);
            }
        }

        if need_lines.chars().nth(index).unwrap() == '#' {
            count += 1;
        }
        index += 3;
    }
    println!("{}", count);
}
