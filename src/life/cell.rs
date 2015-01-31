use life::cord::Cord;

#[derive(Copy)]
struct Cell {
    v:usize,
}

impl Cell {
    pub fn to_uint(&self) -> Option<usize> {
        self.v.to_uint()
    }

    pub fn to_cord(&self,width:usize,height:usize) -> Cord {
        let i = match self.v.to_int() {
            Some(i) => i,
            None => panic!("to_Cord"),
        };
        Cord{r:(i/width)%height,c:(i%width)%width}
    }   
}
