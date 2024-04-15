use crate::{FynnArray, FynnBehavior};

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

    // let out: Vec<Vec<f64>> = (0..h).map(|i| {
    //     (&inputs).matrix[i as usize]
    //         .iter()
    //         .enumerate()
    //         .map(|(weight_row_idx, _)| {
    //             (&weights).matrix
    //                 .iter()
    //                 .enumerate()
    //                 .fold(0., |acc, (weight_col_idx, weight_col)| {
    //                     acc + weight_col[weight_row_idx] * (&inputs).matrix[i as usize][weight_col_idx]
    //                 })
    //         })
    //         .collect()
    // }).collect();

    out.to_fynn_array()
}
