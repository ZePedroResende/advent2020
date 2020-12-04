use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "inputs/2.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let mut count = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  //
        let trimmed = line.trim();

        let matched = re.captures(trimmed).unwrap();

        let min = matched.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let max = matched.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let letter = matched.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = matched.get(4).unwrap().as_str();

        let first = password.chars().nth(min).unwrap();
        let last = password.chars().nth(max).unwrap();

        if (first == letter) ^ (last == letter) {
            count += 1;
        }
    }
    println!("{}", count);
}
