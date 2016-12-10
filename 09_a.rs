use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut s = String::new();
    let mut f = File::open("input-09.txt").unwrap();

    f.read_to_string(&mut s).unwrap();
    let chs = s.chars().collect::<Vec<char>>();

    let (mut i, mut len, mut dcs) = (0, 0, 0);

    while i < chs.len() {
        match chs[i] {
            '(' => {
                dcs = i;
            }
            ')' => {
                let v = chs[(dcs + 1)..i].iter().cloned().collect::<String>();
                let x: Vec<usize> = v.split("x").map(|x| x.parse().unwrap()).collect();
                i += x[0];
                len += x[0] * x[1];
            }
            'A'...'Z' => {
                len += 1;
            }
            _ => (), 
        }

        i += 1;

    }

    println!("{}", len);
}
