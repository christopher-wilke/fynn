use std::{
    any::Any, fmt::Debug, fs::File, io::{self, BufRead, BufReader}
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
        let f_input = File::open(input).expect("could not open file");
        let input_val = Self::to_vec(f_input);
        // let input_out = Self::input_converter(input_val);

        // Y_true
        let f_y_true = File::open(y_true).expect("could not open file");
        let mut y_true_val = Self::to_vec(f_y_true);

        (FynnArray::new(), y_true_val)
    }

    pub fn to_vec(input: File) -> InputType {
        let first_line = io::BufReader::new(&input)
            .lines()
            .next();

        if let Some(val) = first_line 
        {
            let zeros = val
                .unwrap()
                .chars()
                .filter(|c| c == &'0')
                .count();

            let reader = io::BufReader::new(input);

            match zeros {
                21 => Self::get_y(),
                _ => log::info!("I am X")
            }    
        };
        
        InputType::Integer(vec![])
    }

    fn get_y() {
        log::info!("executing get_y");
        let f_y_true = File::open("py/out_Y.txt").expect("could not open file");
        let reader = io::BufReader::new(f_y_true);

        for line in reader.lines() {
            log::info!("{}", line.unwrap());
        }
    
        // for line in reader.lines() {
        //     let val = line.unwrap();
        //     log::info!("jo");
        // }
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


