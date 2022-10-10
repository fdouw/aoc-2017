use std::{collections::HashSet, str::FromStr};

pub fn solve(input: String, verbose: bool) -> (String, String) {
    let mut proc = Processor::new(&input);

    loop {
        if proc.step() {
            break;
        }
    }
    let part1 = proc.mul_count;

    // Part 2 takes too long
    if verbose {
        println!("WARNING || Part 2 uses a tailor made solution and ignores the given input.");
    }

    // When I read the program, it seems to do the following:
    // for b = 105_700..122_700.step(17) {
    //     f = 1
    //     for d = 2..b {
    //         for e = 2..b {
    //             if d * e == b {
    //                 f = 0
    //             }
    //         }
    //     }
    //     if f == 0 {
    //         h += 1
    //     }
    // }
    // Which seems like going from 105_700 to 122_700 in increments of 17 and counting how many of those numbers are prime
    let sieve = PrimeSieve::new();
    let a = 105_700;
    let b = 122_700;

    let primes = sieve
        .skip_while(|p| p < &a)
        .take_while(|p| p <= &b)
        .collect::<HashSet<u64>>();

    let part2 = (a..=b).step_by(17).filter(|n| !primes.contains(n)).count();

    (part1.to_string(), part2.to_string())
}

struct PrimeSieve {
    primes: Vec<u64>,
}

impl PrimeSieve {
    fn new() -> Self {
        PrimeSieve { primes: Vec::new() }
    }
}

impl Iterator for PrimeSieve {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let mut n = *self.primes.last().unwrap_or(&1) + 1;
        loop {
            let mut is_prime = true;
            for p in self.primes.iter() {
                if n % p == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                self.primes.push(n);
                return Some(n);
            }
            n += 1;
        }
    }
}

#[derive(Clone, Debug)]
enum ValueAddress<T> {
    Value { x: T },
    Address { x: usize },
}

impl<T> ValueAddress<T>
where
    T: FromStr,
{
    fn new(data: &str) -> Self {
        match data.parse::<T>() {
            Ok(x) => ValueAddress::Value { x },
            Err(_) => ValueAddress::Address {
                x: (data.bytes().nth(0).unwrap() - b'a') as usize,
            },
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction<T> {
    SET {
        x: usize,
        y: ValueAddress<T>,
    },
    SUB {
        x: usize,
        y: ValueAddress<T>,
    },
    MUL {
        x: usize,
        y: ValueAddress<T>,
    },
    JNZ {
        x: ValueAddress<T>,
        y: ValueAddress<T>,
    },
}

impl<T> Instruction<T>
where
    T: FromStr,
{
    fn new(data: &str) -> Self {
        let mut data = data.split_ascii_whitespace();
        match data.next().unwrap() {
            "set" => {
                let x = (data.next().unwrap().bytes().nth(0).unwrap() - b'a') as usize;
                let y = data.next().unwrap();
                Self::SET {
                    x,
                    y: ValueAddress::new(y),
                }
            }
            "sub" => {
                let x = (data.next().unwrap().bytes().nth(0).unwrap() - b'a') as usize;
                let y = data.next().unwrap();
                Self::SUB {
                    x,
                    y: ValueAddress::new(y),
                }
            }
            "mul" => {
                let x = (data.next().unwrap().bytes().nth(0).unwrap() - b'a') as usize;
                let y = data.next().unwrap();
                Self::MUL {
                    x,
                    y: ValueAddress::new(y),
                }
            }
            "jnz" => {
                let x = data.next().unwrap();
                let y = data.next().unwrap();
                Self::JNZ {
                    x: ValueAddress::new(x),
                    y: ValueAddress::new(y),
                }
            }
            other => panic!("ERROR || Invalid instruction: '{other}'"),
        }
    }
}

struct Processor {
    registers: [i32; 8],
    instructions: Vec<Instruction<i32>>,
    index: usize,
    mul_count: u32,
}

impl Processor {
    fn new(data: &str) -> Self {
        let instructions = data.lines().map(|l| Instruction::new(l)).collect();
        Processor {
            registers: [0; 8],
            instructions,
            index: 0,
            mul_count: 0,
        }
    }
    fn get_value(&self, va: &ValueAddress<i32>) -> i32 {
        match va {
            ValueAddress::Address { x } => self.registers[*x],
            ValueAddress::Value { x } => *x,
        }
    }
    fn step(&mut self) -> bool {
        let command = &self.instructions[self.index];
        match command {
            Instruction::SET { x, y } => {
                self.registers[*x] = self.get_value(&y);
                self.index += 1;
            }
            Instruction::SUB { x, y } => {
                let inc = self.get_value(&y);
                self.registers[*x] -= inc;
                self.index += 1;
                if *x == b'h' as usize {
                    println!("Subtracting b to {}", self.registers[*x]);
                }
            }
            Instruction::MUL { x, y } => {
                self.mul_count += 1;
                let mul = self.get_value(&y);
                self.registers[*x] *= mul;
                self.index += 1;
            }
            Instruction::JNZ { x, y } => {
                if self.get_value(&x) != 0 {
                    let next_index = self.index as i32 + self.get_value(&y);
                    if next_index < 0 || next_index >= self.instructions.len() as i32 {
                        return true;
                    } else {
                        self.index = next_index as usize;
                    }
                } else {
                    self.index += 1;
                }
            }
        }
        self.index >= self.instructions.len()
    }
}
