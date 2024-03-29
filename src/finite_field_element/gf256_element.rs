use std::ops::{Add, Mul, Div, Sub, Neg};

use crate::{finite_field::FiniteField, utils};

use super::FFElement;

#[derive(Debug)]
pub struct GF256Element {
    pub representation: Vec<u8>
}

impl Add for GF256Element{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let gf256_field = FiniteField::new(2, 8, vec![1, 1, 0, 1, 1, 0, 0, 0, 1]);
        let self_as_ffe = FFElement::new(utils::to_usize_vec(&self.representation), &gf256_field);
        let rhs_as_ffe = FFElement::new(utils::to_usize_vec(&rhs.representation), &gf256_field);
        let result = self_as_ffe + rhs_as_ffe;
        GF256Element::new(utils::to_u8_vec(&result.representation))
    }
}

impl Mul for GF256Element{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let gf256_field = FiniteField::new(2, 8, vec![1, 1, 0, 1, 1, 0, 0, 0, 1]);
        let self_as_ffe = FFElement::new(utils::to_usize_vec(&self.representation), &gf256_field);
        let rhs_as_ffe = FFElement::new(utils::to_usize_vec(&rhs.representation), &gf256_field);
        let result = self_as_ffe * rhs_as_ffe;
        GF256Element::new(utils::to_u8_vec(&result.representation))
    }
}

impl Div for GF256Element {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let gf256_field = FiniteField::new(2, 8, vec![1, 1, 0, 1, 1, 0, 0, 0, 1]);
        let self_as_ffe = FFElement::new(utils::to_usize_vec(&self.representation), &gf256_field);
        let rhs_as_ffe = FFElement::new(utils::to_usize_vec(&rhs.representation), &gf256_field);
        let result = self_as_ffe / rhs_as_ffe;
        GF256Element::new(utils::to_u8_vec(&result.representation))
    }
}

impl Sub for GF256Element{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

impl Neg for GF256Element {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        let gf256_field = FiniteField::new(2, 8, vec![1, 1, 0, 1, 1, 0, 0, 0, 1]);
        let self_as_ffe = FFElement::new(utils::to_usize_vec(&self.representation), &gf256_field);
        let result = -self_as_ffe;
        GF256Element::new(utils::to_u8_vec(&result.representation))
    }
}

impl PartialEq for GF256Element{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..8{
            if self.representation[i] != other.representation[i]{
                return false;
            }
        }
        
        true
    }
}


impl GF256Element{
    pub fn inverse(&self) -> GF256Element{
        let gf256_field = FiniteField::new(2, 8, vec![1, 1, 0, 1, 1, 0, 0, 0, 1]);
        let repr_copy = self.representation.clone();
        let self_as_ffe = FFElement::new(utils::to_usize_vec( &repr_copy), &gf256_field);
        let inverse = self_as_ffe.inverse();

        GF256Element::new(utils::to_u8_vec(&inverse.representation))
    }

    pub fn new(representation: Vec<u8>) -> GF256Element{
        GF256Element {
            representation
        }
        
    }
    pub fn to_byte(&self) -> u8{
        let mut result : u8 = 0;
        let mut cur_exp :u32 = 1;
        for i in 0..8{
            result += self.representation[i] * cur_exp as u8;
            cur_exp *= 2;
        }

        result
    }  

    pub fn from_byte(byte: u8) -> GF256Element{
        let mut number = byte;
        let mut representation : Vec<u8> = Vec::with_capacity(8);
        for i in 0..8{
            representation.push(0);
            representation[i] = number % 2; 
            number /= 2;
        }
        GF256Element::new(representation)
    }  
}
