use crate::FynnArray;

pub struct LayerDense {
    weights: FynnArray,
    biases: FynnArray
}

impl LayerDense {
    pub fn new() -> Self {
        let weights = 0.01*FynnArray::randn(2,4);
        let biases = FynnArray::zeros(1, 4);
        
        Self { weights, biases }
    }

    pub fn fwd(&self) {
        println!("forwarding");
    }
}
