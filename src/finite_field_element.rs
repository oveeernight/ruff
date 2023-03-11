use std::ops::{Add, Mul, Sub, Div};
use crate::finite_field::{self, FiniteField}; 
use crate::{utils};


pub struct FFElement<'a>{
    pub representation: Vec<usize>,
    pub field: &'a FiniteField,
}

impl Add for FFElement<'_> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result :Vec<usize> = utils::create_zero_vec(self.field.pow);
        for i in 0..result.capacity(){
            result[i] = (self.representation[i] + rhs.representation[i]) % self.field.characteristics
        }
        FFElement {
            representation : result,
            field: self.field
        }
    }
}

impl Sub for FFElement<'_> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut rhs_rev_repr = utils::create_zero_vec(self.field.pow);
        for i in 0..self.field.pow{
            rhs_rev_repr[i] = self.field.characteristics - rhs.representation[i]
        }

        self + FFElement{
            representation: rhs_rev_repr,
            field: self.field
        }
    }
}

impl FFElement<'_>{
    pub fn new(representation: Vec<usize>, field: &FiniteField) -> FFElement<'_>{
        FFElement{
            representation,
            field
        }
    }

    pub fn inverse(&self) -> Self {

    }
    
}

impl Mul for FFElement<'_> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = utils::create_zero_vec(2 * self.field.characteristics);
    }
}

