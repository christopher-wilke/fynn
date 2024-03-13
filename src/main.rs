mod fynn_array;
mod neurons_layer;

use fynn_array::FynnArray;
use neurons_layer::NeuronsLayer;

pub fn main() {
 
    let inputs = [1., 2., 3., 2.5];
    let weights = [
        [0.2, 0.8, -0.5, 1.],
        [0.5, -0.91, 0.26, -0.5],
        [-0.26, -0.27, 0.17, 0.87]
    ];
    let bias = [2., 3., 0.5];

    let mut layer = NeuronsLayer {
        inputs: inputs,
        weights: weights,
        bias: bias,
        output: Vec::new()
    };

    layer.forward();
    println!("layer output = {:?}", layer.output);
}
