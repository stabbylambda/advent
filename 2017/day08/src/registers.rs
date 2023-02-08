use std::collections::HashMap;

pub type Register<'a> = &'a str;

#[derive(Debug)]
pub struct Registers<'a> {
    pub registers: HashMap<Register<'a>, i32>,
}

impl<'a> Registers<'a> {
    pub fn new() -> Self {
        Registers {
            registers: HashMap::new(),
        }
    }

    pub fn inc(&mut self, c: Register<'a>, i: i32) {
        let v = self.registers.entry(c).or_insert(0);
        *v += i;
    }

    pub fn dec(&mut self, c: Register<'a>, i: i32) {
        let v = self.registers.entry(c).or_insert(0);
        *v -= i;
    }

    pub fn get(&mut self, c: Register<'a>) -> i32 {
        *self.registers.entry(c).or_insert(0)
    }

    pub fn max(&self) -> i32 {
        *self.registers.values().max().unwrap()
    }
}

impl<'a> Default for Registers<'a> {
    fn default() -> Self {
        Self::new()
    }
}
