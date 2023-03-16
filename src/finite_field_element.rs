use core::panic;
use std::ops::{Add, Mul, Sub, Div, Neg};
use crate::finite_field::{FiniteField}; 
use crate::{utils};

pub mod gf256_element;

#[derive(Debug)]
pub struct FFElement<'a>{
    pub representation: Vec<usize>,
    pub field: &'a FiniteField,
}

impl Add for FFElement<'_> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.field.characteristics != rhs.field.characteristics || self.field.pow != rhs.field.pow{
            panic!("Can not perform operations with this elements. Ensure elements are from the same field.")
        }
        let representation :Vec<usize> = utils::add_vecs(&self.representation, &rhs.representation, self.field.characteristics);
        
        FFElement::new(representation, self.field)
    }
}

impl Sub for FFElement<'_> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let rhs_add_rev = utils::get_opposite_vec(&rhs.representation, self.field.characteristics);
        let representation = utils::add_vecs(&self.representation, &rhs_add_rev, self.field.characteristics);
        FFElement::new(representation, self.field)
    }
}

impl Mul for FFElement<'_> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mul_result = utils::mul_vecs(&self.representation, &rhs.representation, self.field.characteristics, self.field.pow);
        let representation = utils::get_division_remainder(&mul_result, 
            &self.field.irr_poly,
             self.field.characteristics, 
             self.field.pow);

        FFElement::new(representation, self.field)
    }
}

impl<'a> Div for FFElement<'a> {
    type Output = Self;
    fn div(self, rhs: Self) -> FFElement<'a> {
        if utils::is_zero_vec(&rhs.representation){
            panic!("Can not divide by zero.")
        }
        let inverse = rhs.inverse();
        // копируется потому что если вернуть self * inverse компилятор будет ругаться на то, что возвращается значение,
        // ссылающееся на параметр функции rhs (borrowing в 57 строке), а аргументы после операции муваются и тогда получится,
        //что операция возвращает значение, которое нельзя использовать, как я понимаю
        self * FFElement::new(inverse.representation, rhs.field)
    }
}

impl Neg for FFElement<'_> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let inv_repr = utils::get_opposite_vec(&self.representation, self.field.characteristics);
        FFElement::new(inv_repr, self.field)
    }
}

impl PartialEq for FFElement<'_>{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.representation.len() {
            if self.representation[i] != other.representation[i]{
                return false
            }
        }
        self.field.characteristics == other.field.characteristics &&
        self.field.pow == other.field.pow
    }
}


impl <'a> FFElement<'a>{
    pub fn new(representation: Vec<usize>, field: &'a FiniteField) -> FFElement<'a>{
        if representation.len() != field.pow{
            println!("{}", representation.len());
            panic!("Incorrect finite field element representation! Components count must be equal to field pow.")
        }
        for item in representation.iter(){
            if *item >= field.characteristics{
                panic!("Incorrect finite field element representation! Component value must be less than field characteristic.")
            }
        }
        FFElement{
            representation,
            field
        }
    }
}


impl <'a> FFElement<'a>{
    pub fn inverse(&self) -> FFElement {

        if utils::is_zero_vec(&self.representation){
            panic!("Can not inverse zero!");
        }

        let mut cur_exp = FFElement::new(self.representation.clone(), self.field);
        let mut result = self.field.create_one();

        let mut order = self.field.characteristics.pow(self.field.pow.try_into().unwrap()) - 2;

        while order > 0 {
            if order % 2 != 0 {
                // без clone cur_exp мувается и нельзя потом его дальше использовать
                result = result * FFElement::new(cur_exp.representation.clone(), self.field);
            }

            cur_exp = FFElement::new(cur_exp.representation, self.field).square();

            order /= 2;
        }
        result
    }

    pub fn square(self) -> FFElement<'a>{
        let mul_result = utils::mul_vecs(&self.representation, &self.representation, self.field.characteristics, self.field.pow);
        let repr_result = utils::get_division_remainder(&mul_result, &self.field.irr_poly, self.field.characteristics, self.field.pow);

        FFElement::new(repr_result, self.field)
    }
}


