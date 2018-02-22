/*
    Genetic Algorithm to evolve code.

*/

extern crate rand;

//use std::collections::HashMap;
//use std::process::exit;
use trader::Trader;
use std::vec::Vec;
use ga::rand::Rng;
use play::Play;

pub struct GA {
    population: Vec<Trader>,
    mutation_probability: u16,
    sz: usize,
    num_instructions: usize
}

impl GA {
    pub fn new() -> GA {
        GA {
            population: Vec::new(),
            mutation_probability: 2,
            sz: 0,
            num_instructions: 10
        }
    }


    pub fn get_rand(&self, n:usize) -> usize {
        let mut rng = rand::thread_rng();
        return rng.gen_range::<usize>(0,n);
    }

    pub fn init_population(&mut self, population_sz: usize, num_instructions: usize) {
        self.sz = population_sz;
        for _ in 0..population_sz {
            let mut trader = Trader::new();
            trader.randomize(num_instructions);
            self.num_instructions = num_instructions;
            self.population.push(trader);
        }
    }

    pub fn sort(&mut self) -> Vec<usize>  {
        let mut tmp: usize;
        let mut sorted: Vec<usize> = Vec::new(); 
        
        for i in 0..self.population.len() {
            sorted.push(i);
        }

        for i in 0..self.population.len()-1 {
            for j in 1..self.population.len() {
                if self.population[sorted[j]].get_fitness() > self.population[sorted[i]].get_fitness() {
                    tmp = sorted[j];
                    sorted[j] = sorted[i];
                    sorted[i] = tmp;
                }
            }
        }

        return sorted;
    }

    pub fn run(&mut self, num_cycles: usize, play: Play) {

        for cycle in 1..num_cycles+1 {

            // evaluate
            for i in 0..self.population.len() {
                let trader: &mut Trader = &mut self.population[i];
                play.simulate(trader);
            }

            // clasify
            let sorted = self.sort();
            println!("\n** Cycle: {} pop: {} max fitness: {} usd: {} eth: {}", cycle, self.population.len(), self.population[sorted[0]].get_fitness(), self.population[sorted[0]].get_usd(), self.population[sorted[0]].get_eth());
            self.population[sorted[0]].print_details();


            let mut ng: Vec<Trader> = Vec::new();

            // crossover top 50, looking for the best individual to pair
            for i in 0..49 {
                let father = &self.population[sorted[i]];
                let mother = &self.population[sorted[i+1]];

                // exchange buy & sell functions 
                let mut tr1 = Trader::new();
                let mut tr2 = Trader::new();

                tr1.set_buy(father.clone_buy());
                tr1.set_sell(mother.clone_sell());
                tr2.set_buy(mother.clone_buy());
                tr2.set_sell(father.clone_sell());

                ng.push(tr1);
                ng.push(tr2);

                let tr = &self.population[sorted[i]];
                let (tr3, tr4) = tr.crossover(mother);
                ng.push(tr3);
                ng.push(tr4);

                // crossover1
            }



            // trascend top 10
            for i in 0..3 {
                ng.push(self.population[sorted[i]].clone());
            }

            // diversity
            for _ in 0..100 {
                let r = self.get_rand(self.population.len());
                ng.push(self.population[r].clone());
            }

     

            // mutation
            for i in 0..ng.len() {
                ng[i].mutate(25);
            }

            // 100 news
            let mut trader;
            for _ in 0..1000 {
                trader = Trader::new();
                trader.randomize(self.num_instructions);
                ng.push(trader);
            }


            self.population = ng;
        }
    }
}

