use crate::finite_field_element::FFElement;
use crate::utils;

#[derive(Debug)]
pub struct FiniteField{
    pub characteristics: usize,
    pub pow: usize,
    pub irr_poly: Vec<usize>
}

impl FiniteField{
    pub fn new(characteristics: usize, pow :usize, irr_poly : Vec<usize>) -> Self{
        FiniteField { characteristics, 
            pow, 
            irr_poly}
    }

    pub fn create_one(&self) -> FFElement{
        FFElement{
            representation: utils::create_one_vec(self.pow),
            field: self
        }
    }

    pub fn create_zero(&self) -> FFElement{
        FFElement{
            representation: utils::create_zero_vec(self.pow),
            field: self
        }
    }
    
}