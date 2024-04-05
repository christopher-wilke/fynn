use crate::FynnArray;

pub struct LayerDense {
    
}

impl LayerDense {
    pub fn new() {
        let multiplier: f64 = 0.01;
        let randon_fynn_array = multiplier*FynnArray::randn(2, 3);
        println!("{:?}", randon_fynn_array);
        
    }
}
