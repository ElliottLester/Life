use std::collections::{BTreeSet,BitvSet};
use std::iter::{range_inclusive};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::num::ToPrimitive;

use life::cord::Cord;
use life::cell::Cell;

struct Board {
    width:usize,
    height:usize,
    total:usize,
    board: BitvSet,
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
    pub fn build_glider(&self) {
        //build a glider
        self.set_cell(Cord{r:2,c:0});
        self.set_cell(Cord{r:2,c:1});
        self.set_cell(Cord{r:2,c:2});
        self.set_cell(Cord{r:1,c:2});
        self.set_cell(Cord{r:0,c:1});
    }

    pub fn set_cell(&self,a:Cord) {
        let cell = a.to_cell(self.width,self.height);
        self.board.borrow_mut().deref_mut().insert(cell.v);
    }

    pub fn get_cell(&self,a:Cord) -> bool{
        let cell = a.to_cell(self.width,self.height);
        self.board.borrow().deref().contains(&cell.v)
    }
}

pub fn build_glider(a:&mut BitvSet,width:usize,height:usize) {
        //build a glider
        set_cell(Cord{r:2,c:0},a,width,height);
        set_cell(Cord{r:2,c:1},a,width,height);
        set_cell(Cord{r:2,c:2},a,width,height);
        set_cell(Cord{r:1,c:2},a,width,height);
        set_cell(Cord{r:0,c:1},a,width,height);
    }

pub fn set_cell(a:Cord,input: &mut BitvSet, width:usize,height:usize) {
    let cell = a.to_cell(width,height);
    input.insert(cell.v);
}

pub fn get_cell(a:Cord,input: &BitvSet,width:usize,height:usize) -> bool {
    let cell = a.to_cell(width,height);
    input.contains(&cell.v)
}

pub fn evolve_cell(a:Cord,new: &mut BitvSet,old:&BitvSet,width:usize,height:usize) {
    let mut n:isize = 0;
    for r in range_inclusive(a.r-1,a.r+1) {
        for c in range_inclusive(a.c-1,a.c+1) {
            if get_cell(Cord{r:r,c:c},old,width,height)  {
                 n += 1;
            }
        }
    }
    let current = get_cell(a,old,width,height);
    if current {
        n -= 1;
    }
    let state = n == 3 || (n == 2 && current );
    if state {
        set_cell(a,new,width,height);
    }
}


pub fn evolve_board(new: &mut BitvSet, old: &BitvSet,start:usize,stop:usize,width:usize,height:usize) {
    new.clear();
    let mut cells:BTreeSet<isize> = BTreeSet::new();
    for x in old.iter().filter(|i| (stop > *i && *i >= start)) {
        let c:Cord = Cell{v:x}.to_cord(width,height);
        for r in range_inclusive(c.r-1,c.r+1) {
            for c in range_inclusive(c.c-1,c.c+1) {
                cells.insert(Cord{r:r,c:c}.to_cell(width,height).v.to_int().unwrap());
            }
        }
    }

    for x in cells.iter() {
        let c = Cell{v:x.to_uint().unwrap()}.to_cord(width,height);
        evolve_cell(c,new,old,width,height);
    }
}
    /*build a Blinker
    set_cell(2,1,&mut alpha);
    set_cell(2,2,&mut alpha);
    set_cell(2,3,&mut alpha);*/

    /* build toad
    set_cell(1,2,&mut alpha);
    set_cell(1,3,&mut alpha);
    set_cell(1,4,&mut alpha);
    set_cell(2,1,&mut alpha);
    set_cell(2,2,&mut alpha);
    set_cell(2,3,&mut alpha);*/


