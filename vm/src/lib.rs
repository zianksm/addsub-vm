pub mod opcode;
pub mod stack;

use opcode::Opcode;
use stack::Stack;

pub struct Vm {
    stack: stack::Stack,
    pc: usize,
    opcodes: Vec<Opcode>,
}

impl Vm {
    pub fn new(opcodes: Vec<Opcode>) -> Self {
        Self {
            stack: Stack::new(),
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
        }
    }

    pub fn run(mut self) -> Option<Stack> {
        while self.pc < self.opcodes.len() {
            let opcode = self.opcodes[self.pc];
            self.exec_opcode(opcode);
            self.pc += 1;
        }

        Some(self.stack.clone())
    }
}

#[macro_export(inner)]
macro_rules! exec_inline {
    ($($opcode:ident $(($lit:literal))?),*) => {
        {        
        let mut vm = Vm::new(vec![$(Opcode::$opcode $(($lit))?),*]);
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
        assert_eq!(result, vec![0].into());
    }
}
