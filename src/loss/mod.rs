use crate::FynnArray;

pub mod categorical_crossentropy;

pub trait Loss {
    fn calculate(y_pred: &mut FynnArray, y_true: Vec<usize>) -> f64; 
}
