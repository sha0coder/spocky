extern crate csv;

use std::vec::Vec;


pub type Prize = Vec<f32>;


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

    pub fn load(&mut self, filename: &str) {
        let mut prev_usd:f32 = 0.0;
        let mut cnt:usize = 0;
        let mut precios:Vec<f32> = Vec::new();
        let mut rdr = csv::Reader::from_file(filename).unwrap().delimiter(b',');
        for row in rdr.records() {
            let r = row.unwrap();

            let mut row:Prize = Vec::new();

            row.push(0.0);      // v1  // bool buy?
            row.push(0.0);      // v2  // int amount
            row.push(0.0);      // v3  // int temp_var

            for i in 0..r.len() {
                row.push(r[i].parse::<f32>().unwrap())
            }


            //[0], r[1].parse::<usize>().unwrap(), r[2].parse::<f32>().unwrap()
            //let prize = Prize::new(r);

            //prev_usd = r.usd;
            self.hist.push(row);

            cnt +=1;
            
        }
        println!("{} records loaded.", cnt);

    }

    pub fn disp(&self) {
        for i in 0..self.hist.len() {
            println!("{}", self.hist[i][2]);
        }
    }

}