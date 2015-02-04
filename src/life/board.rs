use std::collections::{BTreeSet,BitvSet};
use std::iter::{range_inclusive};
use std::cell::RefCell;
use std::num::ToPrimitive;

use life::cord::Cord;

#[derive(Clone)]
pub struct Board {
    pub width:usize,
    pub height:usize,
    pub total:usize,
    pub board : BitvSet,
}

impl Board {
    pub fn new(width:usize,height:usize) -> Board {
        Board{
            width: width,
            height: height,
            total: width*height,
            board: BitvSet::with_capacity(width*height),
        }
    }
    pub fn build_glider(&mut self) {
        let w_center:isize = (self.width/2).to_int().unwrap();
        let h_center:isize = (self.height/2).to_int().unwrap();
        //build a glider
        self.set_cell(Cord{r:h_center + 2,c:w_center});
        self.set_cell(Cord{r:h_center + 2,c:w_center + 1});
        self.set_cell(Cord{r:h_center + 2,c:w_center + 2});
        self.set_cell(Cord{r:h_center + 1,c:w_center + 2});
        self.set_cell(Cord{r:h_center,c:w_center + 1});
    }

    pub fn build_blinker(&mut self) {
        /*build a Blinker*/
        let w_center:isize = (self.width/2).to_int().unwrap();
        let h_center:isize = (self.height/2).to_int().unwrap();
        self.set_cell(Cord{r:h_center + 1,c:w_center + 1});
        self.set_cell(Cord{r:h_center + 1,c:w_center + 2});
        self.set_cell(Cord{r:h_center + 1,c:w_center + 3});
    }

    pub fn build_toad(&mut self) {
        /* build toad */
        let w_center:isize = (self.width/2).to_int().unwrap();
        let h_center:isize = (self.height/2).to_int().unwrap();
        self.set_cell(Cord{r:h_center + 1,c:w_center + 2});
        self.set_cell(Cord{r:h_center + 1,c:w_center + 3});
        self.set_cell(Cord{r:h_center + 1,c:w_center + 4});
        self.set_cell(Cord{r:h_center + 2,c:w_center + 1});
        self.set_cell(Cord{r:h_center + 2,c:w_center + 2});
        self.set_cell(Cord{r:h_center + 2,c:w_center + 3});
    }

    pub fn set_cell(&mut self,a:Cord) {
        self.board.insert(a.to_uint(self.width,self.height));
    }

    pub fn get_cell(&self,a:Cord) -> bool{
        self.board.contains(&a.to_uint(self.width,self.height))
    }
}

fn evolve_cell(a:Cord,alpha: &mut RefCell<Board> ,beta: &RefCell<Board>) {
    let mut alpha = alpha.borrow_mut();
    let beta = beta.borrow();

    let mut n:isize = 0;
    for r in range_inclusive(a.r-1,a.r+1) {
        for c in range_inclusive(a.c-1,a.c+1) {
            if beta.get_cell(Cord{r:r,c:c})  {
                 n += 1;
            }
        }
    }
    let current = beta.get_cell(a);
    if current {
        n -= 1;
    }
    let state = n == 3 || (n == 2 && current );
    if state {
        alpha.set_cell(a);
    }
}


pub fn evolve_board(alpha: &mut RefCell<Board>, beta: &RefCell<Board>,start:usize,stop:usize) {
    let mut cells:BTreeSet<isize> = BTreeSet::new();
    let old = beta.borrow();
    let width = old.width;
    let height = old.height;
    {
    let mut new = alpha.borrow_mut();
    //need old state for comparison
    //new.board.symmetric_difference_with(&old.board);
    for x in old.board.iter().filter(|i| (stop > *i && *i >= start)) {
        let c:Cord = Cord::from_uint(x,width,height);
        for r in range_inclusive(c.r-1,c.r+1) {
            for c in range_inclusive(c.c-1,c.c+1) {
                cells.insert(Cord::new(r,c).to_uint(width,height).to_int().unwrap());
            }
        }
    }
    new.board.clear();
    }
    for x in cells.iter() {
        let c = Cord::from_uint(x.to_uint().unwrap(),width,height);
        evolve_cell(c,alpha, beta);
    }
}

