use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn value_to_int(hash: HashMap<String, String>, key: String) -> i32 {
    hash.get(&key).unwrap().parse::<i32>().unwrap()
}

fn check_regex(hash: HashMap<String, String>, key: String, regex: String) -> bool {
    Regex::new(&regex)
        .unwrap()
        .is_match(hash.get(&key).unwrap())
}

fn hgt_size(hash: HashMap<String, String>) -> bool {
    let value = hash.get("hgt").unwrap();

    let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let matched = re.captures(&value).unwrap();

    let number = matched.get(1).unwrap().as_str().parse::<usize>().unwrap();;
    let measure = matched.get(2).unwrap().as_str();

    if measure == "cm" {
        150 <= number && 193 >= number
    } else {
        59 <= number && 76 >= number
    }
}

fn is_valid(hash: HashMap<String, String>) -> bool {
    let keys = hash.keys();

    if !((keys.len() == 8) || (keys.len() == 7 && !hash.contains_key("cid"))) {
        return false;
    }

    let byr = value_to_int(hash.clone(), "byr".to_string());
    let iyr = value_to_int(hash.clone(), "iyr".to_string());
    let eyr = value_to_int(hash.clone(), "eyr".to_string());

    let hgt = check_regex(hash.clone(), "hgt".to_string(), r"^\d+(cm|in)$".to_string());
    let hcl = check_regex(
        hash.clone(),
        "hcl".to_string(),
        r"^#[0-9a-f]{6}$".to_string(),
    );
    let ecl = check_regex(
        hash.clone(),
        "ecl".to_string(),
        r"^(amb|blu|brn|gry|grn|hzl|oth)$".to_string(),
    );
    let pid = check_regex(hash.clone(), "pid".to_string(), r"^[0-9]{9}$".to_string());

    if !((byr >= 1920 && byr <= 2002)
        && (iyr >= 2010 && iyr <= 2020)
        && (eyr >= 2020 && eyr <= 2030)
        && (hgt)
        && (hcl)
        && (ecl)
        && (pid))
    {
        return false;
    }
    hgt_size(hash.clone())
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
