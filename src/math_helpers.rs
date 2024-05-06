use crate::FynnArray;

pub struct MathHelpers;

impl MathHelpers {

    // Normalization
    pub fn normalize(exp_values: &FynnArray, sum: &FynnArray) -> Vec<Vec<f64>> {
        let mut norm = vec![];
        for (e, s) in exp_values.matrix.iter().zip(sum.matrix.iter()) {
            let summed = *s.get(0).unwrap();
            let mut val = vec![];
            for i in e {
                val.push(i/summed);
            }
            norm.push(val);
        }
        norm
    }

    // keepdims=True, axis=1
    pub fn sum(input: &FynnArray) -> Vec<Vec<f64>> {
        input.matrix
            .iter()
            .map(|row| {
                let sum = row.iter().sum::<f64>();
                vec![sum]   
            })
            .collect()
    }

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

    
    pub fn exp(val: &FynnArray) -> Vec<Vec<f64>> {
        val.matrix
            .iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| std::f64::consts::E.powf(*y))
                    .collect()
            })
            .collect()
    }
}
