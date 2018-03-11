
pub struct Instruction {
    pub op: u8,
    pub a: u8,
    pub b: u8
}

impl Instruction {
    pub fn new(opcode:u8, operandA:u8, operandB:u8) -> Instruction {
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

    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }

    pub fn set_opcode(&mut self, opcode: u8) {
        self.op = opcode;
    }

    pub fn get_a(&self) -> u8 {
        return self.a;
    }

    pub fn get_b(&self) -> u8 {
        return self.b;
    }

    pub fn get_opcode(&self) -> u8 {
        return self.op;
    }

    pub fn to_string(&self) -> String {
        match self.op {
            0 => { return format!("v{} = v{}", self.a, self.b) }

            1 => { return format!("v{} += v{}", self.a, self.b) }
            2 => { return format!("v{} -= v{}", self.a, self.b) }
            3 => { return format!("v{} *= v{} if not overflow", self.a, self.b) }
            4 => { return format!("v{} /= v{} if divisible", self.a, self.b)}
            5 => { return format!("v{} = v{}^v{} if not overflow", self.a, self.a, self.b)}

            6 => { return format!("v{} ++", self.a) }
            7 => { return format!("v{} --", self.a) }
            8 => { return format!("v{} = 0", self.a) }
            9 => { return format!("v{} = 1", self.a) }
            10 => { return format!("v{} = 5", self.a) }
            11 => { return format!("v{} = v{}^2", self.a, self.a) }
            12 => { return format!("sqrt v{}", self.a) }
            13 => { return format!("abs v{}", self.a) }
            14 => { return format!("log2 v{}", self.a) }
            15 => { return format!("sin v{}", self.a) }
            16 => { return format!("cos v{}", self.a) }
            17 => { return format!("tan v{}", self.a) }
            18 => { return format!("v{} = PI", self.a) } //f32::consts::PI

            19 => { return format!("if v{} != v{}", self.a, self.b) } //if self.vars[self.a] == self.vars[self.b] { line+=1 } }
            20 => { return format!("if v{} == v{}", self.a, self.b) } //if self.vars[self.a] != self.vars[self.b] { line+=1 } }
            21 => { return format!("if v{} >= v{}", self.a, self.b) } //if self.vars[self.a] < self.vars[self.b] { line+=1 } }
            22 => { return format!("if v{} > v{}", self.a, self.b) } //if self.vars[self.a] <= self.vars[self.b] { line+=1 } }
            23 => { return format!("if v{} <= v{}", self.a, self.b) }   //if self.vars[self.a] > self.vars[self.b] { line+=1 } }
            24 => { return format!("if v{} < v{}", self.a, self.b) } //if self.vars[ins.a] >= self.vars[ins.b] { line+=1 } }

            25 => { return format!("push v{}", self.a) }
            26 => { return format!("pop  v{}", self.a) }

            27 => { return format!("nop") }
            _ => { return format!("err") }
        }
    }
}
