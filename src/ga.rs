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
    play: Play
}

impl GA {
    pub fn new(play: Play) -> GA {
        GA {
            population: Vec::new(),
            mutation_probability: 2,
            sz: 0,
            play
        }
    }


    pub fn get_rand(&self, n:usize) -> usize {
        let mut rng = rand::thread_rng();
        return rng.gen_range::<usize>(0,n);
    }

    pub fn init_population(&mut self, population_sz: usize) {
        self.sz = population_sz;
        for _ in 0..population_sz {
            let trader = Trader::new();
            self.population.push(trader);
        }
    }

    fn sort(&mut self) -> Vec<usize>  {
        let mut tmp: usize;
        let mut sorted: Vec<usize> = Vec::new(); 
        
        for i in 0..self.population.len() {
            // antes ordenar: println!("fitness: {}",self.population[i].get_fitness());
            sorted.push(i);
        }

        // bubble sort
        for i in 0..self.population.len()-1 {
            let mut k = i;
            for j in i+1..self.population.len() {
                if self.population[sorted[j]].get_fitness() > self.population[sorted[k]].get_fitness() {
                    k = j;
                } else if self.population[sorted[j]].get_fitness() == self.population[sorted[k]].get_fitness() {
                    if self.population[sorted[j]].code_sz() > self.population[sorted[k]].code_sz() {
                        k = j;
                    }
                }
            }

            tmp = sorted[k];
            sorted[k] = sorted[i];
            sorted[i] = tmp;
        }


        // IDEA: si el fitness es el mismo, gana el codigo mas pequeño?




        /* verify sort:
        for i in 0..self.population.len() {
            println!("fitness: {}",self.population[sorted[i]].get_fitness());
        }*/

        return sorted;
    }

    pub fn get_average(&self) -> i32 {
        let mut sum:f32 = 0.0;
        for i in 0..self.population.len() {
            sum += self.population[i].get_usd();
        }
        sum /= self.population.len() as f32;
        return sum as i32;
    }

    pub fn validate(&mut self, mut trader:Trader, expected_fitness:f32) -> bool {
        println!("Starting validation ...");
        trader.reset();
        self.play.validate(&mut trader);
        if trader.get_usd() > expected_fitness {
            println!("target reached!! usd:{} cryptos:{}", trader.get_usd(), trader.get_eth());
            trader.reset();
            trader.trace();
            self.play.play_all(&mut trader);
            println!("final score: usd:{} cryptos:{}", trader.get_usd(), trader.get_eth());
            trader.optimize();
            return true;
        }
        return false;
    }

    pub fn train(&mut self, num_cycles: usize) -> Option<Trader> {

        for i in 0..self.population.len() {
            self.population[i].randomize();
        }

        for cycle in 1..num_cycles+1 {

            // evaluate
            for i in 0..self.population.len() {
                let mut trader: &mut Trader = &mut self.population[i];
                //trader.optimize();
                self.play.simulate(trader);
            }

            // clasify
            let sorted = self.sort();
            
            println!("** Cycle: {} pop: {} avg: {}  balance: usd: {} cryptos: {}", cycle, self.population.len(), self.get_average(), self.population[sorted[0]].get_usd(), self.population[sorted[0]].get_eth());
            self.population[sorted[0]].print_details();

            // TRAINING FINISHED!!
            if cycle == num_cycles {
                let winner:Trader = self.population[sorted[0]].clone();
                return Some(winner);
            }

            let mut ng: Vec<Trader> = Vec::new();

            // crossover top 10%, looking for the best individual to pair
            for i in 0..(10*self.sz/100) {
                let father = &self.population[sorted[i]];
                let mother = &self.population[sorted[i+1]];

                // exchange buy & sell functions 
                let mut tr1 = Trader::new();
                let mut tr2 = Trader::new();

                tr1.set_buy(father.clone_buy());
                tr1.set_sell(mother.clone_sell());
                tr2.set_buy(mother.clone_buy());
                tr2.set_sell(father.clone_sell());
                tr1.optimize();
                tr2.optimize();

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
            for _ in 0..10 {
                let r = self.get_rand(self.population.len());
                ng.push(self.population[r].clone());
            }

     

            // mutation
            for i in 0..ng.len() {
                ng[i].mutate(25);
            }

            // new ones random
            let mut trader;
            while ng.len()<self.sz {
                trader = Trader::new();
                trader.randomize();
                ng.push(trader);
            }


            self.population = ng;
        }

        return None;
    }
}

