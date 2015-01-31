#[derive(Copy)]
struct Cord {
    r:isize,
    c:isize,
}

impl Cord {
    pub fn to_cell(&self) -> Cell {
        if self.r > HEIGHT || self.c > WIDTH  || self.r < -1 || self.c < -1 {
            panic!("Out of range ({},{})",self.r,self.c);
        }
        match (((self.r+HEIGHT)%HEIGHT) * WIDTH + ((self.c+WIDTH)%WIDTH)).to_uint() {
            Some(x) => Cell{v:x},
            None => panic!("to_cell Failed ({},{})",self.r,self.c),
        }
    }
}
