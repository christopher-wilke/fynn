use crate::MathHelpers;
use super::{Loss, FynnArray};

pub struct LossCategoricalCrossentropy;

impl Loss for LossCategoricalCrossentropy {

    // As of May 2024, we expect y_true shape to be 1-dimensional
    fn calculate(y_pred: FynnArray, y_true: Vec<usize>) {
        let samples = y_pred.matrix.len();        
        let y_pred_clipped = MathHelpers::clip(y_pred, 0.0000001, 0.9999999);

        // calculating correct_confidence
        let mut correct_confidence = vec![];
        for (v, v_true) in y_pred_clipped.iter().zip(y_true.iter()) {
            let conf = -v[*v_true].ln();
            correct_confidence.push(conf);
        }

        let avg_loss = correct_confidence.iter().sum::<f64>() / correct_confidence.len() as f64;
        log::error!("{avg_loss:?}");
    }
}
