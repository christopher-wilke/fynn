use super::Activation;
pub struct Softmax;

impl Activation for Softmax {
    fn forward(&self, inputs: &[f64]) {
        println!("trying to fwd using softmax");

        // 1. Get unnormalized probabilities
        //    np.exp(inpuits - np.max(inpuits, axis=1, keepdims=True))
        // 2. Normalize them for each sample
        //    exp_values/np.sum(exp_values, axis=1, keepdims=True)

    }
}
