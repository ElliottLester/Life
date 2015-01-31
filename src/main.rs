extern crate sdl2;
use sdl2::rect::Point;
use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;
use std::collections::{BTreeSet,BitvSet};
use std::iter::{range_inclusive};
use std::old_io::Timer;
use std::time::Duration;
use std::num::ToPrimitive;
use std::mem::swap;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread::Thread;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

static WIDTH: isize = 800;
static HEIGHT: isize = 600;

fn main() {

    let total = match (WIDTH * HEIGHT).to_uint() {
        Some(x) => x,
        None => panic!("Unable to allocate table of size {}",(WIDTH*HEIGHT)),
    };
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

    println!("WIDTH={}\nHEIGHT={}\n",WIDTH,HEIGHT);
    
    let drawer = init_sdl();

    //main loop
    {
    build_glider(&mut(*(alpha.borrow_mut())));
    }
    loop {
       let pool = init_threads(4,total);

       pool.dispatch_threads(alpha);

       pool.compose_threads(beta);

       render_sdl(beta.borrow().deref(),drawer.deref_mut());
        match poll_event() {
            Quit{..} => break,
            KeyDown{keycode:key, ..} => {
                if key == KeyCode::Escape {
                    break;
                }
                if key == KeyCode::G{
                    build_glider(&mut(*(beta.borrow_mut())));
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
