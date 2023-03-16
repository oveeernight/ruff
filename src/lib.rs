pub mod finite_field;

pub mod finite_field_element;

mod utils;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_prime_field_element_is_correct1() {
        let element = 5;
        let char = 7;
        let inverse = utils::inverse_prime_field_element(element, char);
        assert_eq!((inverse * element)% char, 1);
    }

    #[test]
    fn inverse_prime_field_element_is_correct2() {
        let element = 2;
        let char = 3;
        let inverse = utils::inverse_prime_field_element(element, char);
        assert_eq!((inverse * element)% char, 1);
    }

    #[test]
    fn inverse_to_big_prime_field_element_is_correct() {
        let element = 762;
        let char = 65537;
        let inverse = utils::inverse_prime_field_element(element, char);
        assert_eq!((inverse * element)% char, 1);
    }

    #[test]
    fn inverse_to_prime_field_13_element_is_correct() {
        let element = 8;
        let char = 13;
        let inverse = utils::inverse_prime_field_element(element, char);
        assert_eq!((inverse * element)% char, 1);
    }

    #[test]
    fn inverse_to_prime_field_17_element_is_correct() {
        let element = 15;
        let char = 17;
        let inverse = utils::inverse_prime_field_element(element, char);
        assert_eq!((inverse * element)% char, 1);
    }

    #[test]
    fn get_vec_deg_wo_leading_zeros_is_correct() {
        let deg = utils::get_vec_deg(&vec![0, 1, 2, 3]);
        assert_eq!(3, deg);
    }

    #[test]
    fn get_vec_deg_with_leading_zeros_is_correct() {
        let deg = utils::get_vec_deg(&vec![0, 1, 2, 3, 0]);
        assert_eq!(3, deg);
    }

    #[test]
    fn mul_vecs_with_leading_zeros_has_correct_degree() {
        let v1 : Vec<usize> = vec![0, 1, 0];
        let v2 : Vec<usize> = vec![0, 1, 0];
        let mul_res = utils::mul_vecs(&v1, &v2, 2, 3);
        assert_eq!(2, utils::get_vec_deg(&mul_res));
    }

    #[test]
    fn mul_vecs_without_leading_zeros_has_correct_degree() {
        let v1 : Vec<usize> = vec![0, 1, 1];
        let v2 : Vec<usize> = vec![0, 1, 1];
        let mul_res = utils::mul_vecs(&v1, &v2, 2, 3);
        assert_eq!(4, utils::get_vec_deg(&mul_res));
    }

}
