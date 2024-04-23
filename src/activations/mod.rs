pub mod relu;
pub mod softmax;

pub trait Activation {
    fn forward(&self, inputs: &[f64]);
}
