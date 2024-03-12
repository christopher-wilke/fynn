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
    // let mut layer = NeuronsLayer {
    //     inputs: [1., 2., 3., 2.5],
    //     weights: [
    //         [0.2, 0.8, -0.5, 1.0],
    //         [0.5, -0.91, 0.26, -0.5],
    //         [-0.26, -0.27, 0.17, 0.87],
    //     ],
    //     bias: [2.0, 3.0, 0.5],
    //     output: vec![],
    // };
    // layer.forward();
    // println!("{:?}", layer.get_output());

    // let inputs = [1.0, 2.0, 3.0, 2.5];
    // // let i2 = [4; 2; 3];
    // let inputs2 = [[1.0, 2.0, 3.].to_vec(), [4., 5., 6.5].to_vec()].to_vec();

    // let array_1d = ["hello", "how are you"];
    let array_2d = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut fynn_array = FynnArray::new(&array_2d);

    println!("{:?}", fynn_array);
    println!("\n");
    fynn_array.transpose();

    // let weights = [
    //     [0.2, 0.8, -0.5, 1.0].to_vec(),
    //     [0.5, -0.91, 0.26, -0.5].to_vec(),
    //     [-0.26, -0.27, 0.17, 0.87].to_vec(),
    // ]
    // .to_vec();
    // let bias = FynnArray::from([2.0, 3.0, 0.5]);
}
