use crate::finite_field_element::FiniteFieldElement;

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
        let mut one_repr = Vec::with_capacity(self.pow);
        one_repr[0] = 1;
        FiniteFieldElement{
            representation: one_repr,
            field: self
        }
    }
    
}