use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Literal(i64),
    Register(Register),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(x) => write!(f, "{x}"),
            Value::Register(x) => write!(f, "{x}"),
        }
    }
}

pub type Register = char;

#[derive(Clone)]
pub struct Registers {
    pub registers: HashMap<char, i64>,
}

impl Registers {
    pub fn all_zero() -> Self {
        let mut registers = HashMap::new();
        for r in 'a'..='z' {
            registers.insert(r, 0);
        }

        Registers { registers }
    }
    pub fn new(a: i64) -> Self {
        let mut registers = HashMap::new();
        registers.insert('a', a);
        registers.insert('b', 0);
        registers.insert('c', 0);
        registers.insert('d', 0);

        Registers { registers }
    }

    pub fn add(&mut self, c: char, i: i64) {
        self.registers.entry(c).and_modify(|x| *x += i);
    }

    pub fn mul(&mut self, c: char, i: i64) {
        self.registers.entry(c).and_modify(|x| *x *= i);
    }

    pub fn set(&mut self, c: char, i: i64) {
        self.registers.entry(c).and_modify(|c| *c = i);
    }

    pub fn clear(&mut self, c: char) {
        self.registers.entry(c).and_modify(|c| *c = 0);
    }

    pub fn entry(&mut self, c: char) -> std::collections::hash_map::Entry<char, i64> {
        self.registers.entry(c)
    }

    pub fn resolve(&self, v: Value) -> i64 {
        match v {
            Value::Literal(x) => x,
            Value::Register(r) => *self.registers.get(&r).unwrap(),
        }
    }
}
