use core::panic;
use std::ops::{Add, Mul, Sub, Div, Neg};
use std::result;
use crate::finite_field::{FiniteField}; 
use crate::{utils};

mod gf256_element;

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
        let mul_result = utils::mul_vecs(&self.representation, &rhs.representation, self.field.characteristics, self.field.pow);
        let result = utils::get_division_remainder(&mul_result, 
            &self.field.irr_poly,
             self.field.characteristics, 
             self.field.pow);

        FFElement{
            representation: result,
            field: self.field
        }
    }
}

impl Div for FFElement<'_> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if utils::is_zero_vec(&rhs.representation){
            panic!("Can not divide by zero.")
        }
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
        for i in 0..representation.len(){
            if representation[i] >= field.characteristics{
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

        let mut result = FFElement { 
            representation: self.representation.clone(),
            field: self.field 
        };

        let mut temp_result = FFElement { 
            representation: self.representation.clone(),
            field: self.field 
        };


        let mut i = 1;
        let mut temp_i = 1;
        let order = self.field.characteristics.pow(self.field.pow.try_into().unwrap());

        while i != order - 2 {
            if i + temp_i <= order - 2 {
                // этот кошмар происходит потому что result мувается в выражении result * result и компилятор ругается
                // borrow of moved value: `result`
                temp_result = FFElement::new(temp_result.representation.clone(), &self.field) * 
                    FFElement::new(temp_result.representation.clone(), &self.field);
                temp_i *= 2;
            }
            else{
                result = FFElement::new(result.representation.clone(), &self.field) * temp_result;
                i += temp_i;

                temp_i = 1;
                temp_result = FFElement::new(self.representation.clone(), &self.field);

            }
        }
        result
    }
}


