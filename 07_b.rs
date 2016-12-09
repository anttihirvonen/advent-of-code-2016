use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn find_three_letter_seqs(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();

    // There must be a better way to implement this..
    for c in 0..(chars.len() - 2) {
        if chars[c] == chars[c + 2] && chars[c] != chars[c + 1] {
            result.push(chars[c..(c + 3)].iter().cloned().collect::<String>());
        }
    }

    result
}

fn supports_tls(supernet: &Vec<String>, hypernet: &Vec<String>) -> bool {
    // There is most likely a better way to do this
    for s in supernet.iter() {
        for h in hypernet.iter() {
            let sb = s.as_bytes();
            let hb = h.as_bytes();
            if sb[0] == hb[1] && sb[1] == hb[0] {
                return true;
            }
        }
    }

    false
}

fn main() {
    let f = BufReader::new(File::open("input-07.txt").unwrap());
    let mut count = 0;

    for line in f.lines() {
        let l = line.unwrap();
        let v: Vec<&str> = l.split(|x| x == '[' || x == ']').collect();
        let mut supernet_seqs: Vec<String> = Vec::new();
        let mut hypernet_seqs: Vec<String> = Vec::new();

        for (i, s) in v.iter().enumerate() {
            if i % 2 == 0 {
                supernet_seqs.extend(find_three_letter_seqs(s));
            } else {
                hypernet_seqs.extend(find_three_letter_seqs(s));
            }
        }

        if supports_tls(&supernet_seqs, &hypernet_seqs) {
            count += 1;
        }
    }

    println!("{}", count);
}
