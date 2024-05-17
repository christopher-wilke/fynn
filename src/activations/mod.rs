use crate::FynnArray;

pub mod relu;
pub mod softmax;

pub trait Activation {
    fn forward(input: FynnArray) -> FynnArray;
}
