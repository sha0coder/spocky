use std::vec::Vec;
use record::Prize;
use cpu::Cpu;

const FEES: f32 = 0.4;

pub struct Trader {
    buy: Cpu,
    sell: Cpu,
    usd: f32,
    eth: f32,
}

impl Trader {

    pub fn new() -> Trader {
        Trader {
            buy: Cpu::new(),
            sell: Cpu::new(),
            usd: 3000_f32,
            eth: 0_f32
        }
    }

    pub fn reset(&mut self) {
        self.usd = 3000_f32;
        self.eth = 0_f32;
    }

    pub fn set_buy(&mut self, cpu: Cpu) {
        self.buy = cpu;
    }

    pub fn set_sell(&mut self, cpu: Cpu) {
        self.sell = cpu;
    }

    pub fn set_usd(&mut self, usd: f32) {
        self.usd = usd;
    }

    pub fn set_eth(&mut self, eth: f32) {
        self.eth = eth;
    }

    pub fn get_usd(&self) -> f32 {
        return self.usd;
    }

    pub fn get_eth(&self) -> f32 {
        return self.eth;
    }

    pub fn randomize(&mut self, n: usize) {
        self.buy.randomize(n, 6); // 6 -> num_vars
        self.sell.randomize(n, 6);
    }

    pub fn get_fitness(&self) -> f32 {
        return self.usd;
    }

    pub fn getBuy(&self) -> &Cpu {
        return &self.buy;
    }

    pub fn getSell(&self) -> &Cpu {
        return &self.sell;
    }

    pub fn clone_buy(&self) -> Cpu {
        return self.buy.clone();
    }

    pub fn clone_sell(&self) -> Cpu {
        return self.sell.clone();
    }

    pub fn print(&self) {
        println!("best trader earn: ${} & {} eth", self.usd, self.eth);
    }

    pub fn print_details(&self) {
        println!("trader info:  ${} & {} eth", self.usd, self.eth);
        print!("  sell ");
        self.sell.print_opcodes();
        self.sell.print();
        print!("  buy ");
        self.buy.print_opcodes();
        self.buy.print();
    }

    pub fn clone(&self) -> Trader {
        let mut trader: Trader = Trader::new();
        trader.buy = self.buy.clone();
        trader.sell = self.sell.clone();
        trader.usd = self.usd;
        trader.eth = self.eth;
        return trader;
    }

    pub fn mutate(&mut self, n: usize) {
        // mutar solo uno?
        //self.buy.mutate(n);
        //self.sell.mutate(n);
    }

    pub fn crossover(&self, pair: &Trader) -> (Trader, Trader) {
        //let mut ng: Vec<Trader> = Vec::new();

        let mut tr1 = Trader::new();
        let mut tr2 = Trader::new();

        let (buy1,buy2) = self.buy.crossover(pair.getBuy());
        let (sell1,sell2) = self.sell.crossover(pair.getSell());

        tr1.set_buy(buy1);
        tr2.set_buy(buy2);
        tr1.set_sell(sell1);
        tr2.set_sell(sell2);

        //TODO: ussing cpu.crossver2

        /*
        for i in 0..4 {
            let mut tr = Trader::new();
            let mut buy = buy_cpus.pop();
            let mut sell = sell_cpus.pop();
            tr.setBuy(buy);
            tr.setSell(sell);
        }*/

        return (tr1, tr2);
    }

    pub fn do_buy(&mut self, prize: f32) {
        self.usd -= prize;
        self.usd -= (FEES*prize/100.0);
        self.eth += 1_f32;
    }

    pub fn do_sell(&mut self, prize: f32) {
        self.usd -= (FEES*prize/100.0);
        self.eth -= 1_f32;
        self.usd += prize;
    }

    pub fn trade(&mut self, pr: &Prize, trace: bool)  {
        let mut bought = false;

        if pr.usd < 2 {
            return;
        }

        self.buy.init_vars(pr.getVector());
        self.buy.run();
        if self.buy.result() == 1 {
            if self.usd as i32 >= pr.usd {
                
                if trace {
                    println!("buy {} saldo: {} eth: {}", pr.usd, self.usd, self.eth);
                }

                self.do_buy(pr.usd as f32);
                bought = true;
            }

        }

        if !bought {

            self.sell.init_vars(pr.getVector());
            self.sell.run();
            if self.sell.result() == 1 {
                if self.eth >= 1_f32 {
                    
                    if trace {
                        println!("sell {} saldo: {} eth: {}", pr.usd, self.usd, self.eth);
                    }
                    
                    self.do_sell(pr.usd as f32);
                }
            }

        }
    }
}

