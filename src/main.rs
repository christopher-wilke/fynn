struct NeuronsLayer {
    inputs: [f64; 4],
    weights: [[f64; 4]; 3],
    bias: [f64; 3],
}

impl NeuronsLayer {
    pub fn forward(&mut self) {
        let mut value = 0.;
        for neuron in self.weights {
            let n_val = 0.;
            for (idx, val) in neuron.iter().enumerate() {
                println!("{}, {}", idx, val);
            }
            println!("---");
        }
    }
}

fn main() {
    let mut layer = NeuronsLayer {
        inputs: [1., 2., 3., 2.5],
        weights: [
            [0.2, 0.8, -0.5, 1.0],
            [0.5, -0.91, 0.26, -0.5],
            [-0.26, -0.27, 0.17, 0.87],
        ],
        bias: [1.0, 2.0, 0.4],
    };
    layer.forward();
}
