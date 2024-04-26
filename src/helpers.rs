use crate::{FynnArray, FynnBehavior};

/// This function implements the following specs:
/// * axis=1
/// * keepdims=True
pub fn sum(val: &FynnArray) -> Vec<Vec<f64>> {
    let mut v = vec![];
    val.matrix
        .iter()
        .for_each(|x| {
            v.push(vec![x.iter().sum()]);
        });
    v
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

pub fn norm(x: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    x
        .into_iter()
        .map(|x| {
            let sum = x.iter().sum::<f64>();
            x.into_iter()
                .map(|y| y/sum)
                .collect()
        })
        .collect()
}

pub fn normalize(val: FynnArray) -> Vec<Vec<f64>> {
    // Calc exponentials
    let exp_values = exp(val);
    log::debug!("exp_values: {exp_values:?}");

    let normalized = norm(exp_values);
    log::debug!("probablilities={normalized:?}");
   
    normalized
}

pub fn dot(inputs: &FynnArray, weights: &FynnArray) -> FynnArray {
    assert!(inputs.get_dim().0 == weights.get_dim().1);

    let w = *(&weights.get_dim().0) as u32;
    let h = *(&inputs.get_dim().1) as u32;
    let mut out = vec![];

    for i in 0..h {
        let mut row = vec![];
        let mut weight_row_idx = 0;
        for _ in 0..w {
            let mut weight_col_idx: usize = 0;
            let mut res = 0.;
            for val in (&inputs).matrix[i as usize].iter() {
                res += val * (&weights).matrix[weight_col_idx][weight_row_idx];
                weight_col_idx += 1;
            }
            weight_row_idx += 1;
            row.push(res);
        }
        out.push(row);
    }

    out.to_fynn_array()
}
