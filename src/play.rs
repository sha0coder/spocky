use trader::Trader;
use record::Record;

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

        for i in 0..self.rec.sz() {        
            trader.trade(self.rec.get_prize(i));
        }
    }
}
