pub mod opcode;
pub mod stack;

use opcode::{ Get, Opcode, OpcodeExecutor };
use stack::DefaultStack;

pub struct Vm<Stack = DefaultStack> {
    stack: Stack,
    pc: usize,
    opcodes: Vec<Opcode>,
}

impl<Exec> Vm<Exec> where Exec: OpcodeExecutor + Get<Vec<u128>> + Default {
    pub fn new(opcodes: Vec<Opcode>) -> Self {
        Self {
            stack: Exec::default(),
            pc: 0,
            opcodes,
        }
    }

    fn exec_opcode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::PUSH(lit) => self.stack.push(lit),
            Opcode::ADD => self.stack.add(),
            Opcode::SUB => self.stack.sub(),
            Opcode::POP => self.stack.pop(),
            Opcode::DIV => self.stack.div(),
            Opcode::MUL => self.stack.mul(),
            Opcode::MODU => self.stack.modu(),
        }
    }

    pub fn run(mut self) -> Option<Vec<u128>> {
        while self.pc < self.opcodes.len() {
            let opcode = self.opcodes[self.pc];
            self.exec_opcode(opcode);
            self.pc += 1;
        }

        Some(self.stack.get())
    }
}

#[macro_export]
macro_rules! exec_inline {
    ($($opcode:ident $(($lit:literal))?),*) => {
        {        
        let mut vm = Vm::<stack::Stack>::new(vec![$(Opcode::$opcode $(($lit))?),*]);
        let result = vm.run().unwrap();
        result
        }
    };
    () => {

    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembly() {
        let result = exec_inline!(PUSH(1), PUSH(2), ADD, PUSH(3), SUB);
        assert_eq!(result, Into::<Vec<u128>>::into(vec![0]));
    }

    #[test]
    fn test_mul() {
        let result = exec_inline!(PUSH(5), PUSH(5), MUL);
        let result = result.get(0).unwrap().to_owned();
        assert_eq!(result, 25)
    }

    #[test]
    fn test_div() {
        let result = exec_inline!(PUSH(25), PUSH(5), DIV);
        let result = result.get(0).unwrap().to_owned();
        assert_eq!(result, 5)
    }

    #[test]
    fn test_mod() {
        let result = exec_inline!(PUSH(25), PUSH(2), MODU);
        let result = result.get(0).unwrap().to_owned();
        assert_eq!(result, 1)
    }

    #[test]
    fn test_combine() {
        let result = exec_inline!(PUSH(25), PUSH(2), MUL, PUSH(10), DIV, PUSH(20), ADD);
    }
}
