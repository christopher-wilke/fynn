pub fn dot_product<T: Into<f64> + Copy>(
    inputs: &Vec<T>,
    weights: &Vec<Vec<T>>,
    bias: &Vec<T>,
) -> Vec<f64> {
    let mut result = vec![];
    let mut sum = 0.;

    for (i, w) in weights.iter().enumerate() {
        let bias_item: f64 = (*bias.get(i).unwrap()).into();
        let mut out = 0.;
        for (val1, val2) in w.iter().zip(inputs.iter()) {
            let item1 = Into::<f64>::into(*val1);
            let item2 = Into::<f64>::into(*val2);
            out += item1 * item2;
        }
        result.push(out + bias_item);
    }

    result
}
