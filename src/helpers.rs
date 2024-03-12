// pub struct FynnArray {
//     value: Vec<f32>,
// }

// impl FynnArray {
//     pub fn new(arr: &[f32]) -> Self {
//         Self {
//             value: Vec::from(arr),
//         }
//     }
// }

// fn process_number<
//     T: std::ops::Add<Output = T>
//         + std::ops::Mul<Output = T>
//         + std::ops::Sub<Output = T>
//         + std::cmp::PartialOrd
//         + Copy,
// >(
//     number: T,
// ) -> T {
//     number * number
// }

// pub fn array(arr: &[T]) {
//     FynnArray::new(arr)
// }

// pub fn dot<T>(inputs: &Vec<T>, weights: &Vec<Vec<T>>, bias: &Vec<T>) -> Vec<T> {
//     let mut result = vec![];
//     // let mut sum = 0.;

//     // for (i, w) in weights.iter().enumerate() {
//     //     // let b = Into::<f32>::into(bias.get(i).unwrap() as u16);

//     //     println!("{b:?}");
//     //     // let bias_item: f64 = (*bias.get(i).unwrap()).into();
//     //     // let mut out = 0.;
//     //     // for (val1, val2) in w.iter().zip(inputs.iter()) {
//     //     //     // let item1 = Into::<f64>::into(*val1);
//     //     //     // let item2 = Into::<f64>::into(*val2);
//     //     //     out += item1 * item2;
//     //     // }
//     //     // result.push(out + bias_item);
//     // }

//     result
// }

// pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
//     let len = v[0].len();
//     let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

//     (0..len)
//         .map(|_| {
//             iters
//                 .iter_mut()
//                 .map(|n| n.next().unwrap())
//                 .collect::<Vec<T>>()
//         })
//         .collect()
// }
