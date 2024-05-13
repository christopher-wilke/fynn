mod activation_relu;
mod fynn_array;
mod helpers;
mod importer;
mod layer_dense;
mod activations;
mod math_helpers;

use activations::{Activation, softmax::*};
use fynn_array::*;
// use importer::*;
// use layer_dense::LayerDense;
// use activation_relu::ActivationRelu;
// use activations::{softmax::*, Activation};
use math_helpers::*;

use crate::{activation_relu::ActivationRelu, activations::relu::Relu, layer_dense::LayerDense};

static INPUT: &str = "py/out.txt";

pub fn main() {
    env_logger::init();

    
    let input = [[0.1587, -2.3472],
                 [0.0112, -1.4121],
                 [0.0031, -0.0512], 
                 [0.2312, -1.4712],
                 [0.2412, -0.0123]].to_fynn_array();
                 
    let dense1 = LayerDense::new(2, 3);
    let out_dense_1 = dense1.fwd(&input);
    log::debug!("{:?}", out_dense_1);
    // let activation1 = ActivationRelu::fwd(out_dense_1);
    // log::debug!("{:?}", activation1);


    // let softmax = Softmax {};
    // let out = softmax.forward(input);    
    // log::debug!("{out:?}");
   
    // let content = Importer::from(INPUT);
    // let input = content
    //     .get_values()
    //     .iter()
    //     .take(10)
    //     .cloned()
    //     .collect::<Vec<Vec<f64>>>()
    //     .to_fynn_array();

    // let n_inputs = input.matrix.first().unwrap().len();
    // let n_neurons: usize = 3;

    // let layer_dense = LayerDense::new(n_inputs, n_neurons);
    // let output = layer_dense.fwd(&input);
    // let activation = ActivationRelu::fwd(output);
    
    // log::debug!("output={activation:?}");
}
