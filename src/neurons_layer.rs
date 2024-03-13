pub struct NeuronsLayer {
    pub inputs: [f64; 4],
    pub weights: [[f64; 4]; 3],
    pub bias: [f64; 3],
    pub output: Vec<f64>
}

impl NeuronsLayer {
    pub fn forward(&mut self) {
        // can we do that better instead two for loops?
        for (i, bias) in self.bias
            .iter()
            .enumerate() {
                let mut tmp = 0.;
                for (j, input) in self.inputs
                    .iter()
                    .enumerate() {
                        tmp += input*self.weights[i][j];
                }
            tmp += bias;
            self.output.push(tmp);
        }
    }
}
