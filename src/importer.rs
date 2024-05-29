use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::{FynnArray, FynnBehavior};

#[derive(Debug, Clone)]
pub struct Point {
    pub X: f64,
    pub Y: f64,
}

#[derive(Clone)]
pub struct Importer {
    pub v: Vec<Point>,
    pub y_true: Vec<i32>
}

impl Importer {
    pub fn from(f: &str) -> (FynnArray, Vec<i32>) {
        log::trace!("Importing '{f}'");
        let file = File::open(f).expect("could not open input file");
        let reader = io::BufReader::new(file).lines();
        let mut resp = vec![];

        for line in reader.flatten() {
            for v in line.split_whitespace() {
                let val: f64 = v.parse().expect("could not parse value");
                resp.push(val);
            }
        }

        Self::converter(resp)
    }

    fn converter(values: Vec<f64>) -> (FynnArray, Vec<i32>) {
        let mut v: Vec<Point> = vec![];
        let mut y_true = vec![];
        // Startin with -1 as logic increments by 1
        let mut current_y_true = -1;
        
        for chunk in values.chunks(2) {
            if let [x, y] = chunk {
                if *x == 0. && *y == 0. {
                    current_y_true += 1;
                }
                v.push(Point { X: *x, Y: *y });
                y_true.push(current_y_true);
            }
        }

        (v.to_fynn_array(), y_true)
    }

    pub fn get_v(self) -> Vec<Point> {
        self.v
    }

    pub fn get_y_true(self) -> Vec<i32> {
        self.y_true
    }

}
