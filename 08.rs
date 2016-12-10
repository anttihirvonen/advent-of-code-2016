use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::fmt;

struct Screen {
    width: usize,
    height: usize,
    // Laid out in row-major order
    contents: Vec<char>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        Screen {
            width: width,
            height: height,
            contents: vec!['.'; width * height],
        }
    }

    fn rect(&mut self, width: usize, height: usize) -> () {
        for y in 0..height {
            for x in 0..width {
                self.put(x, y, '#');
            }
        }
    }

    fn at(&self, column: usize, row: usize) -> char {
        self.contents[(row % self.height) * self.width + column % self.width]
    }

    fn put(&mut self, column: usize, row: usize, val: char) -> () {
        self.contents[(row % self.height) * self.width + column % self.width] = val;
    }

    fn rotate_column(&mut self, column: usize, rotations: usize) {
        // Sorting in place would be better, but this suffices for now
        let mut temp = Vec::with_capacity(self.height);

        for y in 0..self.height {
            temp.push(self.at(column, y));
        }

        for y in 0..self.height {
            self.put(column, y + rotations, temp[y]);
        }
    }

    fn rotate_row(&mut self, row: usize, rotations: usize) {
        let mut temp = Vec::with_capacity(self.width);

        for x in 0..self.width {
            temp.push(self.at(x, row));
        }

        for x in 0..self.width {
            self.put(x + rotations, row, temp[x]);
        }
    }

    fn lit_count(&self) -> u32 {
        return self.contents.iter().fold(0u32, |acc, &p| acc + if p == '#' { 1 } else { 0 });
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            let row_start = i * self.width;
            let row_end = row_start + self.width;
            write!(f,
                   "{}\n",
                   (self.contents[row_start..row_end]).iter().cloned().collect::<String>());
        }

        Ok(())
    }
}

fn main() {
    let mut screen = Screen::new(50, 6);

    let rotate = |scr: &mut Screen, params: &[&str]| {
        let rotation: usize = params[3].parse().unwrap();
        let location: usize = params[1].split("=").nth(1).unwrap().parse().unwrap();

        match params[0] {
            "row" => scr.rotate_row(location, rotation),
            _ => scr.rotate_column(location, rotation),
        }
    };

    let rect = |scr: &mut Screen, params: &[&str]| {
        let mut s = params[0].split("x");

        scr.rect(s.next().unwrap().parse().unwrap(),
                 s.next().unwrap().parse().unwrap());
    };

    let f = BufReader::new(File::open("input-08.txt").unwrap());

    for line in f.lines() {
        let line = line.unwrap();
        let l: Vec<&str> = line.split(" ").collect();

        match l.iter().next().unwrap() {
            &"rotate" => rotate(&mut screen, &l[1..]),
            _ => rect(&mut screen, &l[1..]),
        }

        println!("{}", screen);
    }

    println!("Lit: {}", screen.lit_count());
}
