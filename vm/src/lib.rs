pub mod opcode;
use opcode::Opcode;

pub struct Vm {
    stack: Vec<u128>,
    pc: usize,
    opcodes: Vec<Opcode>,
}

impl Vm {
    pub fn new(opcodes: Vec<Opcode>) -> Self {
        Self {
            stack: Vec::new(),
            pc: 0,
            opcodes,
        }
    }

    fn stack_sub(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b - a);
    }

    fn stack_add(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b + a);
    }

    fn stack_push(&mut self, value: u128) {
        self.stack.push(value);
    }

    fn stack_pop(&mut self) {
        self.stack.pop();
    }

    fn exec_opcode(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::ADD => self.stack_add(),
            Opcode::SUB => self.stack_sub(),
            Opcode::PUSH(value) => self.stack_push(value),
            Opcode::POP => self.stack_pop(),
        }
    }

    pub fn run(mut self) -> Option<Vec<u128>> {
        while self.pc < self.opcodes.len() {
            let opcode = self.opcodes[self.pc];
            self.exec_opcode(opcode);
            self.pc += 1;
        }

        Some(self.stack.clone())
    }
}

macro_rules! asesmbly {
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
        let result = asesmbly!(PUSH(1), PUSH(2), ADD, PUSH(3), SUB);
        assert_eq!(result, vec![0]);
    }
}
