extern crate csv;

use std::vec::Vec;

pub struct Prize {
    pub date: String,
    pub ts: usize,
    pub supply: i32,
    pub cap: i32,
    pub usd: i32,
    pub prev_usd: i32,
    pub first_usd: i32,
}

impl Prize {
    pub fn getVector(&self) -> Vec<i32> {
        let mut vpr: Vec<i32> = Vec::new();
        vpr.push(0);
        vpr.push(self.supply);
        vpr.push(self.cap);
        vpr.push(self.usd); 
        vpr.push(self.prev_usd);
        vpr.push(self.first_usd);
        return vpr;
    }
}

pub struct Record {
    hist: Vec<Prize>
}

impl Record {
    pub fn new() -> Record {
        Record {
            hist: Vec::new(),
        }
    }

    pub fn sz(&self) -> usize {
        return self.hist.len();
    }

    pub fn get_prize(&self, n: usize) -> &Prize {
        return &self.hist[n];
    }  

    pub fn clear(&mut self) {
        self.hist.clear();
    }

    pub fn load(&mut self, fname: &str) {
        let mut prev_usd = 0;
        let mut cnt = 0;
        let mut rdr = csv::Reader::from_file(fname).unwrap().delimiter(b',');
        for row in rdr.records() {
            let r = row.unwrap();
            
            let usd = r[4].parse::<f32>().unwrap();
            //println!("prize: {}",usd);
            let prize = Prize {
                date: r[0].to_string(),
                ts: r[1].parse::<usize>().unwrap(),
                supply: r[2].parse::<f32>().unwrap() as i32,        // v1
                cap: r[3].parse::<f32>().unwrap() as i32,           // v2
                usd: usd as i32,                                    // v3
                prev_usd: prev_usd,                                 // v4
                first_usd: 0,                                       // v5
            };

            //[0], r[1].parse::<usize>().unwrap(), r[2].parse::<f32>().unwrap()
            //let prize = Prize::new(r);

            prev_usd = prize.usd;
            self.hist.push(prize);
            
            cnt +=1;
            
        }
        println!("{} records loaded.", cnt);
        if cnt != self.hist.len() {
            println!("doesn't match");
        }
    }

    pub fn disp(&self) {
        for i in 0..self.hist.len() {
            println!("{} {}", self.hist[i].ts, self.hist[i].usd);
        }
    }

}