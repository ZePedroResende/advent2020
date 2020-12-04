use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_valid(hash: HashMap<String,String>) -> bool {
    let keys = hash.keys();
    if !(keys.len() == 8 || (keys.len() == 7 && !hash.contains_key("cid"))){
        return false;
    }
//byr (Birth Year) - four digits; at least 1920 and at most 2002.
//iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//hgt (Height) - a number followed by either cm or in:
//
//    If cm, the number must be at least 150 and at most 193.
//    If in, the number must be at least 59 and at most 76.
//
//hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//pid (Passport ID) - a nine-digit number, including leading zeroes.



        let byr = hash.get("byr").unwrap().parse::<i32>().unwrap();
        let iyr = hash.get("iyr").unwrap().parse::<i32>().unwrap();
        let eyr = hash.get("eyr").unwrap().parse::<i32>().unwrap();
        let hgt = Regex::new(r"\d+(cm|inch)").unwrap().is_match(hash.get("hgt").unwrap())  ;
        let hcl = Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(hash.get("hcl").unwrap());
        let ecl = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap().is_match(hash.get("ecl").unwrap());
        let pid = Regex::new(r"0[0-9]{8}$").unwrap().is_match(hash.get("pid").unwrap());

        println!("{} {}", byr, byr >= 1920 && byr <= 2002);
        println!("{} {}", iyr, iyr >= 2010 && iyr <= 2020);
        println!("{}", eyr );
        println!("{}", hgt);
        println!("{}", hcl);
        println!("{}", ecl);
        println!("{} {}", pid, hash.get("pid").unwrap());

        (byr >= 1920 && byr <= 2002 ) &&
        (iyr >= 2010 && iyr <= 2020) &&
        (eyr >= 2020 && eyr <= 2030) &&
        (hgt) &&
        (hcl) &&
        (ecl) &&
        (pid)
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
                hash.insert(matched[2].to_string(),matched[3].to_string());
        }




    }

    println!(
        "{}",count );
}
