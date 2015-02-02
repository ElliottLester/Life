extern crate sdl2;
mod life;
use std::collections::{BTreeSet,BitvSet};
use std::old_io::Timer;
use std::time::Duration;
use std::num::ToPrimitive;
use std::mem::swap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread::Thread;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use life::sdl::{quit_sdl,render_sdl,init_sdl};
use life::board::{build_glider};
use life::thread::{init_threads};

use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;



static WIDTH: usize = 800;
static HEIGHT: usize = 600;

fn main() {

    let total = (WIDTH * HEIGHT);
    //allocate two boards
    let mut a = BitvSet::with_capacity(total);
    let mut b  = BitvSet::with_capacity(total);
    //let alpha = &mut a;
    //let beta = &mut b;
    let alpha = &mut RefCell::new(a);
    let beta = &mut RefCell::new(b);

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

    //create timer
    let mut timer = Timer::new().unwrap();
    let periodic = timer.periodic(Duration::milliseconds(10));

    let render = init_sdl(WIDTH,HEIGHT);

    let pool = init_threads(6,total,WIDTH,HEIGHT);

    //main loop
    loop {

       pool.dispatch_threads(alpha);

       pool.compose_threads(beta);

       render_sdl(beta.borrow().deref(),&render,WIDTH,HEIGHT);
        match poll_event() {
            Quit{..} => break,
            KeyDown{keycode:key, ..} => {
                if key == KeyCode::Escape {
                    break;
                }
                if key == KeyCode::G{
                    build_glider(beta.borrow_mut().deref_mut(),WIDTH,HEIGHT);
                }
            }
            _ => {},
        }
        swap(alpha,beta);
        periodic.recv();
    }
    quit_sdl();
}
#[test]
fn test_board() {
    for row in range_inclusive(-1,HEIGHT) {
        for col in range_inclusive(-1,WIDTH) {
            let ucell = Cord{r:row,c:col}.to_cell();
            let c = ucell.to_cord();
            if row < 0 {
                assert!(c.r == HEIGHT-1,"r == HEIGHT-1 ({},{}) -> {} -> ({},{})",row,col,ucell.v,c.r,c.c);}
            if row == HEIGHT {
                assert!(c.r == 0,"r == 0 ({},{}) -> {} -> ({},{})",row,col,ucell.v,c.r,c.c);}
            if col < 0 {
                assert!(c.c == WIDTH-1,"col == WIDTH-1({},{}) -> {} -> ({},{})",row,col,ucell.v,c.r,c.c);}
            if col == WIDTH {
                assert!(c.c == 0,"c = 0 ({},{}) -> {} -> ({},{})",row,col,ucell.v,c.r,c.c);}

            if row >=0 && col >= 0 && row < HEIGHT && col < WIDTH {
                assert!(c.r == row && c.c == col,"({},{}) -> {} -> ({},{})",row,col,ucell.v,c.r,c.c);
            }
        }
    }
}
