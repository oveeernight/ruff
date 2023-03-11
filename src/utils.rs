pub fn create_zero_vec(n: usize) -> Vec<usize>{
    let mut result = Vec::with_capacity(n);
    for i in 0..n  {
        result.push(0)
    }
    result
}