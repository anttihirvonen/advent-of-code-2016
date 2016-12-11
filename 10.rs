use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::cmp;

#[derive(Debug,Clone)]
enum Target {
    Bot(u32),
    Output(u32),
}

type Chip = u32;

#[derive(Debug)]
struct Transfer {
    chip: Chip,
    target: Target
}

#[derive(Debug)]
struct Bot {
    id: u32,
    chip: Option<Chip>, // can hold only one chip
    lt: Target,
    ht: Target,
}

fn u32_or_panic(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

fn main() {
    let f = BufReader::new(File::open("input-10.txt").unwrap());

    let mut bots: Vec<Bot> = Vec::new();
    let mut transfers: Vec<Transfer> = Vec::new();

    for line in f.lines() {
        let line = line.unwrap();
        let components = line.split(" ").collect::<Vec<&str>>();

        match components[0] {
            "bot" => {
                let parse_target = |i: usize| {
                    let id: u32 = u32_or_panic(components[i+1]);

                    match components[i] {
                        "output" => Target::Output(id),
                        "bot" => Target::Bot(id),
                        _ => panic!(),
                    }
                };

                bots.push(Bot {
                    id: u32_or_panic(components[1]),
                    chip: None,
                    lt: parse_target(5),
                    ht: parse_target(10),
                });
            }
            "value" => {
                transfers.push(Transfer{
                    chip: u32_or_panic(components[1]),
                    target: Target::Bot(u32_or_panic(components[5]))
                });
            }
            _ => panic!()
        }
    }

    bots.sort_by_key(|b| b.id);

    let mut results = (0, 1);
    // evaluation
    while let Some(c) = transfers.pop() {
        match c.target {
            Target::Bot(i) => {
                let bot = &mut bots[i as usize];

                if bot.chip.is_none() {
                    bot.chip = Some(c.chip);
                } else {
                    let val = bot.chip.take().unwrap();
                    let min = cmp::min(c.chip, val);
                    let max = cmp::max(c.chip, val);
                    if min == 17 && max == 61 {
                        results.0 = bot.id;
                    }
                    transfers.push(Transfer{ chip: min, target: bot.lt.clone() });
                    transfers.push(Transfer{ chip: max, target: bot.ht.clone() });
                }
            }
            Target::Output(i) => {
                match i {
                    0...2 => results.1 *= c.chip,
                    _ => continue
                }
            }
        }
    }

    println!("Results (A, B) = {:?}", results);
}
