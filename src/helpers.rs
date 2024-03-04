pub fn dot_product(arr1: &[u32; 3], arr2: &[u32; 3]) -> u32 {
    let mut sum: u32 = 0;

    for (i, val) in arr1.iter().enumerate() {
        sum += arr1[i] * arr2[i];
    }

    sum
}
