// use std::ops::{Add, Sub};

// use crate::fynn_array::FynnArray;

use fynn_array::FynnArray;

mod fynn_array;
// mod helpers;

struct NeuronsLayer {
    inputs: [f64; 4],
    weights: [[f64; 4]; 3],
    bias: [f64; 3],
    output: Vec<f64>,
}

impl NeuronsLayer {
    pub fn forward(&mut self) {
        for (i, bias) in self.bias.iter().enumerate() {
            let mut n_val = 0.;
            for (j, input) in self.inputs.iter().enumerate() {
                n_val += input * self.weights[i][j];
            }
            n_val += bias;
            self.output.push(n_val);
        }
    }

    pub fn get_output(&self) -> &Vec<f64> {
        &self.output
    }
}

fn main() {
    let array_2d = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut fynn_array = FynnArray::new(&array_2d);

    println!("Before:");
    println!("{:?}", fynn_array);
    
    fynn_array = fynn_array.transpose();

    println!("transposed:");
    println!("{:?}", fynn_array);

}
