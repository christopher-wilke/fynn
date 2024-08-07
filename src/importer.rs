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
    Integer(Vec<i32>),
    Float(Vec<f64>)
}

#[derive(Clone)]
pub struct Importer {
    pub v: Vec<Point>,
    pub y_true: Vec<i32>
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

    pub fn to_vec(reader: BufReader<File>) -> InputType {
        // let first_line = io::BufReader::new(&input)
        //     .lines()
        //     .next();
        let mut pointer = reader.lines();

        if let Some(val) = pointer.next() 
        {
            let zeros = val
                .unwrap()
                .chars()
                .filter(|c| c == &'0')
                .count();

            // let f_y_true = File::open("py/out_Y.txt").expect("could not open file");
            // let reader = io::BufReader::new(f_y_true);

            let input = match zeros {
                21 => Self::get_y(pointer),
                _ => Self::get_y(pointer)
            };
        };
        
        InputType::Integer(vec![])
    }

    fn get_x(mut pointer: Lines<BufReader<File>>) -> InputType {
        let mut values: Vec<f64> = vec![];
        for line in pointer.next() {
            let value = line
                .unwrap()
                .parse::<f64>()
                .unwrap();
            values.push(value);
        }
        InputType::Float(values)
    }

    fn get_y(mut reader: Lines<BufReader<File>>) -> InputType {
        let mut values: Vec<i32> = vec![];

        for line in reader.next() {
            let value = line.unwrap();
            let v = value
                .chars()
                .nth(0)
                .unwrap()
                .to_digit(10)
                .unwrap() as i32; 
                
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


