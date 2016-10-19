extern crate sdl2;
extern crate sdl2_ttf;
extern crate num;
extern crate bit_set;
mod life;

use std::thread::sleep;
use std::mem::swap;
use std::ops::{Deref, DerefMut};

use life::sdl::{render_sdl,init_sdl,is_enclosed};
use life::thread::{init_threads};
use life::game::GameState;

use sdl2::rect::Point;
use sdl2::event::Event::{Quit, KeyDown, MouseMotion, MouseButtonDown};
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Mouse};

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

static WIDTH: usize = 200;
static HEIGHT: usize = 125;

fn main() {

    let mut game = GameState::new(WIDTH,HEIGHT);

    let (sdl_context,ttf_context,mut dispcontext) = init_sdl(WIDTH,HEIGHT);

    let mut event_pump  = trying!(sdl_context.event_pump());

    let pool = init_threads(4,game.alpha.borrow().deref());

    let mut cycle:usize = game.game_speed;

    //main loop
    'main: loop {

        'event: for event in event_pump.poll_iter() { //needed to empty the event queue
            match event {
                Quit{..} => break,

                MouseMotion {mousestate,x,y,..} =>
                if is_enclosed(dispcontext.vp_board,Point::new(x,y)) {
                    game.update_board = true;
                    if mousestate.left() {
                    game.alpha.borrow_mut().deref_mut().set_cell(game.mouse_to_cord(x,y,dispcontext.vp_board))
                    }

                    else if mousestate.right() {
                    game.alpha.borrow_mut().deref_mut().clear_cell(game.mouse_to_cord(x,y,dispcontext.vp_board))
                    }
                },

                MouseButtonDown{mouse_btn:Mouse::Left,x,y,..} =>
                if is_enclosed(dispcontext.vp_board,Point::new(x,y)) {
                    game.update_board = true;
                     game.alpha.borrow_mut().deref_mut()
                    .set_cell(game.mouse_to_cord(x,y,dispcontext.vp_board))},

                MouseButtonDown{mouse_btn:Mouse::Right,x,y,..} =>
                if is_enclosed(dispcontext.vp_board,Point::new(x,y)) {
                    game.update_board = true;
                     game.alpha.borrow_mut().deref_mut()
                    .clear_cell(game.mouse_to_cord(x,y,dispcontext.vp_board))},

                KeyDown{keycode:Some(key), ..} => match key {
                    Keycode::Escape =>
                        break 'main,
                    Keycode::G => {
                        game.alpha.borrow_mut().deref_mut().build_glider();
                        game.update_board = true},
                    Keycode::B => {
                        game.alpha.borrow_mut().deref_mut().build_blinker();
                        game.update_board = true}
                    Keycode::T => {
                        game.alpha.borrow_mut().deref_mut().build_toad();
                        game.update_board = true},
                    Keycode::Comma =>
                        if game.game_speed > 0 {
                            game.game_speed -= 5;
                            game.update_menu = true
                        },
                    Keycode::Period =>
                        if game.game_speed < 40 {
                            game.game_speed += 5;
                            game.update_menu = true
                        },
                    Keycode::C => {
                        game.alpha.borrow_mut().deref_mut().board.clear();
                        game.update_board = true},
                    Keycode::Space =>
                        game.pause = !game.pause,

                    _ => (),
                },

                //there was an unmatch event but there are other events to process
                _ => ()
            }
        }

        if !game.pause && cycle == game.game_speed {
            pool.dispatch_threads(&game.alpha);

            pool.compose_threads(&game.beta);

            swap(&mut game.alpha,&mut game.beta);

            game.update_board = true;
        }

        render_sdl(&game,&mut dispcontext);

        game.update_board = false;
        game.update_menu = false;

        sleep(std::time::Duration::new(0,10));

        //count 1 second
        cycle = match cycle {
            x if x < game.game_speed => x+1,
            _ => 0,
        }
    }
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
