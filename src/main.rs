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

    let (input, y_true) = Importer::from_files("py/out.txt", "py/out_Y.txt");

    // Testing Input
    let input = vec![
        vec![0.17640523612499237, 0.5978738069534302],
        vec![0.040015723556280136, 0.7240893244743347]
    ].to_fynn_array();

    let weights = vec![
        vec![0.002473880972289464, 0.014895691422809047, -0.0044469715755695045],
        vec![0.008707085360891076, -0.0064183642832156855, -0.0018901445751785932]
    ].to_fynn_array();

    let result = MathHelpers::dot(&input, &weights);
    log::info!("{:?}", result);
    
    // Layer 1
    // let mut dense1 = LayerDense::new(2, 3);
    // let out_dense_1 = dense1.fwd(&input);
    // log::info!("out_dense_1: {:?}", out_dense_1);
    // let activation1 = ActivationReLU::forward(&out_dense_1);

    // // Layer 2
    // let mut dense2 = LayerDense::new(input.matrix.len(), 3);
    // let out_dense_2 = dense2.fwd(&activation1);
    // let activation2 = ActivationSoftmax::forward(&out_dense_2);

    // log::info!("{activation2:?}");

    // // Helper Vars
    // let mut lowest_loss = 9999999.;
    // let mut best_dense1_weights = dense1.weights.clone();
    // let mut best_dense1_biases = dense1.biases.clone();
    // let mut best_dense2_weights = dense2.weights.clone();
    // let mut best_dense2_biases = dense2.biases.clone();

    // let fa_1 = dense1.fwd(&input);
    // log::info!("fa_1={:?}", fa_1);
    // let activation1 = ActivationReLU::forward(&fa_1);
    // let fa_2 = dense2.fwd(&activation1);
    // let mut activation2 = ActivationSoftmax::forward(&fa_2);

    // let loss = LossCategoricalCrossentropy::calculate(&mut activation2, y_true.clone());

    // for i in 0..100 {
    //     dense1.weights += 0.5*FynnArray::randn(2, 3);
    //     dense1.biases += MathHelpers::rand_biases(3);
    //     dense2.weights += 0.05*FynnArray::randn(3, 3);
    //     dense2.biases += MathHelpers::rand_biases(3);

    //     let fa_1 = dense1.fwd(&input);
    //     let activation1 = ActivationReLU::forward(&fa_1);
    //     let fa_2 = dense2.fwd(&activation1);
    //     let mut activation2 = ActivationSoftmax::forward(&fa_2);

    //     let loss = LossCategoricalCrossentropy::calculate(&mut activation2, y_true.clone());
    //     let predictions = MathHelpers::argmax(activation2.clone());
    //     let accuracy = MathHelpers::mean(&predictions, &y_true);
    //     log::info!("New set of weights found! Iteration: {}, loss: {}, acc: {}", i, loss, accuracy);

    //     // if loss < lowest_loss {
    //     //     log::info!("New set of weights found! Iteration: {}, loss: {}, acc: {}", i, loss, accuracy);
    //     //     best_dense1_weights = dense1.weights.clone();
    //     //     best_dense1_biases = dense1.biases.clone();
    //     //     best_dense2_weights = dense2.weights.clone();
    //     //     best_dense2_biases = dense2.biases.clone();
    //     //     lowest_loss = loss;
    //     // }
    //     // else {
    //     //     dense1.weights = best_dense1_weights.clone();
    //     //     dense1.biases = best_dense1_biases.clone();
    //     //     dense2.weights = best_dense2_weights.clone();
    //     //     dense2.biases = best_dense2_biases.clone();
    //     // }
    // }
}
