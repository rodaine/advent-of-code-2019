use crate::read_to_string;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct IntCodeComputer {
    idx: usize,
    input: i64,
    data: Vec<i64>,
}

impl IntCodeComputer {
    pub fn new(data: Vec<i64>, input: i64) -> Self {
        Self {
            idx: 0,
            data,
            input,
        }
    }

    pub fn from_input_file(name: &str, input: i64) -> Self {
        let data = read_to_string(name)
            .split(',')
            .map(|a| a.parse::<i64>())
            .map(|a| a.expect("failed to parse address"))
            .collect();

        Self::new(data, input)
    }

    pub fn run(&mut self) { self.last(); }

    fn len(&self) -> usize { self.data.len() }
}

impl Iterator for IntCodeComputer {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        use Op::*;
        use Mode::*;

        while self.idx < self.len() {
            let idx = self.idx;
            let data = &mut self.data;
            let op = Op::from(data[idx]);

            match op {
                Add(l, r, Position) => {
                    let (lhs, rhs, target) = (data[idx + 1], data[idx + 2], data[idx + 3]);
                    let left = if l == Position { data[lhs as usize] } else { lhs };
                    let right = if r == Position { data[rhs as usize] } else { rhs };
                    data[target as usize] = left + right;
                }
                Multiply(l, r, Position) => {
                    let (lhs, rhs, target) = (data[idx + 1], data[idx + 2], data[idx + 3]);
                    let left = if l == Position { data[lhs as usize] } else { lhs };
                    let right = if r == Position { data[rhs as usize] } else { rhs };
                    data[target as usize] = left * right;
                }
                Input(Position) => {
                    let address = data[idx + 1] as usize;
                    data[address] = self.input;
                }
                Output(mode) => {
                    let value = data[idx + 1];
                    let output = if mode == Position { data[value as usize] } else { value };
                    self.idx += op.len();
                    return Some(output);
                }
                JumpIfTrue(l, r) => {
                    let (first, second) = (data[idx + 1], data[idx + 2]);
                    let check = if l == Position { data[first as usize] } else { first };

                    if check == 0 {
                        self.idx += 3;
                    } else {
                        let pos = if r == Position { data[second as usize] } else { second };
                        self.idx = pos as usize
                    }
                }
                JumpIfFalse(l, r) => {
                    let (first, second) = (data[idx + 1], data[idx + 2]);
                    let check = if l == Position { data[first as usize] } else { first };

                    if check != 0 {
                        self.idx += 3;
                    } else {
                        let pos = if r == Position { data[second as usize] } else { second };
                        self.idx = pos as usize
                    }
                }
                LessThan(l, r, Position) => {
                    let (lhs, rhs, target) = (data[idx + 1], data[idx + 2], data[idx + 3]);
                    let left = if l == Position { data[lhs as usize] } else { lhs };
                    let right = if r == Position { data[rhs as usize] } else { rhs };
                    data[target as usize] = if left < right { 1 } else { 0 }
                }
                Equals(l, r, Position) => {
                    let (lhs, rhs, target) = (data[idx + 1], data[idx + 2], data[idx + 3]);
                    let left = if l == Position { data[lhs as usize] } else { lhs };
                    let right = if r == Position { data[rhs as usize] } else { rhs };
                    data[target as usize] = if left == right { 1 } else { 0 }
                }
                Halt => {
                    self.idx = self.len();
                    return None;
                }
                _ => panic!("invalid operation at idx {}: {:?}", idx, op),
            }
            self.idx += op.len();
        }

        None
    }
}

impl Index<usize> for IntCodeComputer {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output { &self.data[index] }
}

impl IndexMut<usize> for IntCodeComputer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.data[index] }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Op {
    Add(Mode, Mode, Mode),
    Multiply(Mode, Mode, Mode),
    Input(Mode),
    Output(Mode),
    Halt,
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
}

impl Op {
    fn len(self) -> usize {
        use Op::*;
        match self {
            Add(_, _, _) => 4,
            Multiply(_, _, _) => 4,
            Input(_) => 2,
            Output(_) => 2,
            Halt => 1,
            JumpIfTrue(_, _) => 0, // manually jump
            JumpIfFalse(_, _) => 0, // manually jump
            LessThan(_, _, _) => 4,
            Equals(_, _, _) => 4,
        }
    }
}

impl From<i64> for Op {
    fn from(c: i64) -> Self {
        use Op::*;
        match c % 100 {
            1 => Add(Mode::first(c), Mode::second(c), Mode::third(c)),
            2 => Multiply(Mode::first(c), Mode::second(c), Mode::third(c)),
            3 => Input(Mode::first(c)),
            4 => Output(Mode::first(c)),
            5 => JumpIfTrue(Mode::first(c), Mode::second(c)),
            6 => JumpIfFalse(Mode::first(c), Mode::second(c)),
            7 => LessThan(Mode::first(c), Mode::second(c), Mode::third(c)),
            8 => Equals(Mode::first(c), Mode::second(c), Mode::third(c)),
            99 => Halt,
            _ => panic!("failed to parse operation: {}", c),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn first(op: i64) -> Self { (op % 1_000 / 100).into() }
    fn second(op: i64) -> Self { (op % 10_000/ 1_000).into() }
    fn third(op: i64) -> Self { (op % 100_000/ 10_000).into() }
}

impl From<i64> for Mode {
    fn from(i: i64) -> Self {
        use Mode::*;
        match i {
            0 => Position,
            1 => Immediate,
            _ => unreachable!(),
        }
    }
}