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

    pub fn from_files(input: &str, y_true: &str) -> (FynnArray, Vec<usize>) {
        log::trace!("Input: {input}, y_true: {y_true}");

        // Input
        let f_input = File::open(input).expect("could not open file");
        let input_val = Self::file_to_vec(f_input);
        let input_out = Self::input_converter(input_val);

        // Y_true
        let f_y_true = File::open(y_true).expect("could not open file");
        let y_true_val: Vec<usize> = Self::file_to_vec(f_y_true).into();

        (input_out, y_true_val)
    }

    pub fn to_vec<T>(item: T) -> Vec<T> {
        let mut resp = vec![];
        resp.push(item);
        resp
    }

    pub fn file_to_vec(input: File) -> Vec<f64> {
        let reader = io::BufReader::new(input).lines();
        let mut resp = vec![];

        for line in reader.flatten() {
            for v in line.split_whitespace() {
                let val: f64 = v.parse().expect("could not parse value");
                resp.push(val);
            }
        }

        resp
    }

    fn input_converter(values: Vec<f64>) -> FynnArray {
        let mut v: Vec<Point> = vec![];
        
        for chunk in values.chunks(2) {
            if let [x, y] = chunk {
                v.push(Point { X: *x, Y: *y });
            }
        }

        v.to_fynn_array()
    }

    pub fn get_v(self) -> Vec<Point> {
        self.v
    }

    pub fn get_y_true(self) -> Vec<i32> {
        self.y_true
    }

}
