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

    let mut result;
    let mut id = 0;

    for line in reader.lines() {
        // Show the line and its number.
        //
        let trimmed: String = line
            .unwrap()
            .chars()
            .map(|c| match c {
                'F' => '0',
                'B' => '1',
                'L' => '0',
                'R' => '1',
                _ => c,
            })
            .collect();

        result = calculate_id(trimmed);
        id = if result > id { result } else { id };
    }

    println!("{}", id);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn to_binary(string: &str) -> String {
        string
            .chars()
            .map(|c| match c {
                'F' => '0',
                'B' => '1',
                'L' => '0',
                'R' => '1',
                _ => c,
            })
            .collect()
    }

    #[test]
    fn test_calculate_id() {
        assert_eq!(calculate_id(to_binary("BFFFBBFRRR")), 567);
        assert_eq!(calculate_id(to_binary("FFFBBBFRRR")), 119);
        assert_eq!(calculate_id(to_binary("BBFFBBFRLL")), 820);
        assert_eq!(calculate_id(to_binary("FBFBBFFRLR")), 357);
    }
}
