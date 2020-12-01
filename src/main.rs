use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "inputs/1/input";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut hash: HashMap<i64, i8> = HashMap::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  //
        let trimmed = line.trim();
        let int = trimmed.parse::<i64>().unwrap();
        let int_look = i64::abs(2020 as i64 - int);

        if hash.contains_key(&int_look) {
            panic!("{}", int_look * int);
        } else {
            hash.insert(int, 1);
        }
    }
}
