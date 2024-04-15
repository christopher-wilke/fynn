use std::{convert, fs::File, io::{self, BufRead}};

#[derive(Debug, Clone)]
pub struct Point {
    X: f64,
    Y: f64
}

#[derive(Clone)]
pub struct Importer {
    v: Vec<Point>
}

impl Importer {
    pub fn from(f: &str) -> Self {
        let file = File::open(f).expect("could not open input file");
        let reader = io::BufReader::new(file).lines();
        let mut resp = vec![];

        for line in reader.flatten() {
            for v in line.split_whitespace() {
                let val: f64 = v.parse().expect("could not parse value");
                resp.push(val);
            }   
        }
        
        Self { v: Self::converter(resp) }
    }

    fn converter(values: Vec<f64>) -> Vec<Point> {
        let mut v: Vec<Point> = vec![];
        for chunk in values.chunks(2) {
            if let [first, second] = chunk {
                v.push(Point{ X: *first, Y: *second });
            }
        }
        v
    }

    pub fn get_values(&self) -> Vec<Vec<f64>> {
        let mut v = vec![];
        for p in self.v.iter() {
            v.push(vec![p.X, p.Y]);
        }
        v
    }

    pub fn get_x(&self) -> Vec<f64> {
        self.clone()
            .v
            .iter()
            .map(|v| v.X)
            .collect()
    }

    pub fn get_y(&self) -> Vec<f64> {
        self.clone()
            .v
            .iter()
            .map(|v| v.Y)
            .collect()
    }
    
}
