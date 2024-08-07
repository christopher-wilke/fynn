mod fynn_array;
mod importer;
mod layer_dense;
mod activations;
mod loss;
mod math_helpers;

use activations::{Activation, softmax::*, relu::*};
use fynn_array::*;
use layer_dense::LayerDense;
use importer::*;
use math_helpers::*;

use crate::loss::{categorical_crossentropy::LossCategoricalCrossentropy, Loss};

pub fn main() {
    env_logger::init();

    let (input, y_true) = Importer::from_files("py/out.txt", "py/out_Y.txt");    

    // // Layer 1    
    // let mut dense1 = LayerDense::new(2, 3);
    // let out_dense_1 = dense1.fwd(&input);    
    // let activation1 = ActivationReLU::forward(&out_dense_1);

    // // Layer 2
    // let mut dense2 = LayerDense::new(3, 3);
    // let out_dense_2 = dense2.fwd(&activation1);
    // let mut activation2 = ActivationSoftmax::forward(&out_dense_2);

    // let loss = LossCategoricalCrossentropy::calculate(&mut activation2, y_true.clone());
    // let predictions = MathHelpers::argmax(activation2.clone());
    // // log::info!("predictions: {:?}", predictions); 
    // log::info!("y_true: {:?}", y_true);
    // // let accuracy = MathHelpers::mean(&predictions, &y_true);
    
    // log::info!("New set of weights found. Loss: {loss}, Accuarcy: {accuracy}");
    
    // // Helper Vars
    // let mut lowest_loss = 9999999.;
    // let best_dense1_weights = dense1.weights.clone();
    // let best_dense1_biases = dense1.biases.clone();
    // let best_dense2_weights = dense2.weights.clone();
    // let best_dense2_biases = dense2.biases.clone();

    // for i in 0..100 {
    //     dense1.weights = 0.05*FynnArray::randn(2, 3);
    //     dense1.biases = MathHelpers::rand_biases(3)
    //         .try_into()
    //         .expect("Error while trying to convert to Array");
    //     dense2.weights = 0.05*FynnArray::randn(3, 3);
    //     dense2.biases = MathHelpers::rand_biases(3)
    //         .try_into()
    //         .expect("Error while trying to convert to Array");

    //     let dense1 = dense1.fwd(&input);
    //     let activation1 = ActivationReLU::forward(&dense1);
    //     let dense2 = dense2.fwd(&activation1);
    //     let mut activation2 = ActivationSoftmax::forward(&dense2);
        
    //     let loss = LossCategoricalCrossentropy::calculate(&mut activation2, y_true.clone());
    //     let predictions = MathHelpers::argmax(activation2.clone()); 

    //     let accuracy = MathHelpers::mean(&predictions, &y_true);

    //     if loss < lowest_loss {
    //         lowest_loss = loss;
    //         log::info!("New set of weights found. Iteration: {i}, Loss: {loss}, Accuarcy: {accuracy}");
    //         // best_dense1_weights = dense1.weights.clone();
    //     }
    // }

   
}
