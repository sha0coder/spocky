use trader::Trader;
use record::Record;

const VALIDATION:usize = 100;

pub struct Play {
    rec: Record
}

impl Play {
    pub fn new(rec: Record) -> Play {
        Play {
            rec: rec
        }
    }

    pub fn simulate(&self, trader: &mut Trader) {
        trader.reset();
        if trader.get_usd() != 3000.0 {
            println!("init: {}",trader.get_usd());
        }

        for i in 0..(self.rec.sz()-VALIDATION) {        
            trader.trade(self.rec.get_prize(i));
        }
    }

    pub fn validate(&self, trader: &mut Trader) {
        trader.reset();
        for i in (self.rec.sz()-VALIDATION)..self.rec.sz() {        
            trader.trade(self.rec.get_prize(i));
        }
    }
}
