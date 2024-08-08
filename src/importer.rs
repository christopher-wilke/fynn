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
    Float(Vec<f64>)
}

#[derive(Clone)]
pub struct Importer {
    pub v: Vec<Point>,
    pub y_true: Vec<u32>
}

impl Importer {

    pub fn from_files(input: &str, y_true: &str) -> (FynnArray, InputType) {
        log::trace!("Input: {input}, y_true: {y_true}");

        // // Input
        // let f_input = File::open(input).expect("could not open file");
        // let input_val = Self::to_vec(f_input);
        // let input_out = Self::input_converter(input_val);

        // Y_true
        let f_y_true = File::open(y_true).expect("could not open file");
        let reader = io::BufReader::new(f_y_true);
        let y_true_val = Self::to_vec(reader);

        (FynnArray::new(), y_true_val)
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
                let first_prime = Self::first_char_convert(first_el);
                Self::get_y(first_prime.unwrap(), pointer)
            },
            _ => Self::get_y(0, pointer)
        };
        
        input
    }

    fn first_char_convert(v: String) -> Option<u32> {
        v.chars()
            .nth(0)
            .unwrap()
            .to_digit(10)
    }

    fn get_x(first_item: f64, pointer: Lines<BufReader<File>>) -> InputType {
        let mut values = vec![first_item];
    }

    fn get_y(first_item: u32, pointer: Lines<BufReader<File>>) -> InputType {
        let mut values: Vec<u32> = vec![first_item];

        for value in pointer.flatten() {
            let v = Self::first_char_convert(value).unwrap();
            values.push(v);
        }
        log::info!("length of values is {}", values.len());
        InputType::Integer(values)
    }
        
        // for line in reader.flatten() {
        //     for v in line.split_whitespace() {
        //         // log::info!("v={}", v);
        //         let num = "0".to_owned();
        //         let val = match v.parse::<T>() {
        //             Ok(v) => v,
        //             Err(e) => { panic!("panic!!! {e:?}") }
        //         };
        //         resp.push(val);
        //     }
        // }
        // resp    
    // }

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


