use crate::MathHelpers;
use super::{Loss, FynnArray};

pub struct LossCategoricalCrossentropy;

impl Loss for LossCategoricalCrossentropy {

    // As of May 2024, we expect y_true shape to be 1-dimensional
    fn calculate(y_pred: FynnArray, y_true: Vec<usize>) {
        let samples = y_pred.matrix.len();
        let y_pred_clipped = MathHelpers::clip(y_pred, 0.0000001, 0.9999999);

        // here we need to calculate the correct confidence based on y_true
    }
}
