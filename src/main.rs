mod fynn_array;
mod helpers;
mod importer;
mod layer_dense;

use fynn_array::*;
use importer::*;
use layer_dense::LayerDense;

static INPUT: &str = "py/out.txt";

pub fn main() {

    let content = Importer::from(INPUT);
    let input = content.get_values()
        .iter()
        .take(5)
        .cloned()
        .collect::<Vec<Vec<f64>>>()
        .to_fynn_array();

    let layer_dense = LayerDense::new();
    let output = layer_dense.fwd(&input);
    println!("output={output:?}");
}
