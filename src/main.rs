mod fynn_array;
mod helpers;

use fynn_array::*;

pub fn main() {

   let inputs = [
        [1.0, 2.0, 3.0, 2.5],
        [2.0, 5.0, -1.0, 2.0],
        [-1.5, 2.7, 3.3, -0.8],
    ]
    .to_fynn_array();

    let weights = [
        [0.2, 0.8, -0.5, 1.0],
        [0.5, -0.91, 0.26, -0.5],
        [-0.26, -0.27, 0.17, 0.87]
    ]
    .to_fynn_array();
    let bias = [2.0, 3.0, 0.5];

    let weights2 = [
        [0.1, -0.14, 0.5],
        [-0.5, 0.12, -0.33],
        [-0.44, 0.73, -0.13]
    ]
    .to_fynn_array();
    let bias2 = [-1., 2., -0.5];
    
    let layer1_outputs = helpers::dot(&inputs, &weights.transpose()) + &bias;
    let layer2_outputs = helpers::dot(&layer1_outputs, &weights2.transpose()) + &bias2;

    println!("{layer2_outputs:?}");

}
