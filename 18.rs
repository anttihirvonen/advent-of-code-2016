use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn generate_next_row(prev: &String) -> String {
    let mut next = String::new();
    let prev = String::from(".") + &prev + ".";
    for i in 1..(prev.len() - 1) {
        let b = prev.as_bytes();
        if b[i - 1] != b[i + 1] {
            next.push('^');
        } else {
            next.push('.');
        }
    }

    next
}

fn count_safe_tiles(rows: u32, first_row: &str) -> u32 {
    (0..rows)
        .scan(first_row.to_owned(), |row, _| {
            let c = row.chars().fold(0, |acc, c| if c == '.' { acc + 1 } else { acc });
            *row = generate_next_row(row);
            Some(c)
        })
        .fold(0, |acc, c| acc + c)
}

fn main() {
    let s = BufReader::new(File::open("input-18.txt").unwrap()).lines().next().unwrap().unwrap();

    println!("{}", count_safe_tiles(40, &s));
    println!("{}", count_safe_tiles(400000, &s));
}
