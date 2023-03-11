use std::ops::Add;
use crate::finite_field::{self, FiniteField};

pub struct FiniteFieldElement<'a>{
    pub representation: Vec<usize>,
    pub field: &'a FiniteField,
}

impl FiniteFieldElement<'_>{
    pub fn new(representation: Vec<usize>, field: &FiniteField) -> FiniteFieldElement<'_>{
        FiniteFieldElement{
            representation,
            field
        }
    }
    
}

impl Add for FiniteFieldElement<'_> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result :Vec<usize> = Vec::with_capacity(self.field.pow);
        for i in 0..result.capacity(){
            result[i] = (self.representation[i] + rhs.representation[i]) % self.field.characteristics
        }
        FiniteFieldElement {
            representation : result,
            field: self.field
        }
    }
}

