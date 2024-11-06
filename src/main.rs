mod activations;
mod fynn_array;
mod fynn_bias;
mod importer;
mod layer_dense;
mod loss;
mod math_helpers;

use activations::{relu::*, softmax::*, Activation};
use fynn_array::*;
use importer::*;
use layer_dense::LayerDense;
use math_helpers::*;
use crate::{fynn_bias::FynnBias, loss::{categorical_crossentropy::LossCategoricalCrossentropy, Loss}};

pub fn main() {
    env_logger::init();

    // Passed-in gradients from the next layer
    let mut dvalues = FynnArray { matrix: vec![
        vec![1., 1., 1.],
        vec![2., -2., 2.],
        vec![3., 3., -3.]
    ]};

    log::info!("{:?}", dvalues);

    let act = ActivationReLU::bwd(&dvalues);

    log::info!("{:?}", act);

    // let mut inputs= FynnArray { matrix: vec![
    //     vec![1., 2., 3., 2.5],
    //     vec![2., 5., -1., 2.],
    //     vec![-1.5, 2.7, 3.3, -0.8]
    // ]};

    // let mut weights = FynnArray { matrix: vec![
    //     vec![0.2, 0.8, -0.5, 1.],
    //     vec![0.5, -0.91, 0.26, -0.5],
    //     vec![-0.26, -0.27, 0.17, 0.87]  
    // ]}.transpose();

    // let mut biases = FynnBias { val: vec![2., 3., 0.5] };
    // let layer_outputs = MathHelpers::dot(&inputs, &weights) + &biases;
    // let relu_outputs = ActivationReLU::forward(&layer_outputs);

    // // Backpropagation
    // let drelu = relu_outputs.clone();
    // let dinputs = MathHelpers::dot(&drelu, &weights.clone().transpose());
    // let dweights = MathHelpers::dot(&inputs.transpose(), &drelu);

    // // We should not use separate structs for FynnBias and FynnArray
    // let dbiases = FynnBias { val: MathHelpers::sum(&drelu.matrix) };

    // weights += -0.001*dweights;
    // biases += -0.001*dbiases;
    // log::info!("{:?}", weights);
    // log::info!("{:?}", biases);

    // let (input, y_true) = Importer::from_files("py/out.txt", "py/out_Y.txt");
    
    // // Prepare Layers for initial calculation
    // let mut dense1 = LayerDense::new(2, 3);
    // let mut dense2 = LayerDense::new(3, 3);

    // // Helper Vars
    // let mut lowest_loss = 9999999.;
    // let mut best_dense1_weights = dense1.weights.clone();
    // let mut best_dense1_biases = dense1.biases.clone();
    // let mut best_dense2_weights = dense2.weights.clone();
    // let mut best_dense2_biases = dense2.biases.clone();

    // for i in 0..1 {
    //     dense1.weights += 0.5*FynnArray::randn(2, 3);
    //     dense1.biases += MathHelpers::rand_biases(3);
    //     dense2.weights += 0.5*FynnArray::randn(3, 3);
    //     dense2.biases += MathHelpers::rand_biases(3);

    //     let fa_1 = dense1.fwd(&input);
    //     let activation1 = ActivationReLU::forward(&fa_1);
    //     let fa_2 = dense2.fwd(&activation1);
    //     let mut activation2 = ActivationSoftmax::forward(&fa_2);

    //     let loss = LossCategoricalCrossentropy::calculate(&mut activation2, y_true.clone()); 
    //     let predictions = MathHelpers::argmax(activation2.clone());
    //     let accuracy = MathHelpers::mean(&predictions, &y_true);

    //     if loss < lowest_loss {
    //         log::info!("New set of weights found! Iteration: {}, loss: {}, acc: {}", i, loss, accuracy);
    //         best_dense1_weights = dense1.weights.clone();
    //         best_dense1_biases = dense1.biases.clone();
    //         best_dense2_weights = dense2.weights.clone();
    //         best_dense2_biases = dense2.biases.clone();
    //         lowest_loss = loss;
    //     }
    //     else {
    //         dense1.weights = best_dense1_weights.clone();
    //         dense1.biases = best_dense1_biases.clone();
    //         dense2.weights = best_dense2_weights.clone();
    //         dense2.biases = best_dense2_biases.clone();
    //     }
    // }
}
