use ruff::finite_field_element::gf256_element::GF256Element;

#[test]
fn add_operation_is_correct(){
    let a = GF256Element::new(vec![0,1,1,1,0,1,0,0]);
    let b = GF256Element::new(vec![0,0,1,0,0,1,0,1]);
    let sum = a + b;

    assert_eq!(sum, GF256Element::new(vec![0,1,0,1,0,0,0,1]))
}