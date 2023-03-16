pub fn create_zero_vec(n: usize) -> Vec<usize>{
    let mut result = Vec::with_capacity(n);
    for _i in 0..n  {
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
pub fn get_vec_deg(vec: &Vec<usize>) -> usize{
    for i in (0..vec.len()).rev(){
        if vec[i] != 0{
            return i
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

pub fn get_opposite_vec(v: &[usize], characteristics: usize) -> Vec<usize>{
    let mut result = create_zero_vec(v.len());
    for i in 0..result.len() {
        result[i] = (characteristics - v[i]) % characteristics
    }

    result
}



pub fn add_vecs(v1: &[usize], v2: &[usize], characteristics: usize) -> Vec<usize>{
    let mut result = create_zero_vec(v1.len());
    for i in 0..result.len() {
        result[i] = (v1[i] + v2[i]) % characteristics;
    }

    result

}


/// Returns the result of multiplying polynoms with coefficents from field with given char.
    pub fn mul_vecs(v1: &[usize], v2: &[usize], characteristics: usize, pow: usize) -> Vec<usize>{
    let mut result = create_zero_vec(pow * 2);

    for i in 0..v1.len() {
        for j in 0..v2.len(){
            result[i+j] += (v1[i] * v2[j]) % characteristics
        }
    }

    result
}

pub fn get_division_remainder(polynom: &Vec<usize>, modulus: &[usize], characteristics: usize, pow: usize) -> Vec<usize>{
    let mut cur_deg = get_vec_deg(polynom);
    let mut cur_poly = polynom.clone();
    let modulus_deg = modulus.len() - 1;
    // mlt is for modulus leading term
    let mlt_rev = inverse_prime_field_element(modulus[modulus_deg], characteristics);
    while cur_deg >= modulus_deg{
        // if we divide x^5 by x^3 then current multiplier is x^2
        let cur_multiplier_deg = cur_deg - modulus_deg;

        // we want to decrease cur_poly degree, let senior term of cur_poly be A and modulus senior term be B, 
        // C is desired multiplier
        //then A = BC and C = AB^{-1}
        let multiplier = (cur_poly[cur_deg] * mlt_rev) % characteristics;

        let mut current_divisor = create_zero_vec(cur_multiplier_deg + 1);
        current_divisor[cur_multiplier_deg] = multiplier;

        let subtracted_poly = mul_vecs(modulus, &current_divisor, characteristics, pow);
        let rev_poly = get_opposite_vec(&subtracted_poly, characteristics);
        cur_poly = add_vecs(&cur_poly, &rev_poly, characteristics);
        cur_deg = get_vec_deg(&cur_poly);
    }
    let mut result = create_zero_vec(modulus.len() - 1);

    for i in 0..result.len(){
        result[i] = cur_poly[i] % characteristics
    }

    result
}

pub fn inverse_prime_field_element(a: usize, characteristics: usize) -> usize{
    let mut result = 1;
    let mut cur_exp = a;
    // By Lagrange theorem from group theory a^p = a
    // then a^{-1} = a^{p-2}
    let mut i = characteristics - 2;
    while i  > 0{
        if i % 2 != 0{
            result *= cur_exp;
            result %= characteristics
        }
        cur_exp *= cur_exp;
        cur_exp %= characteristics;
        i /= 2;
    }

    result
}

