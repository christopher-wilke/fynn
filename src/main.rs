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

static INPUT: &str = "py/out.txt";

pub fn main() {
    env_logger::init();

    // let input = [ 
    //              [0.1587, -2.347],
    //              [0.0231, -1.432],
    //              [0.0031, 1.242]
    // ].to_fynn_array();

    let input = [
        [0.5, 1., 1.5]
    ].to_fynn_array();

    let softmax = Softmax {};
    let out = softmax.forward(input);    
    log::debug!("{out:?}");
   
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
