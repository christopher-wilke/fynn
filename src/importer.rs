use std::{fs::File, io::{self, BufRead}};

pub struct Importer {
    v: Vec<f64>
}

impl Importer {
    pub fn from(f: &str) -> Self {
        let file = File::open(f).expect("could not open input file");
        let reader = io::BufReader::new(file).lines();
        let mut resp = vec![];

        for line in reader.flatten() {
            for v in line.split_whitespace() {
                let val = v.parse::<f64>().expect("could not parse value");
                resp.push(val);
            }   
        }

        Self { v: resp }
    }

    pub fn get_data(&self) -> &Vec<f64> {
        &self.v
    }
}
