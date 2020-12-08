use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "inputs/6.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;

    let mut hash = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  //
        let trimmed = line.trim();

        if trimmed == "" {
            count += hash.len();
            hash = HashSet::new();
        } else {
            trimmed.chars().for_each(|c| {
                hash.insert(c);
                ()
            });
        }
    }

    println!("{}", count);
}
