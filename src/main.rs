extern crate csv;
extern crate rand;

mod instruction;
mod record;
mod trader;
mod play;
mod cpu;
mod ga;

use record::Record;
use trader::Trader;
use play::Play;
use ga::GA;


const GENERATIONS: usize = 5;
const POPULATION: usize = 100;
const EXPECTED_FITNESS: f32 = 1000.0;

fn test() {
    let mut trader = Trader::new();
    trader.load("winner.buy","winner.sell");
    trader.print_details();
}

fn Spocky_AI() {
    println!("Booting Spocky ...");
    let mut rec = Record::new();
    rec.load("data/bitcoin.csv");

    let play = Play::new(rec);
    let mut ga = GA::new(play);

    ga.init_population(POPULATION);
    loop {
        let mut winner = ga.train(GENERATIONS).unwrap();
        winner.print_details();

        if ga.validate(winner.clone(), EXPECTED_FITNESS) {
            println!("Winner!!");
            winner.optimize();
            winner.print_details();
            winner.save("winner.buy","winner.sell");
            break;
        } else {
            println!("failed validation, training again!");
        }
    }
}

fn main() {
    Spocky_AI();
}
