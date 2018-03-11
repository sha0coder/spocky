use trader::Trader;
use record::Record;

const VALIDATION:usize = 100;

pub struct Play {
    rec: Record
}

impl Play {
    pub fn new(rec: Record) -> Play {
        Play {
            rec
        }
    }

    pub fn simulate(&self, trader: &mut Trader) {
        trader.reset();


        for i in 0..(self.rec.sz()-VALIDATION) {
            trader.trade(self.rec.get_prize(i));
        }

        trader.do_sell_all(self.rec.get_prize(self.rec.sz()-VALIDATION-1));
    }

    pub fn validate(&self, trader: &mut Trader) {
        trader.reset();
        trader.trace();
        for i in (self.rec.sz()-VALIDATION)..self.rec.sz() {        
            trader.trade(self.rec.get_prize(i));
        }

        trader.do_sell_all(self.rec.get_prize(self.rec.sz()-1));
    }

    pub fn play_all(&self, trader: &mut Trader) {
        trader.reset();


        for i in 0..self.rec.sz() {
            trader.trade(self.rec.get_prize(i));
        }

        trader.do_sell_all(self.rec.get_prize(self.rec.sz()-1));
    }
}

