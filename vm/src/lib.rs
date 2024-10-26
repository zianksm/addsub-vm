pub mod opcode;
pub mod stack;

use opcode::{ Get, Opcode, OpcodeExecutor };
use stack::Stack;

pub struct Vm<Exec = Stack> {
    stack: Exec,
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
            _ => unimplemented!(),
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
}
