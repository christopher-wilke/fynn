use rand::distributions::Weight;
use rand::Rng;

use crate::fynn_bias::FynnBias;
use crate::FynnArray;
use crate::FynnBehavior;

pub struct MathHelpers;

impl MathHelpers {
    pub fn mean(a: &Vec<u32>, b: &Vec<u32>) -> f64 {
        // assert!(a.len() == b.len());

        let counter: usize = a
            .iter()
            .zip(b.iter())
            .filter(|&(a_v, b_v)| a_v == b_v)
            .count();

        counter as f64 / a.len() as f64
    }

    pub fn argmax(v: FynnArray) -> Vec<u32> {
        let mut classes = vec![];
        for i in v.matrix {
            if let Some((highest_el_pos, _)) = i
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            {
                classes.push(highest_el_pos as u32);
            }
        }
        classes
    }

    pub fn clip(input: &mut FynnArray, min: f64, max: f64) -> Vec<Vec<f64>> {
        input
            .matrix
            .iter_mut()
            .map(|row| row.iter_mut().map(|v| v.clamp(min, max)).collect())
            .collect()
    }

    // math tells to transpose the weight. I cannot see the advantage so I am using the non-transposed vec.
    pub fn dot(inputs: &FynnArray, weights: &FynnArray) -> FynnArray {
        let mut res = vec![];
        for (current_input, w) in inputs.matrix.iter().zip(weights.matrix.iter()) {
            let mut row = vec![];
            for i in 0..w.len() - 1 {
                let mut sum = 0.;
                for (k, v) in current_input.iter().zip(weights.matrix[i].iter()) {
                    sum += k * v;
                }
                // Rounding by 3 digits avoiding String casting using format!()
                row.push((sum*1000.).round() / 1000.);
            }
            res.push(row);
        }
        res.to_fynn_array()
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
        input
            .matrix
            .iter()
            .map(|row| {
                let max = row.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
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

    pub fn rand_biases(w: usize) -> FynnBias {
        let mut val = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..w {
            val.push(rng.gen_range(0.000001..1.99999));
        }
        FynnBias { val }
    }
}
