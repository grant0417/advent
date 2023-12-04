#[derive(Debug, Clone)]
pub struct IntcodeI {
    memory: Vec<u32>,
    pc: usize,
}

impl IntcodeI {
    pub fn new(input: &str) -> Self {
        IntcodeI {
            memory: input
                .trim()
                .split(',')
                .map(|c| c.parse::<u32>().unwrap())
                .collect(),
            pc: 0,
        }
    }

    pub fn interpret(&mut self) -> Option<&mut Self> {
        self.pc = 0;
        while self.step()? {
            // println!("{self:?}");
        }
        Some(self)
    }

    pub fn step(&mut self) -> Option<bool> {
        match self.memory[self.pc] {
            // Add
            1 => {
                let a = self.param(1)?;
                let b = self.param(2)?;
                *self.param_mut(3)? = a + b;
                self.pc += 4;
            }
            // Mul
            2 => {
                let a = self.param(1)?;
                let b = self.param(2)?;
                *self.param_mut(3)? = a * b;
                self.pc += 4;
            }
            // Hlt
            99 => {
                return Some(false);
            }
            op => panic!("Unimplemented opcode {op}"),
        }

        Some(self.pc < self.memory.len())
    }

    pub fn memory(&self) -> &[u32] {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut [u32] {
        &mut self.memory
    }

    fn param(&self, offset: usize) -> Option<&u32> {
        let addr = *self.memory.get(self.pc + offset)? as usize;
        self.memory.get(addr)
    }

    fn param_mut(&mut self, offset: usize) -> Option<&mut u32> {
        let addr = *self.memory.get(self.pc + offset)? as usize;
        self.memory.get_mut(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(
            IntcodeI::new("1,9,10,3,2,3,11,0,99,30,40,50")
                .interpret()
                .unwrap()
                .memory(),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        assert_eq!(
            IntcodeI::new("1,0,0,0,99").interpret().unwrap().memory(),
            [2, 0, 0, 0, 99]
        );

        assert_eq!(
            IntcodeI::new("2,3,0,3,99").interpret().unwrap().memory(),
            [2, 3, 0, 6, 99]
        );

        assert_eq!(
            IntcodeI::new("2,4,4,5,99,0").interpret().unwrap().memory(),
            [2, 4, 4, 5, 99, 9801]
        );

        assert_eq!(
            IntcodeI::new("1,1,1,4,99,5,6,0,99")
                .interpret()
                .unwrap()
                .memory(),
            [30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
