use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Debug)]
pub struct Instruction<T> {
    pub opcode: T,
    pub input_a: usize,
    pub input_b: usize,
    pub output: usize,
}

impl Instruction<usize> {
    pub fn execute(&self, mapping: &HashMap<usize, Opcode>, registers: &[usize]) -> Vec<usize> {
        let Some(opcode) = mapping.get(&self.opcode) else {
            panic!("Couldn't find opcode {}", self.opcode);
        };
        self.inner_execute(*opcode, registers)
    }
}

impl Instruction<Opcode> {
    pub fn new(opcode: Opcode) -> impl Fn(usize, usize, usize) -> Instruction<Opcode> {
        move |input_a, input_b, output| Instruction {
            opcode,
            input_a,
            input_b,
            output,
        }
    }

    pub fn execute(&self, registers: &[usize]) -> Vec<usize> {
        self.inner_execute(self.opcode, registers)
    }
}

impl<T> Instruction<T> {
    fn inner_execute(&self, opcode: Opcode, registers: &[usize]) -> Vec<usize> {
        match opcode {
            Opcode::Addr => self.addr(registers),
            Opcode::Addi => self.addi(registers),
            Opcode::Mulr => self.mulr(registers),
            Opcode::Muli => self.muli(registers),
            Opcode::Banr => self.banr(registers),
            Opcode::Bani => self.bani(registers),
            Opcode::Borr => self.borr(registers),
            Opcode::Bori => self.bori(registers),
            Opcode::Setr => self.setr(registers),
            Opcode::Seti => self.seti(registers),
            Opcode::Gtir => self.gtir(registers),
            Opcode::Gtri => self.gtri(registers),
            Opcode::Gtrr => self.gtrr(registers),
            Opcode::Eqir => self.eqir(registers),
            Opcode::Eqri => self.eqri(registers),
            Opcode::Eqrr => self.eqrr(registers),
        }
    }
    pub fn addr(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = registers[self.input_b];
        registers[self.output] = a + b;

        registers
    }

    pub fn addi(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = self.input_b;
        registers[self.output] = a + b;

        registers
    }

    pub fn mulr(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = registers[self.input_b];
        registers[self.output] = a * b;

        registers
    }

    pub fn muli(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = self.input_b;
        registers[self.output] = a * b;

        registers
    }

    pub fn banr(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = registers[self.input_b];
        registers[self.output] = a & b;

        registers
    }

    pub fn bani(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = self.input_b;
        registers[self.output] = a & b;

        registers
    }

    pub fn borr(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = registers[self.input_b];
        registers[self.output] = a | b;

        registers
    }

    pub fn bori(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = self.input_b;
        registers[self.output] = a | b;

        registers
    }

    pub fn setr(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        registers[self.output] = a;

        registers
    }

    pub fn seti(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = self.input_a;
        registers[self.output] = a;

        registers
    }

    pub fn gtir(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = self.input_a;
        let b = registers[self.input_b];
        registers[self.output] = usize::from(a > b);

        registers
    }

    pub fn gtri(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = self.input_b;
        registers[self.output] = usize::from(a > b);

        registers
    }

    pub fn gtrr(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = registers[self.input_b];
        registers[self.output] = usize::from(a > b);

        registers
    }

    pub fn eqir(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = self.input_a;
        let b = registers[self.input_b];
        registers[self.output] = usize::from(a == b);

        registers
    }

    pub fn eqri(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = self.input_b;
        registers[self.output] = usize::from(a == b);

        registers
    }

    pub fn eqrr(&self, registers: &[usize]) -> Vec<usize> {
        let mut registers = registers.to_vec();
        let a = registers[self.input_a];
        let b = registers[self.input_b];
        registers[self.output] = usize::from(a == b);

        registers
    }
}
