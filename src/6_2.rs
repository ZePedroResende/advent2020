use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

fn inplace_intersection<T>(a: HashSet<T>, b: HashSet<T>) -> HashSet<T>
where
    T: Hash,
    T: Eq,
{
    a.into_iter().filter(|e| b.contains(e)).collect()
}

fn main() {
    let filename = "inputs/6.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;

    let mut vec_hash: Vec<HashSet<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  //
        let trimmed = line.trim();

        if trimmed == "" {
            let started: HashSet<char> = vec_hash.get(0).unwrap().to_owned();
            let h = vec_hash.clone().into_iter().fold(started, |acc, set| {
                let c = inplace_intersection(acc.clone(), set);

                c
            });
            count += h.len();

            vec_hash.clear();
        } else {
            let mut hash = HashSet::new();
            trimmed.chars().for_each(|c| {
                hash.insert(c);
                ()
            });
            vec_hash.push(hash);
        }
    }

    println!("{}", count);
}
