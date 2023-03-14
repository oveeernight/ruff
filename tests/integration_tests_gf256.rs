use ruff::finite_field_element::gf256_element::GF256Element;

#[test]
fn add_operation_is_correct(){
    let a = GF256Element::new(vec![0,1,1,1,0,1,0,0]);
    let b = GF256Element::new(vec![0,0,1,0,0,1,0,1]);
    let sum = a + b;

    assert_eq!(sum, GF256Element::new(vec![0,1,0,1,0,0,0,1]))
}

#[test]
fn mul_operation_is_correct(){
    let a = GF256Element::new(vec![1,0,1,1,0,0,0,0]);
    let b = GF256Element::new(vec![1,0,1,0,0,0,0,0]);
    let mul = a * b;

    assert_eq!(mul, GF256Element::new(vec![1,0,0,1,1,1,0,0]))
}

#[test]
fn sub_operation_is_correct(){
    let a = GF256Element::new(vec![1,0,0,1,0,0,0,1]);
    let b = GF256Element::new(vec![1,0,1,0,0,0,1,0]);
    let sub = a - b;

    assert_eq!(sub, GF256Element::new(vec![0,0,1,1,0,0,1,1]))
}

#[test]
fn inverse_is_correct(){
    let a = GF256Element::new(vec![1,0,0,1,0,0,0,1]);
    let inverse = a.inverse();
    let mul = a * inverse;
    assert_eq!(mul, GF256Element::new(vec![1,0,0,0,0,0,0,0]))
}

#[test]
fn div_operation_is_correct(){
    let a = GF256Element::new(vec![1,0,1,1,1,0,0,0]);
    let a_copy = GF256Element::new(vec![1,0,1,1,1,0,0,0]);
    let b = GF256Element::new(vec![0,0,1,1,0,1,1,0]); 
    let b_copy = GF256Element::new(vec![0,0,1,1,0,1,1,0]);
    let div = a / b;
    let mul = div * b_copy;
    assert_eq!(mul, a_copy);
}

#[test]
fn to_byte_operation_is_correct(){
    let a = GF256Element::new(vec![1,0,1,1,1,0,0,0]);
    let byte = a.to_byte();
    assert_eq!(29, byte);
}

#[test]
fn from_byte_operation_is_correct(){
    let a = GF256Element::from_byte(29);
    assert_eq!(a, GF256Element::new(vec![1,0,1,1,1,0,0,0]));
}