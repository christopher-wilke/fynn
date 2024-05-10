mod activation_relu;
mod fynn_array;
mod helpers;
mod importer;
mod layer_dense;
mod activations;
mod math_helpers;

use fynn_array::*;
// use importer::*;
// use layer_dense::LayerDense;
// use activation_relu::ActivationRelu;
// use activations::{softmax::*, Activation};
use math_helpers::*;

static INPUT: &str = "py/out.txt";

pub fn main() {
    env_logger::init();

    let input = [ 
                 [0.1587, -2.347],
                 [0.0231, -1.432],
                 [0.0031, 1.242]
    ].to_fynn_array();
    
    let exp_values = MathHelpers::exp(&input - &MathHelpers::max(&input));
    let sum = MathHelpers::sum(&exp_values).to_fynn_array();
    let probabilities = &exp_values / &sum;

    log::debug!("final={probabilities:?}");
    // let probabilities = MathHelpers::normalize(&exp_values, &MathHelpers::sum(&exp_values));
    // log::debug!("{probabilities:?}");

    // let softmax = Softmax {};
    // softmax.forward(input);

    // let dense1 = LayerDense::new(2, 3);
    // let out = dense1.fwd(&input);
    // log::debug!("out={out:?}");


    // let sum = helpers::sum(&v);
    // log::debug!("sum={sum:?}");

    // let probabilities= helpers::normalize(v);
    // log::debug!("{probabilities:?}");
   
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
