mod fynn_array;
mod helpers;

use fynn_array::*;

pub fn main() {

   let inputs = [
        [1.0, 2.0, 3.0, 2.5],
        [2.0, 5.0, -1.0, 2.0],
        [-1.5, 2.7, 3.3, -0.8],
    ].to_fynn_array();

    let weights = [
        [0.2, 0.8, -0.5, 1.0],
        [0.5, -0.91, 0.26, -0.5],
        [-0.26, -0.27, 0.17, 0.87]
    ]
    .to_fynn_array()
    .transpose();

    let out = helpers::dot(&inputs, &weights);
    println!("{out:?}");

}
