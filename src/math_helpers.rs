use rand::Rng;

use crate::FynnArray;
use crate::FynnBehavior;

pub struct MathHelpers;

impl MathHelpers {

    pub fn mean(a: &Vec<usize>, b: &Vec<usize>) -> f64 {
        assert!(a.len() == b.len());

        let counter: usize = a.iter()
            .zip(b.iter())
            .filter(|&(a_v, b_v)| a_v == b_v)
            .count();
        
        counter as f64 / a.len() as f64
    }

    pub fn argmax(v: FynnArray) -> Vec<usize> {
        let mut classes = vec![];
        for i in v.matrix {
            if let Some((highest_el_pos, _)) = i
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) 
            {
                classes.push(highest_el_pos);
            }            
        }
        classes
     }
    
    pub fn clip(input: &mut FynnArray, min: f64, max: f64) -> Vec<Vec<f64>> {
        input
            .matrix
            .iter_mut()
            .map(|row| {
                row
                    .iter_mut()
                    .map(|v| {
                        v.clamp(min, max)
                    })
                    .collect()
            })
            .collect()
    }

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

        out.to_fynn_array()
    }
     
    // keepdimis=True, axis=1
    pub fn sum(input: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        input
            .iter()
            .map(|row| {
                let sum = row.iter().sum::<f64>();
                vec![sum]   
            })
            .collect()
    }

    // keepdims = True, axis=1
    pub fn max(input: &FynnArray) -> Vec<Vec<f64>> {
        input.matrix
            .iter()
            .map(|row| {
                let max = row.iter().fold(
                    f64::NEG_INFINITY, 
                    |a, &b| a.max(b)
                );
                vec![max]
            })
            .collect()
    }
    
    pub fn exp(val: &FynnArray) -> Vec<Vec<f64>> {
        val.matrix
            .iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| std::f64::consts::E.powf(*y))
                    .collect()
            })
            .collect()
    }

    pub fn rand_biases(w: usize) -> Vec<f64> {
        let mut v = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..w {
            v.push(rng.gen_range(0.000001..1.99999));
        }
        v 
    }
}
