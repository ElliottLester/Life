use life::cell::Cell;
use std::num::ToPrimitive;

#[derive(Copy)]
pub struct Cord {
    pub r:isize,
    pub c:isize,
}

impl Cord {
    pub fn to_cell(&self,width:usize,height:usize) -> Cell {
        let iheight = height.to_int().unwrap();
        let iwidth = width.to_int().unwrap();

        if self.r > iheight || self.c > iwidth  || self.r < -1 || self.c < -1 {
            panic!("Out of range ({},{})",self.r,self.c);
        }
        match (((self.r+iheight)%iheight) * iwidth + ((self.c+iwidth)%iwidth)).to_uint() {
            Some(x) => Cell{v:x},
            None => panic!("to_cell Failed ({},{})",self.r,self.c),
        }
    }
}
