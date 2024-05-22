use crate::FynnArray;

pub mod categorical_crossentropy;

pub trait Loss {
    fn calculate(y_pred: FynnArray, y_true: Vec<f64>); 
}
