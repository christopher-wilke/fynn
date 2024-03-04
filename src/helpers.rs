pub fn dot_product<T: Into<f64> + Copy>(inputs: &[T], weights: &[T], bias: T) -> f64 {
    let mut result = 0.;

    for (&val1, &val2) in inputs.iter().zip(weights.iter()) {
        result += val1.into() * val2.into();
    }

    result + bias.into()
}
