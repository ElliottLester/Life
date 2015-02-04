#![feature(core)]
#![feature(collections)]
#![feature(std_misc)]
#![feature(io)]

extern crate sdl2;
mod life;

use std::old_io::Timer;
use std::time::Duration;
use std::mem::swap;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::num::ToPrimitive;

use life::sdl::{quit_sdl,render_sdl,init_sdl};
use life::board::Board;
use life::thread::{init_threads};

use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;



static WIDTH: usize = 256;
static HEIGHT: usize = 192;

fn main() {

    //allocate two boards
    let a = Board::new(WIDTH,HEIGHT);
    let b = Board::new(WIDTH,HEIGHT);
    let alpha = &mut RefCell::new(a);
    let beta  = &mut RefCell::new(b);

    //create timer for ticks
    let mut timer = Timer::new().unwrap();

    let render = init_sdl(WIDTH,HEIGHT);

    let pool = init_threads(4,alpha.borrow().deref());

    let mut game_speed:usize = 1;

    //main loop
    loop {
        if game_speed > 0 {
            pool.dispatch_threads(alpha);

            pool.compose_threads(beta);

            swap(alpha,beta);
        }

        render_sdl(alpha.borrow().deref(),&render,WIDTH,HEIGHT);

        match poll_event() {
            Quit{..} => break,
            KeyDown{keycode:key, ..} => match key {
                KeyCode::Escape =>
                    break,
                KeyCode::G =>
                    alpha.borrow_mut().deref_mut().build_glider(),
                KeyCode::B =>
                    alpha.borrow_mut().deref_mut().build_blinker(),
                KeyCode::T =>
                    alpha.borrow_mut().deref_mut().build_toad(),
                KeyCode::Comma =>
                    if game_speed <= 5 {game_speed += 1},
                KeyCode::Period =>
                    if game_speed > 1 {game_speed -= 1},
                KeyCode::Space => {
                    match game_speed {
                        0 => game_speed = 1,
                        _ => game_speed = 0,
                    }},

                _ => (),
            },
            _ => {},
        }
        timer.sleep(Duration::milliseconds((16*game_speed).to_i64().unwrap()));
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
