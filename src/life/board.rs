struct board {
    width:usize,
    height:usize,
    total:usize,
    board: mut RefCell<BitvSet>,
}

impl board {
    pub fn new(width:usize,height:usize) -> board {
        board{
            width: width,
            height: height,
            total: width*height,
            board: RefCell::new(BitvSet::with_capacity(width*height));
        }
    }
    pub fn build_glider(&self) {
    //build a glider
    self.set_cell(Cord{r:2,c:0});
    self.set_cell(Cord{r:2,c:1});
    self.set_cell(Cord{r:2,c:2});
    self.set_cell(Cord{r:1,c:2});
    self.set_cell(Cord{r:0,c:1});
}

pub fn set_cell(&self,a:Cord) {
    let cell = a.to_cell();
    self.board.borrow_mut().deref_mut().insert(cell.v);
}

pub fn get_cell(&self,a:Cord) -> bool{
    let cell = a.to_cell();
    self.board.borrow().deref().contains(&cell.v)
}

pub fn evolve_cell(a:Cord,new: &mut BitvSet,old:&BitvSet) {
    let mut n:isize = 0;
    for r in range_inclusive(a.r-1,a.r+1) {
        for c in range_inclusive(a.c-1,a.c+1) {
            if get_cell(Cord{r:r,c:c},old)  {
                 n += 1;
            }
        }
    }
    let current = get_cell(a,old);
    if current {
        n -= 1;
    }
    let state = n == 3 || (n == 2 && current );
    if state {
        set_cell(a,new);
    }
}


pub fn evolve_board(new: &mut BitvSet, old: &BitvSet,start:usize,stop:usize) {
    new.clear();
    let mut cells:BTreeSet<isize> = BTreeSet::new();
    for x in old.iter().filter(|i| (stop > *i && *i >= start)) {
        let c:Cord = Cell{v:x}.to_cord();
        for r in range_inclusive(c.r-1,c.r+1) {
            for c in range_inclusive(c.c-1,c.c+1) {
                cells.insert(Cord{r:r,c:c}.to_cell().v.to_int().unwrap());
            }
        }
    }

    for x in cells.iter() {
        let c = Cell{v:x.to_uint().unwrap()}.to_cord();
        evolve_cell(c,new,old);
    }
}
