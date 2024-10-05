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
        let mut result = vec![];

        for input_idx in 0..inputs.matrix.len() {
            let mut row = vec![];
            for weight_idx in 0..weights.matrix[0].len() {
                let mut current_idx: usize = 0;
                let mut current_sum: f64 = 0.;
                for i in 0..inputs.matrix[input_idx].len() {
                    current_sum += inputs.matrix[input_idx][current_idx]*weights.matrix[i][weight_idx];
                    current_idx += 1; 
                }
                row.push(current_sum);
            }
            result.push(row);
        }
        result.to_fynn_array()

// poissble refactoring
//         pub fn dot(inputs: &FynnArray, weights: &FynnArray) -> FynnArray {
//     let mut result = Vec::with_capacity(inputs.matrix.len());

//     for input_row in &inputs.matrix {
//         let mut row = Vec::with_capacity(weights.matrix[0].len());

//         for weight_col_idx in 0..weights.matrix[0].len() {
//             let current_sum: f64 = input_row
//                 .iter()
//                 .zip(weights.matrix.iter())
//                 .map(|(input_val, weight_row)| input_val * weight_row[weight_col_idx])
//                 .sum();

//             row.push(current_sum);
//         }

//         result.push(row);
//     }

//     result.to_fynn_array()
// }

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
