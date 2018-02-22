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

        let mut trace: bool = true;
        for i in 0..self.rec.sz() {
            //let pr = self.rec.getPrize(i);
            /*
            let mut vpr: Vec<i32> = Vec::new();
            vpr.push(0);
            vpr.push(pr.supply);
            vpr.push(pr.cap);
            vpr.push(pr.usd); //TODO: es mucho mas rapido hacer la conversion en el load
            */
            trader.trade(self.rec.get_prize(i), trace);
            trace = false;
        }
    }
}
