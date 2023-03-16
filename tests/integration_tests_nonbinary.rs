use ruff::{finite_field::{FiniteField}, finite_field_element::FFElement};

#[test]
fn add_operation_without_overflow_is_correct(){
    let irr : Vec<usize> = vec![2,1,1];

    let field =  FiniteField::new(3, 2, irr);
    let element1 = FFElement::new(vec![0,1], &field);
    let element2 = FFElement::new(vec![2,1], &field);
    let result = element1 + element2;

    assert_eq!(result, FFElement::new(vec![2,2], &field));
}

#[test]
fn add_operation_with_overflow_is_correct(){
    let irr : Vec<usize> = vec![2,1,1];

    let field =  FiniteField::new(3, 2, irr);
    let element1 = FFElement::new(vec![2,2], &field);
    let element2 = FFElement::new(vec![2,2], &field);
    let result = element1 + element2;

    assert_eq!(result, FFElement::new(vec![1,1], &field));
}

#[test]
fn sub_operation_without_overflow_is_correct(){
    let irr : Vec<usize> = vec![2,1,1];

    let field =  FiniteField::new(3, 2, irr);
    let element1 = FFElement::new(vec![2,2], &field);
    let element2 = FFElement::new(vec![2,1], &field);
    let result = element1 - element2;

    assert_eq!(result, FFElement::new(vec![0,1], &field));
}

#[test]
fn sub_operation_with_overflow_is_correct(){
    let irr : Vec<usize> = vec![2,1,1];

    let field =  FiniteField::new(3, 2, irr);
    let element1 = FFElement::new(vec![0,2], &field);
    let element2 = FFElement::new(vec![2,1], &field);
    let result = element1 - element2;

    assert_eq!(result, FFElement::new(vec![1,1], &field));
}

#[test]
fn neg_evaluation_is_correct(){
    let irr : Vec<usize> = vec![2,1,1];

    let field =  FiniteField::new(3, 2, irr);
    let element = FFElement::new(vec![2,0], &field);
    let rev = -element;
    assert_eq!(rev, FFElement::new(vec![1,0], &field));
}

#[test]
#[should_panic(expected = "Components count must be equal to field pow" )]
fn ffe_new_with_invalid_repr_len_panics(){
    let irr : Vec<usize> = vec![2,1,1];

    let field =  FiniteField::new(3, 2, irr);
    let _element = FFElement::new(vec![0,1,0], &field);
}

#[test]
#[should_panic(expected = "Component value must be less than field characteristic" )]
fn ffe_new_with_invalid_repr_vals_panics(){
    let irr : Vec<usize> = vec![2,1,1];

    let field =  FiniteField::new(3, 2, irr);
    let _element = FFElement::new(vec![3,5], &field);
}

#[test]
fn mul_operation_with_eq_degs_is_correct(){
    let irr : Vec<usize> = vec![5,1,0,3,1,0,0,0,1];

    let field =  FiniteField::new(7, 8, irr);
    let element1 = FFElement::new(vec![1, 0, 2, 5, 4, 0, 0, 5], &field);
    let element2 = FFElement::new(vec![6, 1, 2, 5, 3, 3, 6, 6], &field);
    let result = element1 * element2;

    assert_eq!(result, FFElement::new(vec![3, 6, 0, 4,  4, 2, 0, 5], &field));
}

#[test]
fn mul_operation_with_noteq_degs_is_correct(){
    let irr : Vec<usize> = vec![5,1,0,3,1,0,0,0,1];

    let field =  FiniteField::new(7, 8, irr);
    let element1 = FFElement::new(vec![1, 0, 2, 5, 4, 0, 1, 0], &field);
    let element2 = FFElement::new(vec![6, 1, 2, 5, 3, 0, 0, 0], &field);
    let result = element1 * element2;

    assert_eq!(result, FFElement::new(vec![6, 4, 1, 6, 0, 3, 0, 1], &field));
}

#[test]
fn inverse_in_5exp4field_is_correct(){
    let irr : Vec<usize> = vec![2, 2, 1, 0, 1];
    let field =  FiniteField::new(5, 4, irr);
    let element = FFElement::new(vec![1, 0, 3, 3], &field);
    // in order to not to move element  after inverse
    let element_copy = FFElement::new(vec![1, 0, 3, 3], &field);
    let inverse = element_copy.inverse();
    let result = element * inverse;

    assert_eq!(result, FFElement::new(field.create_one().representation, &field));
}

#[test]
fn inverse_in_7exp8field_is_correct(){
    let irr : Vec<usize> = vec![5,1,0,3,1,0,0,0,1];
    let field =  FiniteField::new(7, 8, irr);
    let element = FFElement::new(vec![1, 0, 2, 5, 4, 0, 1, 0], &field);
    let element_copy = FFElement::new(vec![1, 0, 2, 5, 4, 0, 1, 0], &field);
    let inverse = element_copy.inverse();
    let result = element * inverse;

    assert_eq!(result, FFElement::new(field.create_one().representation, &field));
}

#[test]
#[should_panic(expected = "Can not inverse zero" )]
fn inverse_to_zero_panics(){
    let irr : Vec<usize> = vec![2, 2, 2, 0, 1];

    let field =  FiniteField::new(5, 4, irr);
    let element = field.create_zero();
    element.inverse();
}

#[test]
fn inverse_to_one_is_one(){
    let irr : Vec<usize> = vec![2,1,1];

    let field =  FiniteField::new(3, 2, irr);
    let element = FFElement::new(vec![1, 0], &field);
    // in order to not to move element  after inverse
    let element_copy = FFElement::new(vec![1, 0], &field);
    let inverse = element_copy.inverse();
    let result = element * inverse;

    assert_eq!(result, FFElement::new(field.create_one().representation, &field));
}

#[test]
fn div_in_7exp8field_is_correct(){
    let irr : Vec<usize> = vec![5,1,0,3,1,0,0,0,1];
    let field =  FiniteField::new(7, 8, irr);
    let element1 = FFElement::new(vec![6, 0, 3, 5, 4, 0, 1, 3], &field);
    let element1_copy = FFElement::new(vec![6, 0, 3, 5, 4, 0, 1, 3], &field);
    let element2 = FFElement::new(vec![1, 6, 2, 0, 1, 0, 1, 0], &field);
    let element2_copy = FFElement::new(vec![1, 6, 2, 0, 1, 0, 1, 0], &field);
    let div_res = element1_copy / element2_copy;
    assert_eq!(div_res * element2, element1);
}