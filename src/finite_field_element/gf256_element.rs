use std::ops::{Add, Mul, Div, Sub, Neg};

use crate::finite_field::FiniteField;

use super::FFElement;


struct GF256Element{
    pub representation: Vec<usize>
}

impl Add for GF256Element{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let gf256_field = create_GF256_field();
        let self_as_ffe = create_ff_element(self.representation, &gf256_field);
        let rhs_as_ffe = create_ff_element(rhs.representation, &gf256_field);

        let add_result = self_as_ffe + rhs_as_ffe;

        GF256Element{
            representation: add_result.representation
        }
    }
}

impl Mul for GF256Element{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let gf256_field = create_GF256_field();
        let self_as_ffe = create_ff_element(self.representation, &gf256_field);
        let rhs_as_ffe = create_ff_element(rhs.representation, &gf256_field);

        let mul_result = self_as_ffe * rhs_as_ffe;

        GF256Element{
            representation: mul_result.representation
        }
    }
}

impl Div for GF256Element {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let gf256_field = create_GF256_field();
        let self_as_ffe = create_ff_element(self.representation, &gf256_field);
        let rhs_as_ffe = create_ff_element(rhs.representation, &gf256_field);

        let div_result = self_as_ffe / rhs_as_ffe;

        GF256Element{
            representation: div_result.representation
        }
    }
}

impl Sub for GF256Element{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let gf256_field = create_GF256_field();
        let self_as_ffe = create_ff_element(self.representation, &gf256_field);
        let rhs_as_ffe = create_ff_element(rhs.representation, &gf256_field);

        let sub_result = self_as_ffe - rhs_as_ffe;

        GF256Element{
            representation: sub_result.representation
        }
    }
}

impl Neg for GF256Element {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        let gf256_field = create_GF256_field();
        let self_as_ffe = create_ff_element(self.representation, &gf256_field);

        let result = -self_as_ffe;

        GF256Element{
            representation: result.representation
        }
    }
}

fn create_GF256_field() -> FiniteField{
    FiniteField { characteristics:2, pow: 8, irr_poly: vec![1, 1, 0, 1, 1, 0, 0, 0, 0, 1] }
}

fn create_ff_element(representation: Vec<usize>, field: &FiniteField) -> FFElement{
    FFElement { representation, field }
}