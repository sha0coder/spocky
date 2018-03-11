// S3 trading system


/*
        Se ha probado el sistema de realizar 3 compras a market y programar 3 ventas limits con sus stops,
        esto no renta ni en mercado alcista.

        Se ha probado un contador de tendencia, cuando llega a 4 compra y cuando llega a -4 vende
*/

use std::vec::Vec;
use record_btc::Prize;
use record_btc::Record;

const FEES:f32 = 0.4;

#[derive(PartialEq,PartialOrd,Clone)]
pub enum OrderType {
    Limit,
    Stop
}

#[derive(PartialEq,PartialOrd,Clone)]
pub enum Operation {
    Buy,
    Sell
}

pub struct Order {
    pub order_type: OrderType,
    pub operation: Operation,
    pub amount: f32,
    pub prize: f32
}

impl Order {
    pub fn clone(&self) -> Order {
        return Order {
            order_type: self.order_type.clone(),
            operation: self.operation.clone(),
            amount: self.amount.clone(),
            prize: self.prize.clone(),
        }
    }
}

pub type Orders = Vec<Order>;


pub struct S3 {
    orders: Orders,
    usd: f32,
    eth: f32,
    doTrace: bool
}

impl S3 {
    pub fn new() -> S3 {
        S3{
            orders: Vec::new(),
            usd: 60000_f32,
            eth: 0_f32,
            doTrace: false
        }
    }

    pub fn run(&mut self) {
        let mut rec:Record = Record::new();
        let prizes = rec.load("/home/user/src/spocky/data/btc.csv");

        let mut trend:i32 = 0;
        let mut last:f32 = 0.0;

        for i in 0..prizes.len() {
            let prize = prizes[i];

            if i>0 {
                if prize > last {
                    trend +=1;
                } else if prize < last {
                    trend -=1;
                }
            }
            last = prize.clone();

            if trend == 10 {
                println!("BUY {}",trend);
                //trend=0;
                self.buy(&prize, 1.0);
            } else if trend < -10 {
                println!("SELL {}",trend);
                self.sell(&prize, 1.0);
               // trend=0;
            }

            /*
            if trend > 300 {
                trend = 0;
            }*/


            // 2 ->  9.000
            // 3 -> 23.000
            // 4 -> 26.000
            // 5 -> 12.000
            // 6 ->  9.000



            //self.trade_s3(prize);
            //self.execute_orders(prize);

            println!("score: ${}  eth:{} orders:{} value:{} trend:{} benefit:{}",self.usd,self.eth,self.orders.len(), prize*self.eth+self.usd, trend, prize*self.eth+self.usd-60000.0);
        }

    }
/*
    fn execute_orders(&mut self, prize:&Prize) {
        let mut delete_orders:Vec<usize> = Vec::new();

        for i in 0..self.orders.len() {
            let order = self.orders[i].clone();

            if order.order_type == OrderType::Stop {
                if prize.usd <= order.amount {
                    self.sell(prize, order.amount.clone());
                }
            } else if order.order_type == OrderType::Limit {
                if prize.usd >= order.amount {
                    self.sell(prize, order.amount.clone());
                }
            }

            delete_orders.push(i);
        }

        let mut idx:usize = 0;
        for i in 0..delete_orders.len() {
            self.orders.remove(i-idx);
            idx+=1;
        }
    }
*/


    fn buy(&mut self, pr: &f32, amount:f32) {
        let prize: f32 = amount * pr;
        let fees: f32 = FEES*prize/100.0;
        if self.usd >= (prize+fees) {
            self.usd -= prize;
            self.usd -= fees;
            self.eth += amount;
            println!("BUY");

            if self.doTrace {
                //println!("buy_one prize: {} balance: {} eth: {} supply: {} cap: {} ", prize, self.usd, self.eth, pr.supply, pr.cap);
            }
        }
    }

    fn sell(&mut self, pr: &f32, amount:f32) { // amount is the number of eth's to buy/sell
        if self.eth > amount {
            let prize: f32 = amount * pr;
            self.usd -= FEES*prize/100.0;
            self.eth -= amount;
            self.usd += prize;
            println!("SELL");
            if self.doTrace {
                //println!("sell_one prize: {} balance: {} eth: {} supply: {} cap: {} ", prize, self.usd, self.eth, pr.supply, pr.cap);
            }
        }
    }

/*

    fn trade_s3(&mut self, prize:&Prize) {

        // si no hay ordenes programar 3
        if self.orders.len() == 0 {
            // 1

            self.orders.push(Order{
                order_type: OrderType::Limit,
                operation: Operation::Sell,
                amount: 1.0,
                prize: prize.usd +100.0
            });

            self.orders.push(Order{
                order_type: OrderType::Stop,
                operation: Operation::Sell,
                amount: 1.0,
                prize: prize.usd -60.0
            });

            // 2
            self.orders.push(Order{
                order_type: OrderType::Limit,
                operation: Operation::Sell,
                amount: 1.0,
                prize: prize.usd +300.0
            });

            self.orders.push(Order{
                order_type: OrderType::Stop,
                operation: Operation::Sell,
                amount: 1.0,
                prize: prize.usd -120.0
            });

            // 3
            self.orders.push(Order{
                order_type: OrderType::Limit,
                operation: Operation::Sell,
                amount: 1.0,
                prize: prize.usd +650.0
            });

            self.orders.push(Order{
                order_type: OrderType::Stop,
                operation: Operation::Sell,
                amount: 1.0,
                prize: prize.usd -200.0
            });


            self.buy(&prize, 3.0);
        }
    }*/






        /*
             hay mas probabilidades de ganar pero gano menos de lo que perderia:

        3 ordenes

        si no hay ordenes programadas:
            compras 3 a market
            1 limit a +20  con stop en -60
            1 limit a +100 con stop en -120
            1 limit a +600 con stop en -200


        si cae y solo se ha ejecutado la de +20:
            1 limit +80

        */



}

