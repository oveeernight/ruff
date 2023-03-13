use std::vec;

/// Creates
pub fn create_zero_vec(n: usize) -> Vec<usize>{
    let mut result = Vec::with_capacity(n);
    for i in 0..n  {
        result.push(0)
    }
    result
}

pub fn create_one_vec(n: usize) -> Vec<usize>{
    let mut result = create_zero_vec(n);
    result[0] = 1;
    result
}

/// Returns vector degree as polynomial degree.
pub fn get_vec_deg(polynom: &Vec<usize>) -> usize{
    for i in (0..polynom.len()).rev(){
        if polynom[i] != 0{
            return i + 1
        }
    }

    0
}

/// Determines whether the polynom is zero.
pub fn is_zero_vec(polynom: &Vec<usize>) -> bool{
    for i in polynom{
        if *i != 0{
            return false;
        }
    }
    true
}

pub fn add_inverse_vec(v: &Vec<usize>, characteristics: usize) -> Vec<usize>{
    let mut result = create_zero_vec(v.len());
    for i in 0..result.len() {
        result[i] = characteristics - v[i] 
    }

    result
}



pub fn add_vecs(v1: &Vec<usize>, v2: &Vec<usize>, characteristics: usize) -> Vec<usize>{
    let mut result = create_zero_vec(v1.len());
    for i in 0..result.len() {
        result[i] = (v1[i] + v2[i]) % characteristics;
    }

    result

}


/// Returns the result of multiplying polynoms with coefficents from field with given char.
pub fn mul_vecs(v1: &Vec<usize>, v2: &Vec<usize>, characteristics: usize) -> Vec<usize>{
    let mut result = create_zero_vec(v1.len() + v2.len());

    for i in 0..result.len() {
        for j in 0..=i{
            result[i] += (v1[j] * v2[i-j]) % characteristics
        }
    }

    result
}

pub fn get_division_remainder(polynom: &Vec<usize>, modulus: &Vec<usize>, characteristics: usize) -> Vec<usize>{
    let mut cur_deg = get_vec_deg(&polynom);
    let mut cur_poly = polynom.clone();
    let modulus_deg = modulus.len();
    // mst is for modulus senior term
    let mst_rev = inverse_prime_field_element(modulus[modulus_deg-1], characteristics);
    while cur_deg > modulus_deg{
        // if we divide x^5 by x^3 then current multiplier is x^2
        let cur_multiplier_deg = cur_deg - modulus_deg;

        // we want to decrease cur_poly degree, let senior term of cur_poly be A and modulus senior term be B, 
        // C is desired multiplier
        //then A = BC and C = AB^{-1}
        let multiplier = (cur_poly[cur_deg-1] * mst_rev) % characteristics;

        let mut current_divisor = create_zero_vec(cur_multiplier_deg);
        current_divisor[cur_multiplier_deg - 1] = multiplier;

        let subtracted_poly = mul_vecs(&modulus, &current_divisor, characteristics);

        let rev_poly = add_inverse_vec(&subtracted_poly, characteristics);
        cur_poly = add_vecs(&cur_poly, &rev_poly, characteristics);
        cur_deg = get_vec_deg(&cur_poly);
    }
    let mut result = create_zero_vec(modulus.len() - 1);

    for i in 0..result.len(){
        result[i] = cur_poly[i]
    }

    result
}

fn inverse_prime_field_element(a: usize, characteristics: usize) -> usize{
    let mut result = 1;
    // By Lagrange theorem from group theory
    for i in 0..(characteristics-2)  {
        result *= a;
        result %= characteristics
    }
    result
}