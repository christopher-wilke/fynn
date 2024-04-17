mod activation_relu;
mod fynn_array;
mod helpers;
mod importer;
mod layer_dense;

use fynn_array::*;
use importer::*;
use layer_dense::LayerDense;
use activation_relu::ActivationRelu;

static INPUT: &str = "py/out.txt";

pub fn main() {
    env_logger::init();

    let input = [2., -1., 3.3, -2.7, 1.1, 2.2, -100.];
    let out = ActivationRelu::fwd(&input);

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
    // log::debug!("output={output:?}");
}
