use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq, Eq)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

type Instruction = (Op, i64);

fn main() {
    let filename = "inputs/5.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut id = 0;

    for line in reader.lines() {
        // Show the line and its number.
        //
        let trimmed: String = line.unwrap();
    }

    println!("{}", id);
}
