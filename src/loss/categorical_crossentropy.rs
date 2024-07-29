use crate::MathHelpers;
use super::{Loss, FynnArray};

pub struct LossCategoricalCrossentropy;

impl Loss for LossCategoricalCrossentropy {

    // todo: add support for y_true to be multidimensional
    fn calculate(y_pred: &mut FynnArray, y_true: Vec<usize>) -> f64 {
        let y_pred_clipped = MathHelpers::clip(y_pred, 0.0000001, 0.9999999);

        let correct_confidence: Vec<f64> = y_pred_clipped
            .iter()
            .zip(y_true.iter())
            .map(|(v, v_true)| {
                let val = *v.get(*v_true).unwrap();
                -val.ln()
            })
            .collect();

        correct_confidence.iter().sum::<f64>() / correct_confidence.len() as f64
    }
}
