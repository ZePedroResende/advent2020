use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_valid(hash: HashMap<String, String>) -> bool {
    let keys = hash.keys();

    (keys.len() == 8) || (keys.len() == 7 && !hash.contains_key("cid"))
}

fn main() {
    let filename = "inputs/4.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"((\w+):([#\w]+)+)").unwrap();

    //let lines: Vec<String> = reader.lines().map(|res| res.unwrap()).collect();

    let mut count = 0;

    let mut hash = HashMap::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  //
        let trimmed = line.trim();

        if trimmed == "" {
            if is_valid(hash.clone()) {
                count += 1;
            }
            hash = HashMap::new();
        }

        //let matched = re.captures(trimmed).unwrap();
        for matched in re.captures_iter(trimmed) {
            hash.insert(matched[2].to_string(), matched[3].to_string());
        }
    }

    println!("{}", count);
}
