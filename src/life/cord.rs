use life::cell::Cell;

#[derive(Copy)]
struct Cord {
    r:isize,
    c:isize,
}

impl Cord {
    pub fn to_cell(&self,width:usize,height:usize) -> Cell {
        if self.r > height || self.c > width  || self.r < -1 || self.c < -1 {
            panic!("Out of range ({},{})",self.r,self.c);
        }
        match (((self.r+height)%height) * width + ((self.c+width)%width)).to_uint() {
            Some(x) => Cell{v:x},
            None => panic!("to_cell Failed ({},{})",self.r,self.c),
        }
    }
}
