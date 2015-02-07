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
use life::cord::Cord;

use sdl2::event::{poll_event, Event};
use sdl2::event::Event::{Quit, KeyDown, MouseMotion, MouseButtonDown};
use sdl2::keycode::KeyCode;
use sdl2::mouse::{RIGHTMOUSESTATE,LEFTMOUSESTATE,Mouse};


static WIDTH: usize = 128;
static HEIGHT: usize = 96;

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

    let mut game_speed:usize = 2;

    //main loop
    'main: loop {
        if game_speed > 1 {
            pool.dispatch_threads(alpha);

            pool.compose_threads(beta);

            swap(alpha,beta);
        } 

        'event: loop { //needed to empty the event queue
            match poll_event() {
                Quit{..} => break,

                MouseMotion {mousestate:LEFTMOUSESTATE,x,y,..} => {
                    alpha.borrow_mut().deref_mut()
                    .set_cell(mouse_to_board(x,y,&render))},

                MouseMotion {mousestate:RIGHTMOUSESTATE,x,y,..} => {
                    alpha.borrow_mut().deref_mut()
                    .clear_cell(mouse_to_board(x,y,&render))},

                MouseButtonDown{mouse_btn:Mouse::Left,x,y,..} => {
                     alpha.borrow_mut().deref_mut()
                    .set_cell(mouse_to_board(x,y,&render))},

                MouseButtonDown{mouse_btn:Mouse::Right,x,y,..} => {
                     alpha.borrow_mut().deref_mut()
                    .clear_cell(mouse_to_board(x,y,&render))},

                KeyDown{keycode:key, ..} => match key {
                    KeyCode::Escape =>
                        break 'main,
                    KeyCode::G =>
                        alpha.borrow_mut().deref_mut().build_glider(),
                    KeyCode::B =>
                        alpha.borrow_mut().deref_mut().build_blinker(),
                    KeyCode::T =>
                        alpha.borrow_mut().deref_mut().build_toad(),
                    KeyCode::Comma =>
                        if game_speed < 25 && game_speed > 1 {game_speed += 1},
                    KeyCode::Period =>
                        if game_speed > 2 {game_speed -= 1},
                    KeyCode::Space => {
                        match game_speed {
                            1 => game_speed = 2,
                            _ => game_speed = 1,
                        }},

                    _ => (),
                },

                //the event loop is empty break
                Event::None => break 'event,

                //there was an unmatch event but there are other events to process
                _ => ()
            }
        }
        render_sdl(alpha.borrow().deref(),&render,game_speed);
        timer.sleep(Duration::milliseconds((10*game_speed.to_i64().unwrap())));
    }
    quit_sdl();
}

fn mouse_to_board(x:i32, y:i32,render:&sdl2::render::Renderer) -> Cord {
    let (x_scale,y_scale) = render.drawer().get_scale();
    let x_size = (x.to_f32().unwrap()/x_scale).to_int().unwrap();
    let y_size = (y.to_f32().unwrap()/y_scale).to_int().unwrap();
    Cord::new(y_size,x_size)
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
