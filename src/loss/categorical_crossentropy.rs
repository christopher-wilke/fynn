use crate::MathHelpers;

use super::{Loss, FynnArray};

pub struct LossCategoricalCrossentropy;

impl Loss for LossCategoricalCrossentropy {
    fn calculate(y_pred: FynnArray, y_true: Vec<f64>) {
        let samples = y_pred.matrix.len();
        let y_pred_clipped = MathHelpers::clip(y_pred, 0.00001, 0.9999);
    }
}
