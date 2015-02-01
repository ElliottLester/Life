use life::cord::Cord;
use std::num::ToPrimitive;

#[derive(Copy)]
pub struct Cell {
    pub v:usize,
}

impl Cell {
    pub fn to_uint(&self) -> Option<usize> {
        self.v.to_uint()
    }

    pub fn to_cord(&self,width:usize,height:usize) -> Cord {
        let i = self.v;
        Cord{r:((i/width)%height).to_int().unwrap(),c:((i%width)%width).to_int().unwrap()}
    }
}
