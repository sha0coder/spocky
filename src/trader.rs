use std::vec::Vec;
use record::Prize;
use cpu::Cpu;

const FEES:f32 = 0.4;
const VARS:u8 = 9;
const LINES:usize = 10;
const INIT_MONEY:f32 = 100000.0;

pub struct Trader {
    buy: Cpu,
    sell: Cpu,
    usd: f32,
    eth: f32,
    doTrace: bool
}

impl Trader {

    pub fn new() -> Trader {
        Trader {
            buy: Cpu::new(),
            sell: Cpu::new(),
            usd: INIT_MONEY,
            eth: 0_f32,
            doTrace: false
        }
    }

    pub fn trace(&mut self) {
        self.doTrace = true;
    }

    pub fn reset(&mut self) {
        self.usd = INIT_MONEY;
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

    pub fn randomize(&mut self) {
        self.buy.randomize(LINES);
        self.sell.randomize(LINES);
    }

    pub fn code_sz(&self) -> usize {
        return self.buy.get_code_sz() + self.sell.get_code_sz();
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

    pub fn load(&mut self, buy:&str, sell:&str) {
        self.buy.load(buy);
        self.sell.load(sell);
    }

    pub fn save(&self, buy:&str, sell:&str) {
        self.buy.save(buy);
        self.sell.save(sell);
    }

    pub fn print(&self) {
        println!("best trader earn: ${} & {} eth", self.usd, self.eth);
    }

    pub fn print_details(&mut self) {
        println!("trader info:  ${} & {} eth", self.usd, self.eth);
        print!("  sell ");
        self.sell.optimizer();
        self.sell.print_opcodes();
        self.sell.print();
        print!("  buy ");
        self.buy.optimizer();
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

    pub fn optimize(&mut self) {
        self.buy.optimizer();
        self.buy.optimizer();
    }

    pub fn mutate(&mut self, n: usize) {
        if self.buy.get_rand(100) <= n {
            if self.buy.get_rand(2)==1 {
                self.buy.mutate(n);
            } else {
                self.sell.mutate(n);
            }
        }
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

    pub fn do_buy_all(&mut self, pr: &Prize) {
        if self.usd > 1.0 {
            let prize: f32 = pr[3];
            let vol_eth = self.usd/prize;

            self.usd = 0.0;
            self.eth -= FEES*vol_eth/100.0;
            self.eth += vol_eth;

            if self.doTrace {
                println!("buy_all prize: {} balance: {} eth: {}", prize, self.usd, self.eth);
            }
        }
    }

    pub fn do_sell_all(&mut self, pr: &Prize) {
        if self.eth > 0.0 {
            let prize: f32 = pr[3];
            let vol_usd = prize * self.eth;

            self.usd -= FEES*vol_usd/100.0;
            self.eth = 0.0;
            self.usd += vol_usd;
            if self.doTrace {
                println!("sell_all prize: {} balance: {} eth: {}", prize, self.usd, self.eth);
            }
        }
    }

    pub fn do_buy_one(&mut self, pr: &Prize) {
        let prize: f32 = pr[3];
        if self.usd >= prize {
            self.usd -= prize;
            self.usd -= FEES*prize/100.0;
            self.eth += 1_f32; 
            if self.doTrace {
                println!("buy_one prize: {} balance: {} eth: {} ", prize, self.usd, self.eth);
            }
        }
    }

    pub fn do_sell_one(&mut self, pr: &Prize) {
        if self.eth > 0.0 {
            let prize: f32 = pr[3];
            self.usd -= FEES * prize / 100.0;
            self.eth -= 1.0;
            self.usd += prize;
            if self.doTrace {
                println!("sell_one prize: {} balance: {} eth: {} ", prize, self.usd, self.eth);
            }
        }
    }

    pub fn do_buy_selective(&mut self, pr: &Prize, amount:f32) {
        if amount >= 0.1 {
            let prize: f32 = amount * pr[3];
            let fees: f32 = FEES * prize / 100.0;
            if amount >= 0.1 && self.usd >= (prize + fees) {
                self.usd -= prize;
                self.usd -= fees;
                self.eth += amount;
                if self.doTrace {
                    println!("buy: {}x{} balance: {} eth: {} suply: {} cap: {}", amount, prize, self.usd, self.eth, pr[3], pr[4]);
                }
            }
        }
    }

    pub fn do_sell_selective(&mut self, pr: &Prize, amount:f32) { // amount is the number of eth's to buy/sell
        if amount >= 0.1 && self.eth > amount {
            let prize: f32 = amount * pr[3];
            self.usd -= FEES*prize/100.0;
            self.eth -= amount;
            self.usd += prize;
            if self.doTrace {
                println!("sell: {}x{} balance: {} eth: {} suply: {} cap: {}", amount, prize, self.usd, self.eth, pr[3], pr[4]);
            }
        }
    }

    pub fn trade(&mut self, pr: &Prize)  {
        let mut bought = false;

        if pr[3] < 2.0 {
            return;
        }

        self.buy.init_vars(pr.clone());
        self.buy.run();


        if self.buy.result() == 1.0 {
            let buy_amount:f32 = self.buy.get_var(1);
            self.do_buy_selective(pr, buy_amount);
        } else {
            self.sell.init_vars(pr.clone());
            self.sell.run();
            if self.sell.result() == 1.0 {
                let sell_amount:f32 = self.sell.get_var(1);
                self.do_sell_selective(pr,sell_amount);
            }
        }
    }
}

