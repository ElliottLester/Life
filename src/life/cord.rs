use std::num::ToPrimitive;

#[derive(Copy)]
pub struct Cord {
    pub r:isize,
    pub c:isize,
}

impl Cord {
    pub fn new(r:isize,c:isize) -> Cord {
        Cord{r:r,c:c}
    }
    pub fn from_uint(i:usize,width:usize,height:usize) -> Cord {
        Cord{
            r:((i/width)%height).to_int().unwrap(),
            c:((i%width)%width).to_int().unwrap()
        }
    }

    pub fn to_uint(&self,width:usize,height:usize) -> usize {
        let iheight = height.to_int().unwrap();
        let iwidth = width.to_int().unwrap();

        if self.r > iheight || self.c > iwidth  || self.r < -1 || self.c < -1 {
            panic!("Out of range ({},{})",self.r,self.c);
        }
        match (((self.r+iheight)%iheight) * iwidth + ((self.c+iwidth)%iwidth)).to_uint() {
            Some(x) => x as usize,
            None => panic!("to_cell Failed ({},{})",self.r,self.c),
        }
    }
}
