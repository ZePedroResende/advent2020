use std::fs::File;
use std::io::{BufRead, BufReader};


struct Passport {
    cid: Option<str>
}

struct NorthPole {
    byr: Option<str>
    iyr:
    eyr:
    hgt:
    hcl:
    ecl:
    pid:

}

fn calculate_colisions(right: usize, down: usize, lines: Vec<String>) -> i32 {
    let mut count = 0;
    let mut index = 0;

    for line in lines.iter().step_by(down) {
        let mut need_lines = line.to_owned();
        if line.len() < (index + 1) {
            let multiplicator = (index + 1 / line.len()) as usize;
            for _i in 1..multiplicator {
                need_lines.push_str(&line.clone());
            }
        }

        if need_lines.chars().nth(index).unwrap() == '#' {
            count += 1;
        }
        index += right;
    }
    count
}

fn main() {
    let filename = "inputs/3.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|res| res.unwrap()).collect();

    println!(
        "{}",
        calculate_colisions(1, 1, lines.clone())
            * calculate_colisions(3, 1, lines.clone())
            * calculate_colisions(5, 1, lines.clone())
            * calculate_colisions(7, 1, lines.clone())
            * calculate_colisions(1, 2, lines.clone())
    );
}
