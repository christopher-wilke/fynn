use std::{
    any::Any,
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
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
    pub y_true: Vec<u32>,
}

impl Importer {
    pub fn from_files(input: &str, y_true: &str) -> (FynnArray, Vec<u32>) {
        log::trace!("Input: {input}, y_true: {y_true}");

        // Input X
        let f_input = File::open(input).expect("could not open file");
        let reader = io::BufReader::new(f_input);
        let x = Self::get_x(reader.lines());

        // Input Y
        let f_y_true = File::open(y_true).expect("could not open file");
        let reader = io::BufReader::new(f_y_true);
        let y = Self::get_y(reader.lines());

        (x, y)
    }

    fn to_f64_by_whitespace(v: String) -> Vec<f64> {
        v.split_whitespace()
            .map(|k| k.parse::<f64>().expect("could not parse"))
            .collect()
    }

    fn first_char_to_u32(v: String) -> Option<u32> {
        v.chars().nth(0).unwrap().to_digit(10)
    }

    fn get_x(pointer: Lines<BufReader<File>>) -> FynnArray {
        let mut values = vec![];

        for value in pointer.flatten() {
            for k in Self::to_f64_by_whitespace(value) {
                values.push(k);
            }
        }

        Self::input_converter(values)
    }

    fn get_y(pointer: Lines<BufReader<File>>) -> Vec<u32> {
        let mut values: Vec<u32> = vec![];

        for value in pointer.flatten() {
            let v = Self::first_char_to_u32(value).unwrap();
            values.push(v);
        }
        values
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
}
