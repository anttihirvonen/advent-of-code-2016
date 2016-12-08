// Probably not very idiomatic Rust. My first Rust impl.
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn contains_pair(s : &str) -> bool {
    let chars : Vec<char> = s.chars().collect();

    // There must be better way to do this
    for c in 0..(chars.len()-3) {
        if chars[c] == chars[c+3] &&
           chars[c+1] == chars[c+2] &&
           chars[c] != chars[c+1]
           {
               return true;
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
        let mut ip_tls = [false, false];

        for (i, s) in v.iter().enumerate() {
            ip_tls[i % 2] |= contains_pair(s);
        }

        if ip_tls[0] && !ip_tls[1] { 
            count += 1 
        }
    }

    println!("{}", count);
}
