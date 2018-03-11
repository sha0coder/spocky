/*
    Basic & fast cpu emulator
    1.8seconds -> execute 1.000.000 instructions
    
    @sha0coder
*/


extern crate rand;

use std::vec::Vec;
use cpu::rand::Rng;
use std::fs::File;
use std::f32::{self, consts};
use instruction::Instruction;
use std::io::{Write,Read};



const NUM_OPCODES:u8 = 26;

pub struct Cpu {
    mem: Vec<f32>,
    stack: Vec<f32>,
    vars: Vec<f32>,
    code: Vec<Instruction>,
    fitness: f32,
    tv: Vec<u8>,  // tests results vector
    debugMode: bool,
    num_vars: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            mem: Vec::new(),
            stack: Vec::new(),
            vars: Vec::new(),
            code: Vec::new(),
            fitness: 0.0,
            tv: Vec::new(),
            debugMode: false,
            num_vars: 9
        }
    }

    pub fn get_num_vars(&self) -> u8 {
        if self.vars.len() == 0 {
            return self.num_vars;
        }
        return self.vars.len() as u8;
    }


    pub fn set_raw_code(&mut self, raw:&[u8]) {
        self.code.clear();
        if raw.len() % 3 != 0 {
            println!("bad raw code");
            return;
        }

        for mut i in 0..raw.len()-2 {
            let mut l  = Instruction::new(raw[i],raw[i+1],raw[i+2]);
            self.code.push(l);
            i+=2;
        }
    }

    pub fn get_raw_code(&self) -> Vec<u8> {
        let mut raw:Vec<u8> = Vec::new();

        for i in 0..self.code.len() {
            let l: &Instruction = &self.code[i];

            raw.push(l.get_opcode());
            raw.push(l.get_a());
            raw.push(l.get_b());
        }

        return raw;
    }

    pub fn save(&self, filename:&str) {
        let mut f = File::create(filename).unwrap();
        f.write(self.get_raw_code().as_slice());
    }

    pub fn load(&mut self, filename:&str) {
        let mut raw:&mut [u8] = &mut[];
        let mut f = File::open(filename).unwrap();

        f.read(raw).unwrap();
        self.set_raw_code(raw);
    }

    // tv deprepcated
    pub fn set_fitness(&mut self, fitness: f32, tv: Vec<u8>) {
        self.tv = tv;
        self.fitness = fitness;
    }

    pub fn debug(&mut self) {
        self.debugMode = true;
    }

    pub fn get_tv(&self) -> Vec<u8> {
        return self.tv.clone();
    }

    pub fn get_fitness(&self) -> f32 {
        return self.fitness;
    }


    // vars

    pub fn init_vars(&mut self, v: Vec<f32>) {
        self.vars = v;
    }

    pub fn get_var(&self, i:usize) -> f32 {
        return self.vars[i].clone();
    }

    pub fn get_code_sz(&self) -> usize {
        return self.code.len();
    }

    pub fn add_instruction(&mut self, i: Instruction) {
        self.code.push(i);
    }

    pub fn result(&self) -> f32 {
        return self.vars[0];
    }

    pub fn add(&mut self, op:u8, a:u8, b:u8) {
        // add code instruction
        self.code.push(Instruction::new(op,a,b));
    }

    pub fn instructions(&self) -> usize {
        return self.code.len();
    }

    pub fn load_instructions(&mut self, code: Vec<Instruction>) {
        self.code = code;
    }

    pub fn get_rand(&self, n:usize) -> usize {
        let mut rng = rand::thread_rng();
        return rng.gen_range::<usize>(0,n);
    }

    pub fn add_random_line(&mut self, mut rng:rand::ThreadRng) {
        let mut num_vars = self.get_num_vars().clone();
        self.add(rng.gen_range::<u8>(0,NUM_OPCODES), rng.gen_range::<u8>(0,num_vars), rng.gen_range::<u8>(0,num_vars));
    }

    pub fn randomize(&mut self, num_lines:usize) {
        // TODO: this should be called from run() if there is no code, in that moment we know the number of vars
        // Is not possible to use randomize until the variables are set.
        let mut num_vars = self.get_num_vars().clone();
        // generate a random code of n lines
        let mut rng = rand::thread_rng();
        //println!("creating");
        let mut i:usize = 0;

        self.code.clear();

        for i in 0..num_lines {
            //TODO: que sea mas probable crear una aritmetica que un if
            self.add(rng.gen_range::<u8>(0,NUM_OPCODES), rng.gen_range::<u8>(0,num_vars), rng.gen_range::<u8>(0,num_vars));
        }
        //println!("created");
    }

    pub fn init(&mut self) {
        for i in 0..self.vars.len() {
            self.vars[i] = 0.0;
        }
    }

    pub fn run(&mut self) {
        let mut line: usize = 0;
        let len: usize = self.code.len();

        //self.init();

        if len == 0 {
            self.randomize(5);
        }

        loop {

            if line >= len {
                break;
            }

            let ins: &Instruction = &self.code[line];

            if self.debugMode {
                println!("{})  [{} {} {}]", line, ins.op, ins.a, ins.b);
            
                for i in 0..self.vars.len() {
                    println!("1 var[{}] = {}",i,self.vars[i]);
                }
            }

            let a:usize = ins.a as usize;
            let b:usize = ins.b as usize;

            match ins.op {
                0 => { self.vars[a] = self.vars[b] }

                1 => { self.vars[a] += self.vars[b] }
                2 => { self.vars[a] -= self.vars[b] }
                3 => { if self.vars[a]<9999.0 && self.vars[b]<9999.0 && self.vars[a]>-9999.0 && self.vars[b]>-9999.0 { self.vars[a] *= self.vars[b] } }
                4 => { if self.vars[b] != 0.0 {self.vars[a] /= self.vars[b] }}
                5 => { if self.vars[a] < 9999.0 && self.vars[b] < 9999.0 {self.vars[a] = self.vars[a].powf(self.vars[b]) }}

                6 => {  self.vars[a] += 1.0 }
                7 => {  self.vars[a] -= 1.0 }
                8 => {  self.vars[a] = 0.0 }
                9 => {  self.vars[a] = 1.0 }
                10 => { self.vars[a] = 5.0 }
                11 => { self.vars[a] *= self.vars[a] }
                12 => { self.vars[a] = self.vars[a].sqrt() }
                13 => { self.vars[a] = self.vars[a].abs() }
                14 => { self.vars[a] = self.vars[a].log2() }
                15 => { self.vars[a] = self.vars[a].sin() }
                16 => { self.vars[a] = self.vars[a].cos() }
                17 => { self.vars[a] = self.vars[a].tan() }
                18 => { self.vars[a] = consts::PI }

                19 => { if self.vars[a] == self.vars[b] { line+=1 } }
                20 => { if self.vars[a] != self.vars[b] { line+=1 } }
                21 => { if self.vars[a] < self.vars[b] { line+=1 } }
                22 => { if self.vars[a] <= self.vars[b] { line+=1 } }
                23 => { if self.vars[a] > self.vars[b] { line+=1 } }
                24 => { if self.vars[a] >= self.vars[b] { line+=1 } }

                25 => {  self.stack.push(self.vars[a].clone()); } // push
                26 => {  self.vars[a] = self.stack.pop().unwrap(); } // pop

                _ => {;}
            }

            if self.debugMode {
                for i in 0..self.vars.len() {
                    println!("2 var[{}] = {}", i, self.vars[i]);
                }
            }

            line += 1;
       }
    }

    pub fn set_opcodes(&mut self, opcodes: Vec<u8>) {
        loop {
            if opcodes.len() == 0 {
                break;
            }

            //let mut ins = Instruction::new(opcodes.pop().unwrap(), opcodes.pop().unwrap() as usize, opcodes.pop().unwrap() as usize);
        }
    }

    pub fn get_opcodes(&self) -> Vec<u8> {
        let mut opcodes: Vec<u8> = Vec::new();

        for line in 0..self.code.len() {
            let l: &Instruction = &self.code[line];
            opcodes.push(l.get_opcode());
            opcodes.push(l.get_a() as u8);
            opcodes.push(l.get_b() as u8);
        }
        return opcodes; 
    }

    pub fn print_opcodes(&self) {
        print!("opcodes: ");
        for line in 0..self.code.len() {
            let l: &Instruction = &self.code[line];
            print!("[{},{},{}] ", l.get_opcode(), l.get_a(), l.get_b());
        } 
        println!(" ");
    }

    fn opcode_is_if(&self, op:u8) -> bool {
        if op >= 19 && op <= 24 { return true; }
        return false;
    }

    pub fn print(&mut self) {
        let mut line: usize = 0;
        let len: usize = self.code.len();

        loop {

            if line >= len {
                break;
            }

            let ins:&Instruction = &self.code[line];
            println!("{}: {}", line+1,ins.to_string());

            line += 1;
       }
    }

       
    pub fn clone(&self) -> Cpu {
        let mut new_cpu: Cpu;

        new_cpu = Cpu::new();

        // clone code
        for instr in self.code.iter() {
            new_cpu.code.push(instr.clone());
        }

        // clone memory
        for m in self.mem.iter() {
            new_cpu.mem.push(m.clone());
        }

        // clone stack?
        for m in self.stack.iter() {
            new_cpu.stack.push(m.clone());
        }

        // clone vars
        for v in self.vars.iter() {
            new_cpu.vars.push(0.0);
        }

        return new_cpu;
    }

    pub fn mutate(&mut self, prob: usize) {
        let mut rng = rand::thread_rng();

        if self.get_rand(100) > 85 {
            // modification
            for i in 0..self.code.len() {
                if self.get_rand(100) < prob {
                    if self.get_rand(100) < 75 {
                        if self.get_rand(100) < 55 {
                            self.code[i].set_b(rng.gen_range::<u8>(0, 3));
                        } else {
                            self.code[i].set_a(rng.gen_range::<u8>(0, 3));
                        }
                    } else {
                        self.code[i].set_opcode(rng.gen_range::<u8>(0, NUM_OPCODES));
                    }
                }
            }

        } else {

            let mut del:bool = true;
            let mut add:bool = true;

            if self.code.len() > 25 {
                add = false;
            }
            if self.code.len() < 4 {
                del = false;
            }

            if del && add {
                if self.get_rand(30) > self.code.len() {
                    add = false
                } else {
                    del = false
                }
            }
            if add {
                self.add_random_line(rand::thread_rng());

            } else if del {
                let l = self.get_rand(self.code.len());
                self.code.remove(l);

            } else {
                println!("no size mutation {}",self.code.len());
            }
        }
    }

    pub fn is_assign(&self, op:u8) -> bool {
        if op==0_u8 || op==8_u8 || op==9_u8 || op==10_u8 {
            return true;
        }
        return false;
    }

    pub fn crossover2(&self, cpu_mother: &Cpu) -> Vec<Cpu> {
        let mut childs: Vec<Cpu> = Vec::new();

        for _ in 0..4 {
            childs.push(Cpu::new());
        }

        for i in 0..self.code.len() {
            for j in 0..4 {
                if self.get_rand(100) < 50 {
                    childs[j].code.push(self.code[i].clone());
                } else {
                    childs[j].code.push(cpu_mother.code[i].clone());
                }
            }
        }

        return childs;
    }

    pub fn crossover(&self, cpu_mother: &Cpu) -> (Cpu,Cpu) {
        let mut child1 = Cpu::new();
        let mut child2 = Cpu::new();
        let half_father = self.code.len()/2;
        let half_mother = cpu_mother.code.len()/2;

        // optimize this:
        for i in 0..half_father {
            child1.code.push(self.code[i].clone());
        }
        for i in 0..half_mother {
            child2.code.push(cpu_mother.code[i].clone());
        }
        for i in half_father..self.code.len() {
            child2.code.push(self.code[i].clone());
        }
        for i in half_mother..cpu_mother.code.len() {
            child1.code.push(cpu_mother.code[i].clone());
        }

        //child1.init_vars(self.vars.len());
        //child2.init_vars(self.vars.len());

        return (child1,child2);
    }

    pub fn remove_code(&mut self, to_remove:&mut Vec<usize>) {
        let mut ctx:usize = 0;
        for i in 0..to_remove.len() {
            self.code.remove(i-ctx);
            ctx+=1;
        }
        to_remove.clear();
    }


    pub fn optimizer(&mut self) {
        /*
            Static Optimizer.

            IDEA: en lugar de optimizar, generar y mutar inteligentemente
        */

        // 1. remove nops
        let mut clean:bool;
        loop {
            clean = true;
            for i in 0..self.code.len() {
                if self.code[i].op == 25 {
                    self.code.remove(i);
                    clean = false;
                    break;
                }
            }
            if clean {break}
        }


        // 2. remove counteracted x ++ x --   o x -- x ++
        loop {
            clean = true;
            if self.code.len() >=2 {
                for i in 0..self.code.len() - 1 {
                    if self.code[i].op == 6 && self.code[i + 1].op == 7 &&
                        self.code[i].a == self.code[i + 1].a {
                        self.code.remove(i);
                        clean = false;
                        break;
                    }
                    if self.code[i].op == 7 && self.code[i + 1].op == 6 &&
                        self.code[i].a == self.code[i + 1].a {
                        self.code.remove(i);
                        clean = false;
                        break;
                    }
                }
            }
            if clean {break}
        }



        // remove operations with vars that are not involved on the v1, v2 or other vars involved on v1 or v2
        // - follow all variables involved with v1 && v2 and add to list
        // - follow all variables involved with the




        // 3. remove lines with unused and unconected vars, whitelist all used vars
        let mut whitelist:Vec<u8> = Vec::new();
        let mut to_remove:Vec<usize> = Vec::new();

        if self.code.len() > 2 {
            for i in self.code.len()..0 {
                let a = self.code[i].a;
                let b = self.code[i].b;

                if self.code[i].op > 5 {
                    continue;
                }

                if a == 0 || a == 1 {
                    whitelist.push(b);
                    continue;
                }

                for j in 0..whitelist.len() {
                    if a == whitelist[j] {
                        whitelist.push(b);
                        continue;
                    }
                }

                to_remove.push(i);
            }
        }
        self.remove_code(&mut to_remove);


        let mut existsPush:bool = false;
        let mut existsPop:bool = false;

        for i in 0..self.code.len() {
            if self.code[i].op == 25 {
                existsPush = true;
                continue;
            }

            if self.code[i].op == 26 {
                existsPop = true;
                continue;
            }

            // 4.   v0 = 1; v0 = 2 seguidos
            if i>1 && self.is_assign(self.code[i].op) && self.is_assign(self.code[i-1].op) && self.code[i].a == self.code[i-1].a {
                to_remove.push(i);
            } else {

                // 5. v0 = v0  y if v3 ** v3
                if self.code[i].op == 0 || self.opcode_is_if(self.code[i].op) {
                    if self.code[i].a == self.code[i].b {
                        to_remove.push(i);
                    }
                }
            }
        }
        self.remove_code(&mut to_remove);


        // 6. push without pop
        if existsPush && !existsPop {
            self.code.insert(0,Instruction::new(26,2,0));
        } else if !existsPush && existsPop {
            self.code.push(Instruction::new(27, 2, 0))
        }


        // 7. remove ending if's (this optimization mut be at the end)
        while self.code.len() > 2 && self.opcode_is_if(self.code[self.code.len()-1].op) {
            let i = self.code.len() - 1;
            self.code.remove(i);
        }

    }

}


