use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "inputs/1/input";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut vec = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  //
        let trimmed = line.trim();
        let int = trimmed.parse::<i64>().unwrap();
        vec.push(int);
    }

    let arr_size = vec.len();
    let sum = 2020;

    for i in 0..arr_size - 1 {
        let mut s = std::collections::HashSet::new();

        let curr_sum = sum - vec[i];
        for j in i + 1..arr_size {
            let k = curr_sum - vec[j];
            if s.contains(&k) {
                panic!("{}", vec[i] * vec[j] * k);
            }
            s.insert(vec[j]);
        }
    }
}
