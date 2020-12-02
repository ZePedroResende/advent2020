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

        let min = matched.get(1).unwrap().as_str().parse::<usize>().unwrap();;
        let max = matched.get(2).unwrap().as_str().parse::<usize>().unwrap();;
        let letter = matched.get(3).unwrap().as_str();
        let password = matched.get(4).unwrap().as_str();
        let letter_count = password.matches(letter).count();

        if (min <= letter_count) && (letter_count <= max) {
            count += 1;
        }
    }
    println!("{}", count);
}
