use crate::FynnArray;

pub struct MathHelpers;

impl MathHelpers {
    pub fn max(input: FynnArray) -> FynnArray {
        log::debug!("trying to get the max");
        for i in input.matrix.iter() {
            let max = i.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            log::error!("{max}");
        }
        input
    }
}
