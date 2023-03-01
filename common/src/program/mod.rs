pub mod parsing;
pub mod registers;

use std::fmt::Display;

use crate::program::registers::Registers;

#[derive(Clone)]
pub struct Program<I: Display, D = ()> {
    pub instructions: Vec<I>,
    pub counter: i64,
    pub registers: Registers,
    pub data: D,
}

impl<I, D> Program<I, D>
where
    I: Display,
{
    pub fn new(instructions: Vec<I>) -> Program<I> {
        Program {
            instructions,
            counter: 0,
            registers: Registers::all_zero(),
            data: (),
        }
    }

    pub fn with_data(instructions: Vec<I>, data: D) -> Program<I, D> {
        Program {
            instructions,
            counter: 0,
            registers: Registers::all_zero(),
            data,
        }
    }

    pub fn get(&self, idx: i64) -> Option<&I> {
        self.instructions.get(idx as usize)
    }

    pub fn get_mut(&mut self, idx: i64) -> Option<&mut I> {
        self.instructions.get_mut(idx as usize)
    }

    pub fn current(&self) -> Option<&I> {
        self.get(self.counter)
    }

    // pub fn run<F>(&mut self, mut f: F)
    // where
    //     F: FnMut(&I) -> i64,
    // {
    //     while let Some(instruction) = self.current() {
    //         self.counter += f(instruction, self.registers);
    //     }
    // }
}

impl<I: Display, D> Display for Program<I, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, x) in self.instructions.iter().enumerate() {
            let pointer = if i == self.counter as usize { ">" } else { " " };
            writeln!(f, "{}{x}", pointer)?;
        }
        Ok(())
    }
}
