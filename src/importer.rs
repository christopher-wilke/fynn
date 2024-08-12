use std::{
    any::Any, fmt::Debug, fs::File, io::{self, BufRead, BufReader, Lines}
};

use crate::{FynnArray, FynnBehavior};

#[derive(Debug, Clone)]
pub struct Point {
    pub X: f64,
    pub Y: f64,
}

pub enum InputType {
    Integer(Vec<u32>),
    Float(FynnArray)
}

#[derive(Clone)]
pub struct Importer {
    pub v: Vec<Point>,
    pub y_true: Vec<u32>
}

impl Importer {

    pub fn from_files(input: &str, y_true: &str) -> (InputType, InputType) {
        log::trace!("Input: {input}, y_true: {y_true}");

        // Input X
        let f_input = File::open(input).expect("could not open file");
        let reader = io::BufReader::new(f_input);
        let input_x = Self::to_vec(reader);

        // Input Y
        let f_y_true = File::open(y_true).expect("could not open file");
        let reader = io::BufReader::new(f_y_true);
        let input_y = Self::to_vec(reader);

        (input_x, input_y)
    }

    pub fn to_vec(mut reader: BufReader<File>) -> InputType {

        let mut first_el = String::new();
        let _ = reader.read_line(&mut first_el);
        
        let pointer = reader.lines();
        let zeros = first_el
            .chars()
            .filter(|c| c == &'0')
            .count();

        let input = match zeros {
            21 => {
                let first_prime = Self::first_char_to_u32(first_el);
                Self::get_y(first_prime.unwrap(), pointer)
            },
            _ => {
                let first_row = Self::split_by_whitespace(first_el);
                Self::get_x(first_row, pointer)
            }
        };
        
        input
    }

    fn split_by_whitespace(v: String) -> Vec<f64> {
        v.split_whitespace()
            .map(|k| k.parse::<f64>().unwrap())
            .collect()
    }

    fn first_char_to_u32(v: String) -> Option<u32> {
        v.chars()
            .nth(0)
            .unwrap()
            .to_digit(10)
    }

    fn get_x(first_row: Vec<f64>, pointer: Lines<BufReader<File>>) -> InputType {
        let mut values = vec![first_row[0], first_row[1]];

        for value in pointer.flatten() {
            for k in Self::split_by_whitespace(value) {
                
            }
            match value.parse::<f64>() {
                Ok(v) => values.push(v),
                Err(e) => {
                    log::error!("{e:?}");
                }
            }
        }
        
        InputType::Float(Self::input_converter(values))
    }

    fn get_y(first_item: u32, pointer: Lines<BufReader<File>>) -> InputType {
        let mut values: Vec<u32> = vec![first_item];

        for value in pointer.flatten() {
            let v = Self::first_char_to_u32(value).unwrap();
            values.push(v);
        }
        log::info!("length of values is {}", values.len());
        InputType::Integer(values)
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


