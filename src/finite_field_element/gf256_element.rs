use std::ops::{Add, Mul, Div, Sub, Neg};

use crate::{finite_field::FiniteField, utils};

use super::FFElement;

#[derive(Debug)]
pub struct GF256Element {
    pub representation: Vec<usize>
}

impl Add for GF256Element{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let gf256_field = create_gf256_field();
        let self_as_ffe = FFElement::new(self.representation, &gf256_field);
        let rhs_as_ffe = FFElement::new(rhs.representation, &gf256_field);
        let result = self_as_ffe + rhs_as_ffe;
        GF256Element::new(result.representation)
    }
}

impl Mul for GF256Element{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let gf256_field = create_gf256_field();
        let self_as_ffe = FFElement::new(self.representation, &gf256_field);
        let rhs_as_ffe = FFElement::new(rhs.representation, &gf256_field);
        let result = self_as_ffe * rhs_as_ffe;
        GF256Element::new(result.representation)
    }
}

impl Div for GF256Element {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let gf256_field = create_gf256_field();
        let self_as_ffe = FFElement::new(self.representation, &gf256_field);
        let rhs_as_ffe = FFElement::new(rhs.representation, &gf256_field);
        let result = self_as_ffe / rhs_as_ffe;
        GF256Element::new(result.representation)
    }
}

impl Sub for GF256Element{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let gf256_field = create_gf256_field();
        let self_as_ffe = FFElement::new(self.representation, &gf256_field);
        let rhs_as_ffe = FFElement::new(rhs.representation, &gf256_field);
        let result = self_as_ffe - rhs_as_ffe;
        GF256Element::new(result.representation)
    }
}

impl Neg for GF256Element {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        let gf256_field = create_gf256_field();
        let self_as_ffe = FFElement::new(self.representation, &gf256_field);
        let result = -self_as_ffe;
        GF256Element::new(result.representation)
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
        let gf256_field = create_gf256_field();
        let repr_copy = self.representation.clone();
        let self_as_ffe = FFElement::new(repr_copy, &gf256_field);
        let inverse = self_as_ffe.inverse();

        GF256Element::new(inverse.representation)
    }

    pub fn new(representation: Vec<usize>) -> GF256Element{
        GF256Element {
            representation
        }
        
    }
    pub fn to_byte(&self) -> u8{
        let mut result : u8 = 0;
        let mut cur_exp :u32 = 1;
        for i in 0..8{
            result += self.representation[i] as u8 * cur_exp as u8;
            cur_exp *= 2;
        }

        result
    }  

    pub fn from_byte(byte: u8) -> GF256Element{
        let mut number = byte;
        let mut representation = utils::create_zero_vec(8);
        for i in 0..8 {
            representation[i] = (number % 2) as usize; 
            number /= 2;
        }

        

        GF256Element::new(representation)
    }  
}


fn create_gf256_field() -> FiniteField{
    FiniteField { characteristics:2, pow: 8, irr_poly: vec![1, 1, 0, 1, 1, 0, 0, 0, 1] }
}
