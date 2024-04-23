mod activation_relu;
mod fynn_array;
mod helpers;
mod importer;
mod layer_dense;
mod activations;

use fynn_array::*;
use importer::*;
use layer_dense::LayerDense;
use activation_relu::ActivationRelu;
use activations::{softmax::*, Activation};

static INPUT: &str = "py/out.txt";

pub fn main() {
    env_logger::init();

    let val = [1., 2., 3.];

    let s = Softmax {};
    // s.forward(&val);

    let v = [
        [4.8, 1.21, 2.385],
        [8.9, -1.81, 0.2],
        [1.41, 1.051, 0.026]
    ].to_fynn_array();

    let sum = helpers::sum(&v);
    log::debug!("sum={sum:?}");

    let probabilities= helpers::normalize(v);
    log::debug!("{probabilities:?}");
   
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
