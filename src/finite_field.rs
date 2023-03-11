use crate::finite_field_element::FiniteFieldElement;
use crate::utils;

pub struct FiniteField{
    pub characteristics: usize,
    pub pow: usize,
    irr_poly: Vec<usize>
}

impl FiniteField{
    pub fn new(characteristics: usize, pow :usize, irr_poly : Vec<usize>) -> Self{
        FiniteField { characteristics, 
            pow, 
            irr_poly}
    }

    pub fn create_one(&self) -> FiniteFieldElement{
        let mut one_repr = utils::create_zero_vec(self.pow);
        one_repr[0] = 1;
        FiniteFieldElement{
            representation: one_repr,
            field: self
        }
    }
    
}