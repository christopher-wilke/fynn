use crate::FynnArray;

pub struct MathHelpers;

impl MathHelpers {

    // keepdims = True, axis=1
    pub fn max(input: &FynnArray) -> Vec<Vec<f64>> {
        input.matrix
            .iter()
            .map(|row| {
                let max = row.iter().fold(
                    f64::NEG_INFINITY, 
                    |a, &b| a.max(b)
                );
                vec![max]
            })
            .collect()
    }

    
    pub fn exp(val: FynnArray) -> Vec<Vec<f64>> {
        val.matrix
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| std::f64::consts::E.powf(y))
                    .collect()
            })
            .collect()
    }
}
