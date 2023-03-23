#[derive(Clone, Debug)]
pub struct Intcode {
    program: Vec<usize>,
}

impl Intcode {
    pub fn new(input: &[usize]) -> Self {
        Intcode {
            program: input.to_vec(),
        }
    }

    pub fn execute(&mut self) -> usize {
        let mut pc = 0;

        while pc < self.program.len() {
            let instruction = self.program[pc];
            // 99 is halt
            if instruction == 99 {
                break;
            }

            // load the next 3 registers
            let &[a, b, c] = &self.program[pc + 1..pc + 4] else {
                panic!("Couldn't load the 3 registers after {instruction}")
            };

            // get the values at a and b
            let a = self.program[a];
            let b = self.program[b];

            match instruction {
                1 => {
                    if let Some(result) = self.program.get_mut(c) {
                        *result = a + b;
                    }
                }
                2 => {
                    if let Some(result) = self.program.get_mut(c) {
                        *result = a * b;
                    }
                }
                _ => unreachable!("The opcode {instruction} hasn't been implemented"),
            }

            pc += 4;
        }

        self.program[0]
    }

    pub fn set_inputs(&mut self, arg_1: usize, arg_2: usize) {
        if let Some(x) = self.program.get_mut(1) {
            *x = arg_1;
        }
        if let Some(x) = self.program.get_mut(2) {
            *x = arg_2;
        }
    }
}
