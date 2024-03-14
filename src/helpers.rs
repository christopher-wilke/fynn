use crate::FynnArray;

pub fn dot(inputs: &FynnArray, weights: &FynnArray) {
    let mut out: Vec<Vec<f64>> = Vec::new();

    // Making sure that Vector is parsable
    assert!(inputs.get_dim().0 == weights.get_dim().1);

    for (i, j) in (&inputs).matrix.iter().enumerate() {
        let mut current_col_weights = 0;
        let mut dotted = vec![];
        for y in 0..inputs.get_dim().1 {
            let mut out = 0.;
            for (k, val) in (&inputs).matrix[i].iter().enumerate() {
                out += val*&weights.matrix[k][current_col_weights];
            }
            dotted.push(out);
            current_col_weights += 1; 
        }
        out.push(dotted);
    }
    println!("{:?}", out);    
}
