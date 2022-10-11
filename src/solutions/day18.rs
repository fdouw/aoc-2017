use std::{collections::HashMap, str::FromStr, sync::mpsc};

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    #[allow(unused_variables)]
    let test_input = "snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d".to_string();
    let mut tablet = Tablet::new(&input);
    let part1 = tablet.run_to_recover();

    let (txa, rxa) = mpsc::channel();
    let (txb, rxb) = mpsc::channel();
    let mut tablet_0 = MultiTablet::new(&input, 0, rxa, txb);
    let mut tablet_1 = MultiTablet::new(&input, 1, rxb, txa);

    while tablet_0.state == State::Running || tablet_1.state == State::Running {
        tablet_0.step();
        tablet_1.step();
    }
    let part2 = tablet_1.send_count;

    (part1.to_string(), part2.to_string())
}

#[derive(Clone, Debug)]
enum ValueAddress<T> {
    Value { x: T },
    Address { x: String },
}

impl<T> ValueAddress<T>
where
    T: FromStr,
{
    fn new(data: &str) -> Self {
        match data.parse::<T>() {
            Ok(x) => ValueAddress::Value { x },
            Err(_) => ValueAddress::Address {
                x: data.to_string(),
            },
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction<T> {
    SND {
        x: ValueAddress<T>,
    },
    SET {
        x: String,
        y: ValueAddress<T>,
    },
    ADD {
        x: String,
        y: ValueAddress<T>,
    },
    MUL {
        x: String,
        y: ValueAddress<T>,
    },
    MOD {
        x: String,
        y: ValueAddress<T>,
    },
    RCV {
        x: ValueAddress<T>,
    },
    JGZ {
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
            "snd" => Self::SND {
                x: ValueAddress::new(data.next().unwrap()),
            },
            "set" => {
                let x = data.next().unwrap();
                let y = data.next().unwrap();
                Self::SET {
                    x: x.to_string(),
                    y: ValueAddress::new(y),
                }
            }
            "add" => {
                let x = data.next().unwrap();
                let y = data.next().unwrap();
                Self::ADD {
                    x: x.to_string(),
                    y: ValueAddress::new(y),
                }
            }
            "mul" => {
                let x = data.next().unwrap();
                let y = data.next().unwrap();
                Self::MUL {
                    x: x.to_string(),
                    y: ValueAddress::new(y),
                }
            }
            "mod" => {
                let x = data.next().unwrap();
                let y = data.next().unwrap();
                Self::MOD {
                    x: x.to_string(),
                    y: ValueAddress::new(y),
                }
            }
            "rcv" => {
                let x = data.next().unwrap();
                Self::RCV {
                    x: ValueAddress::new(x),
                }
            }
            "jgz" => {
                let x = data.next().unwrap();
                let y = data.next().unwrap();
                Self::JGZ {
                    x: ValueAddress::new(x),
                    y: ValueAddress::new(y),
                }
            }
            other => panic!("ERROR || Invalid instruction: '{other}'"),
        }
    }
}

#[derive(PartialEq)]
enum State {
    Running,
    Waiting,
    Stopped,
}

// Tablet as used in part 2
struct MultiTablet {
    registers: HashMap<String, i64>,
    instructions: Vec<Instruction<i64>>,
    index: usize,
    receiver: mpsc::Receiver<i64>,
    sender: mpsc::Sender<i64>,
    send_count: u32,
    state: State,
    // id: i64,
}

impl<'a> MultiTablet {
    fn new(
        data: &String,
        id: i64,
        receiver: mpsc::Receiver<i64>,
        sender: mpsc::Sender<i64>,
    ) -> Self {
        let instructions = data.lines().map(|l| Instruction::new(l)).collect();
        let mut registers = HashMap::new();
        registers.insert("p".to_string(), id);
        MultiTablet {
            registers,
            instructions,
            index: 0,
            receiver,
            sender,
            send_count: 0,
            state: State::Running,
            // id,
        }
    }
    fn get_value(&self, va: &ValueAddress<i64>) -> i64 {
        match va {
            ValueAddress::Address { x } => self.registers[x],
            ValueAddress::Value { x } => *x,
        }
    }
    fn step(&mut self) {
        // Do nothing if state is stopped;
        // if state is waiting, we should still be at the RCV instruction
        if self.state == State::Stopped {
            return;
        }

        let command = &self.instructions[self.index];
        // println!("Tablet ({}): {:?}", self.id, command);
        match command {
            Instruction::SND { x } => {
                let val = self.get_value(x);
                self.sender.send(val).unwrap();
                self.send_count += 1;
                self.index += 1;
            }
            Instruction::SET { x, y } => {
                self.registers.insert(x.clone(), self.get_value(y));
                self.index += 1;
            }
            Instruction::ADD { x, y } => {
                let old = *self.registers.get(x).unwrap_or(&0);
                let inc = self.get_value(y);
                self.registers.insert(x.clone(), old + inc);
                self.index += 1;
            }
            Instruction::MUL { x, y } => {
                let old = *self.registers.get(x).unwrap_or(&0);
                let mul = self.get_value(y);
                self.registers.insert(x.clone(), old * mul);
                self.index += 1;
            }
            Instruction::MOD { x, y } => {
                let old = *self.registers.get(x).unwrap_or(&0);
                let div = self.get_value(y);
                self.registers.insert(x.clone(), old % div);
                self.index += 1;
            }
            Instruction::RCV { x } => {
                let val = match self.receiver.try_recv() {
                    Ok(v) => {
                        self.state = State::Running;
                        v
                    }
                    Err(_) => {
                        // Next step we will try to read again; return from execution here and don't update index
                        self.state = State::Waiting;
                        return;
                    }
                };
                match x {
                    ValueAddress::Address { x } => {
                        self.registers.insert(x.clone(), val);
                    }
                    ValueAddress::Value { x: _x } => {
                        panic!("ERROR || Expected an address, but found a value");
                    }
                }
                self.index += 1;
            }
            Instruction::JGZ { x, y } => {
                if self.get_value(&x) > 0 {
                    let next_index = self.index as i64 + self.get_value(&y);
                    if next_index < 0 || next_index >= self.instructions.len() as i64 {
                        self.state = State::Stopped;
                    } else {
                        self.index = next_index as usize;
                    }
                } else {
                    self.index += 1;
                }
            }
        }
    }
}

// Tablet as used in part 1
struct Tablet {
    registers: HashMap<String, i64>,
    instructions: Vec<Instruction<i64>>,
    index: usize,
    last_sound: i64,
}

impl<'a> Tablet {
    fn new(data: &String) -> Self {
        let instructions = data.lines().map(|l| Instruction::new(l)).collect();
        Tablet {
            registers: HashMap::new(),
            instructions,
            index: 0,
            last_sound: 0,
        }
    }
    fn get_value(&self, va: &ValueAddress<i64>) -> i64 {
        match va {
            ValueAddress::Address { x } => self.registers[x],
            ValueAddress::Value { x } => *x,
        }
    }
    fn step(&mut self) -> Option<Instruction<i64>> {
        let command = &self.instructions[self.index];
        match command {
            Instruction::SND { x } => {
                self.last_sound = self.get_value(&x);
                self.index += 1;
            }
            Instruction::SET { x, y } => {
                self.registers.insert(x.clone(), self.get_value(&y));
                self.index += 1;
            }
            Instruction::ADD { x, y } => {
                let old = *self.registers.get(x).unwrap_or(&0);
                let inc = self.get_value(&y);
                self.registers.insert(x.clone(), old + inc);
                self.index += 1;
            }
            Instruction::MUL { x, y } => {
                let old = *self.registers.get(x).unwrap_or(&0);
                let mul = self.get_value(&y);
                self.registers.insert(x.clone(), old * mul);
                self.index += 1;
            }
            Instruction::MOD { x, y } => {
                let old = *self.registers.get(x).unwrap_or(&0);
                let div = self.get_value(&y);
                self.registers.insert(x.clone(), old % div);
                self.index += 1;
            }
            Instruction::RCV { x: _x } => {
                // basically a no-op, because the caller has to handle the recovery
                self.index += 1;
            }
            Instruction::JGZ { x, y } => {
                if self.get_value(&x) > 0 {
                    let next_index = self.index as i64 + self.get_value(&y);
                    if next_index < 0 || next_index >= self.instructions.len() as i64 {
                        return None;
                    } else {
                        self.index = next_index as usize;
                    }
                } else {
                    self.index += 1;
                }
            }
        }
        if self.index >= self.instructions.len() {
            None
        } else {
            Some(command.clone())
        }
    }
    fn run_to_recover(&'a mut self) -> i64 {
        while let Some(cmd) = self.step() {
            if let Instruction::RCV { x } = cmd {
                if self.get_value(&x) != 0 {
                    return self.last_sound;
                }
            }
        }
        panic!("ERROR || Program ended without recovery!");
    }
}
