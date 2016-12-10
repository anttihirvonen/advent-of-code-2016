use std::io::prelude::*;
use std::fs::File;

fn decompressed_len(chs: &[char]) -> (usize, usize) {
    let (mut i, mut len, mut dcs) = (0, 0, 0);
    while i < chs.len() {
        match chs[i] {
            '(' => {
                dcs = i;
            },
            ')' => {
                let v = chs[(dcs + 1)..i].iter().cloned().collect::<String>();
                let x: Vec<usize> = v.split("x").map(|x| x.parse().unwrap()).collect();
                let (k, l) = decompressed_len(&chs[(i + 1)..(i + x[0] + 1)]);
                i += k;
                len += l * x[1];
            },
            'A'...'Z' => {
                len += 1;
            },
            _ => (),
        }

        i += 1
    }

    (i, len)
}

fn main() {
    let mut f = File::open("input-09.txt").unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();

    println!("{}", decompressed_len(&s.chars().collect::<Vec<char>>()).1);
}
