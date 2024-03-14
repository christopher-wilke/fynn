mod fynn_array;
mod helpers;

use fynn_array::FynnArray;

pub fn main() {

    let inputs = FynnArray::new(&[
        [1.0, 2.0, 3.0, 2.5],
        [2.0, 5.0, -1.0, 2.0],
        [-1.5, 2.7, 3.3, -0.8],
    ]);

    let weights = FynnArray::new(&[
        [0.2, 0.8, -0.5, 1.0],
        [0.5, -0.91, 0.26, -0.5],
        [-0.26, -0.27, 0.17, 0.87]
    ]).transpose();

    helpers::dot(&inputs, &weights);

    // println!("{:?}", test_array.transpose());
 
    // let inputs = [1., 2., 3., 2.5];
    // let weights = [
    //     [0.2, 0.8, -0.5, 1.],
    //     [0.5, -0.91, 0.26, -0.5],
    //     [-0.26, -0.27, 0.17, 0.87]
    // ];
    // let bias = [2., 3., 0.5];

    // let mut layer = NeuronsLayer {
    //     inputs: inputs,
    //     weights: weights,
    //     bias: bias,
    //     output: Vec::new()
    // };

    // layer.forward();
    // println!("layer output = {:?}", layer.output);
}
