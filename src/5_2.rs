use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_id(trimmed: String) -> isize {
    let number = isize::from_str_radix(&trimmed, 2).unwrap();
    let row = number >> 3;

    let seat = number & 7;

    row * 8 + seat
}

fn main() {
    let filename = "inputs/5.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let vector: Vec<isize> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    'B' => '1',
                    'R' => '1',
                    _ => '0',
                })
                .collect()
        })
        .map(|string| calculate_id(string))
        .collect();

    let max = vector.iter().max().unwrap();
    let empty_id = (40..*max)
        .find(|id| {
            !vector.contains(id) && vector.contains(&(*id - 1)) && vector.contains(&(*id + 1))
        })
        .unwrap();

    println!("{}", empty_id);
}

/*
fn main() {
    let filename = "inputs/5.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut v: Vec<isize> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    'B' => '1',
                    'R' => '1',
                    _ => '0',
                })
                .collect()
        })
        .map(|string| calculate_id(string))
        .collect();

    v.sort();
    for (l, r) in v.iter().zip(v[0]..v[v.len() - 1]) {
        if *l != r {
            println!("{}", r);
            break;
        }
    }
}
*/
