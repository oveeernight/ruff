use core::panic;
use std::ops::{Add, Mul, Sub, Div, Neg};
use crate::finite_field::{FiniteField}; 
use crate::{utils};

mod gf256_element;

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
        let result :Vec<usize> = utils::add_vecs(&self.representation, &rhs.representation, self.field.characteristics);
        
        FFElement {
            representation : result,
            field: self.field
        }
    }
}

impl Sub for FFElement<'_> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let rhs_add_rev = utils::add_inverse_vec(&rhs.representation, self.field.characteristics);
        
        let result = utils::add_vecs(&self.representation, &rhs_add_rev, self.field.characteristics);
        FFElement{
            representation: result,
            field: self.field
        }
    }
}

impl Mul for FFElement<'_> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mul_result = utils::mul_vecs(&self.representation, &rhs.representation, self.field.characteristics);
        let result = utils::get_division_remainder(&mul_result, &self.field.irr_poly, self.field.characteristics);

        FFElement{
            representation: result,
            field: self.field
        }
    }
}

impl Div for FFElement<'_> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let inverse = rhs.inverse();
        self * FFElement{
            representation: inverse.representation,
            field: &rhs.field
        }
    }
}

impl Neg for FFElement<'_> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let inv_repr = utils::add_inverse_vec(&self.representation, self.field.characteristics);

        FFElement{
            representation: inv_repr,
            field: &self.field
        }
    }
}


impl <'a> FFElement<'a>{
    pub fn new(representation: Vec<usize>, field: &'a FiniteField) -> FFElement<'a>{
        if representation.len() != field.pow{
            panic!("Incorrect finite field element representation! Components count must be equal to field pow.")
        }
        for i in 0..representation.len(){
            if representation[i] >= field.characteristics{
                panic!("Incorrect finite field element representation! Component value must be less than field characteristic")
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
        let mut result = FFElement { 
            representation: self.representation.clone(),
            field: self.field 
        };


        // этот кошмар происходит потому что result мувается в выражении result * result и компилятор ругается
        // borrow of moved value: `result`
        for i in 0..self.field.characteristics.pow(self.field.pow.try_into().unwrap())-2{
            let square = FFElement { representation: result.representation.clone(), field: self.field } 
            * FFElement { representation: result.representation.clone(), field: self.field };
            result = square;
        }
        result
    }
}

