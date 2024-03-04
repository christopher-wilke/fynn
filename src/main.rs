mod helpers;

struct NeuronsLayer {
    inputs: [f64; 4],
    weights: [[f64; 4]; 3],
    bias: [f64; 3],
    output: Vec<f64>,
}

impl NeuronsLayer {
    pub fn forward(&mut self) {
        for (i, bias) in self.bias.iter().enumerate() {
            let mut n_val = 0.;
            for (j, input) in self.inputs.iter().enumerate() {
                n_val += input * self.weights[i][j];
            }
            n_val += bias;
            self.output.push(n_val);
        }
    }

    pub fn get_output(&self) -> &Vec<f64> {
        &self.output
    }
}

fn main() {
    // let mut layer = NeuronsLayer {
    //     inputs: [1., 2., 3., 2.5],
    //     weights: [
    //         [0.2, 0.8, -0.5, 1.0],
    //         [0.5, -0.91, 0.26, -0.5],
    //         [-0.26, -0.27, 0.17, 0.87],
    //     ],
    //     bias: [2.0, 3.0, 0.5],
    //     output: vec![],
    // };
    // layer.forward();
    // println!("{:?}", layer.get_output());

    let a = [1.0, 2.0, 3.0, 2.5];
    let b = [0.2, 0.8, -0.5, 1.0];

    let dp = helpers::dot_product(&a, &b, 2.);
    println!("{dp:?}");
}
