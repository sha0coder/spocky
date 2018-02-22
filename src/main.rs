extern crate csv;
extern crate rand;

mod instruction;
mod record;
mod trader;
mod play;
mod cpu;
mod ga;

use record::Record;
use play::Play;
use ga::GA;


const GENERATIONS: usize = 500;
const POPULATION: usize = 1000;
const INSTRUCTIONS: usize = 5;

fn main() {
    println!("Booting Spocky ...");
    let mut rec = Record::new();
    rec.load("/home/user/src/spocky/data/Ethereum.csv");

    let play = Play::new(rec);
    
    let mut ga = GA::new();
    ga.init_population(POPULATION, INSTRUCTIONS);
    ga.run(GENERATIONS, play);

}

