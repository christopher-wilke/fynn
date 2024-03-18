use crate::{FynnArray, FynnBehavior};

pub fn dot(inputs: &FynnArray, weights: &FynnArray) -> FynnArray {
    assert!(inputs.get_dim().0 == weights.get_dim().1);
    
    let mut out: Vec<Vec<f64>> = Vec::new();

    for (i, _) in (&inputs).matrix.iter().enumerate() {
        let mut current_col_weights = 0;
        let mut dotted = vec![];
        for _ in 0..inputs.get_dim().1 {
            let mut out = 0.;
            for (k, val) in (&inputs).matrix[i].iter().enumerate() {
                out += val*&weights.matrix[k][current_col_weights];
            }
            // Is there a better way to round instead parsing?
            dotted.push(format!("{:.3}", out).parse().unwrap());
            current_col_weights += 1; 
        }
        out.push(dotted);
    }
    out.to_fynn_array() 
}

