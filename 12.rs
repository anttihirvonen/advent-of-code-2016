use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

type Register = u8;

#[derive(Debug)]
enum Instruction {
    CpyV(i32, Register),
    CpyR(Register, Register),
    Inc(Register),
    Dec(Register),
    JmpR(Register, i32),
    JmpA(i32)
}

fn register(s: &str) -> u8 {
    (s.chars().next().unwrap() as u8) - ('a' as u8)
}

impl Instruction {
    fn decode(s: &str) -> Instruction {
        let c = s.split(char::is_whitespace).collect::<Vec<&str>>();

        match c[0] {
            "cpy" => {
                let num = c[1].parse();

                match num {
                    Ok(num) => Instruction::CpyV(num, register(c[2])),
                    Err(_) => Instruction::CpyR(register(c[1]), register(c[2]))
                }
            }
            "inc" => Instruction::Inc(register(c[1])),
            "dec" => Instruction::Dec(register(c[1])),
            "jnz" => {
                let num = c[1].parse::<i32>();
                match num {
                    Ok(num) if num != 0 => Instruction::JmpA(c[2].parse().unwrap()),
                    Ok(_) => Instruction::JmpA(1),
                    Err(_) => Instruction::JmpR(register(c[1]) , c[2].parse().unwrap())
                }
            }
            _ => panic!()
        }
    }
}

struct CPU {
    registers: [i32; 4],
    instructions: Vec<Instruction>
}

impl CPU {
    fn execute(&mut self) -> i32 {
        let mut pc = 0;

        while pc < self.instructions.len() {
            let ref instr = self.instructions[pc];

            pc += 1;

            match *instr {
                Instruction::CpyV(value, reg) => self.registers[reg as usize] = value,
                Instruction::CpyR(regs, regt) => self.registers[regt as usize] = self.registers[regs as usize],
                Instruction::Inc(reg) => self.registers[reg as usize] += 1,
                Instruction::Dec(reg) => self.registers[reg as usize] -= 1,
                Instruction::JmpA(target) => { pc = (pc as i32 + target - 1) as usize; }
                Instruction::JmpR(reg, target) => {
                    if self.registers[reg as usize] != 0 {
                        pc = (pc as i32 + target - 1) as usize;
                    }
                }
            }
        }

        self.registers[0]
    }
}

fn main() {
    let file = BufReader::new(File::open("input-12.txt").unwrap());
    let mut cpu = CPU{ registers: [0; 4], instructions: Vec::new() };

    for line in file.lines() {
        let line = line.unwrap();
        cpu.instructions.push(Instruction::decode(&line));
    }

    println!("A: register a: {}", cpu.execute());
    cpu.registers = [0, 0, 1, 0];
    println!("B: register a: {}", cpu.execute());

}
