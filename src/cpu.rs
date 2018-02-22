/*
    Basic & fast cpu emulator
    1.8seconds -> execute 1.000.000 instructions
    
    @sha0coder
*/


extern crate rand;

use std::vec::Vec;
use cpu::rand::Rng;
use instruction::Instruction;


pub struct Cpu {
    params: Vec<i32>,
    vars: Vec<i32>,
    code: Vec<Instruction>,
    fitness: i32,
    tv: Vec<u8>,  // tests results vector
    debugMode: bool,
    num_vars: usize
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            params: Vec::new(),
            vars: Vec::new(),
            code: Vec::new(),
            fitness: 0,
            tv: Vec::new(),
            debugMode: false,
            num_vars: 0
        }
    }

    pub fn set_fitness(&mut self, fitness: i32, tv: Vec<u8>) {
        self.tv = tv;
        self.fitness = fitness;
    }

    pub fn debug(&mut self) {
        self.debugMode = true;
    }

    pub fn get_tv(&self) -> Vec<u8> {
        return self.tv.clone();
    }

    pub fn get_fitness(&self) -> i32 {
        return self.fitness;
    }

    pub fn init_params(&mut self, p: Vec<i32>) {
        self.params = p;
    }

    pub fn init_vars(&mut self, v: Vec<i32>) {
        self.num_vars = v.len();
        self.vars = v;
    }

    pub fn get_var(&self, i: usize) -> i32 {
        return self.vars[i];
    }

    pub fn add_instruction(&mut self, i: Instruction) {
        self.code.push(i);
    }

    pub fn result(&self) -> i32 {
        return self.vars[0];
    }

    pub fn add(&mut self, op:u8, a:usize, b:usize) {
        // add code instruction
        self.code.push(Instruction::new(op,a,b));
    }

    pub fn instructions(&self) -> usize {
        return self.code.len();
    }

    pub fn load(&mut self, code: Vec<Instruction>) {
        self.code = code;
    }

    pub fn get_rand(&self, n:usize) -> usize {
        let mut rng = rand::thread_rng();
        return rng.gen_range::<usize>(0,n);
    }

    pub fn randomize(&mut self, n:usize, num_vars:usize) {
        self.num_vars = self.vars.len();
        // generate a random code of n lines
        let mut rng = rand::thread_rng();
        //println!("creating");
        let mut i:usize = 0;
        loop {
            if i >= n {
                break;
            }
            self.add(rng.gen_range::<u8>(0,14), rng.gen_range::<usize>(0,num_vars), rng.gen_range::<usize>(0,num_vars));
            i+=1;
        }
        //println!("created");
    }

    pub fn init(&mut self) {
        for i in 0..self.vars.len() {
            self.vars[i] = 0;
        }
    }

    pub fn run(&mut self) {
        let mut line: usize = 0;
        let len: usize = self.code.len();

        //self.init();

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


            match ins.op {
                0 => { self.vars[ins.a] = self.vars[ins.b] }

                1 => { self.vars[ins.a] += self.vars[ins.b] }
                2 => { self.vars[ins.a] -= self.vars[ins.b] }
                3 => { if self.vars[ins.a]<99 && self.vars[ins.b]<99 && self.vars[ins.a]>-99 && self.vars[ins.b]>-99 { self.vars[ins.a] *= self.vars[ins.b] } }
                4 => { if self.vars[ins.b] != 0 {self.vars[ins.a] /= self.vars[ins.b] }} 

                5 => { self.vars[ins.a] += 1 }
                6 => { self.vars[ins.a] -= 1 }
                7 => { self.vars[ins.a] = 0 }

                8 => { if self.vars[ins.a] == self.vars[ins.b] { line+=1 } }
                9 => { if self.vars[ins.a] != self.vars[ins.b] { line+=1 } }
                10 => { if self.vars[ins.a] < self.vars[ins.b] { line+=1 } }
                11 => { if self.vars[ins.a] <= self.vars[ins.b] { line+=1 } }
                12 => { if self.vars[ins.a] > self.vars[ins.b] { line+=1 } }
                13 => { if self.vars[ins.a] >= self.vars[ins.b] { line+=1 } }

                _ => {;}
            }

            if self.debugMode {
                for i in 0..self.vars.len() {
                    println!("2 var[{}] = {}",i,self.vars[i]);
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

    pub fn print(&self) {
        let mut line: usize = 0;
        let len: usize = self.code.len();

        loop {

            if line >= len {
                break;
            }

            let ins:&Instruction = &self.code[line];

            match ins.op {
                0 => { println!("{}: v{} = v{}", line, ins.a, ins.b) }

                1 => { println!("{}: v{} += v{}", line, ins.a, ins.b) }
                2 => { println!("{}: v{} -= v{}", line, ins.a, ins.b) }
                3 => { println!("{}: v{} *= v{} if not overflow", line, ins.a, ins.b) }
                4 => { println!("{}: v{} /= v{} if divisible", line, ins.a, ins.b)} 

                5 => { println!("{}: v{} ++", line, ins.a) }
                6 => { println!("{}: v{} --", line, ins.a) }
                7 => { println!("{}: v{} = 0", line, ins.a) }

                8 => { println!("{}: if v{} != v{}", line, ins.a, ins.b) } //if self.vars[ins.a] == self.vars[ins.b] { line+=1 } }
                9 => { println!("{}: if v{} == v{}", line, ins.a, ins.b) } //if self.vars[ins.a] != self.vars[ins.b] { line+=1 } }
                10 => { println!("{}: if v{} >= v{}", line, ins.a, ins.b) } //if self.vars[ins.a] < self.vars[ins.b] { line+=1 } }
                11 => { println!("{}: if v{} > v{}", line, ins.a, ins.b) } //if self.vars[ins.a] <= self.vars[ins.b] { line+=1 } }
                12 => { println!("{}: if v{} <= v{}", line, ins.a, ins.b) }   //if self.vars[ins.a] > self.vars[ins.b] { line+=1 } }
                13 => { println!("{}: if v{} < v{}", line, ins.a, ins.b) } //if self.vars[ins.a] >= self.vars[ins.b] { line+=1 } }

                _ => { println!("{}: nop", line) }
            }

            line += 1;
       }
    }

       
    pub fn clone(&self) -> Cpu {
        let mut new_cpu: Cpu;

        new_cpu = Cpu::new();
        //new_cpu.init_vars(self.vars.len());
        for instr in self.code.iter() {
            new_cpu.code.push(instr.clone());
        }

        return new_cpu;
    }

    pub fn mutate(&mut self, prob: usize) {
        let mut rng = rand::thread_rng();

        for i in 0..self.code.len() {
            if self.get_rand(100) < prob {
                if self.get_rand(100) < 75 {
                    if self.get_rand(100) < 55 {
                        self.code[i].set_b(rng.gen_range::<usize>(0,3));
                    } else {
                        self.code[i].set_a(rng.gen_range::<usize>(0,3));
                    }
                } else {
                    self.code[i].set_opcode(rng.gen_range::<u8>(0,14));
                }
            }
        }

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
        for i in half_mother..self.code.len() {
            child1.code.push(cpu_mother.code[i].clone());
        }

        //child1.init_vars(self.vars.len());
        //child2.init_vars(self.vars.len());

        return (child1,child2);
    }

}


