use num::ToPrimitive;

#[derive(Copy,Clone)]
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
            r:((i/width)%height).to_isize().unwrap(),
            c:((i%width)%width).to_isize().unwrap()
        }
    }

    pub fn to_usize(&self,width:usize,height:usize) -> usize {
        let iheight = height.to_isize().unwrap();
        let iwidth = width.to_isize().unwrap();

        if self.r > iheight || self.c > iwidth  || self.r < -1 || self.c < -1 {
            panic!("Out of range ({},{})",self.r,self.c);
        }
        match (((self.r+iheight)%iheight) * iwidth + ((self.c+iwidth)%iwidth)).to_usize() {
            Some(x) => x as usize,
            None => panic!("to_cell Failed ({},{})",self.r,self.c),
        }
    }
}
