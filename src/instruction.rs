
pub struct Instruction {
    pub op: u8,
    pub a: usize,
    pub b: usize
}

impl Instruction {
    pub fn new(opcode:u8, operandA:usize, operandB:usize) -> Instruction {
        Instruction {
            op: opcode,
            a: operandA,
            b: operandB
        }
    }

    pub fn clone(&self) -> Instruction {
        Instruction {
            op: self.op.clone(),
            a: self.a.clone(),
            b: self.b.clone()
        }
    }

    pub fn set_a(&mut self, a: usize) {
        self.a = a;
    }

    pub fn set_b(&mut self, b: usize) {
        self.b = b;
    }

    pub fn set_opcode(&mut self, opcode: u8) {
        self.op = opcode;
    }

    pub fn get_a(&self) -> usize {
        return self.a;
    }

    pub fn get_b(&self) -> usize {
        return self.b;
    }

    pub fn get_opcode(&self) -> u8 {
        return self.op;
    }
}
