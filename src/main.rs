extern crate sdl2;
extern crate num;
extern crate bit_set;
extern crate num_cpus;
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

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

static WIDTH: usize = 400;
static HEIGHT: usize = 250;

fn main() {

    let mut game = GameState::new(WIDTH,HEIGHT);

    let mut dispcontext = init_sdl(WIDTH,HEIGHT);

    let pool = init_threads(num_cpus::get(),game.alpha.borrow().deref());

    let mut cycle:usize = game.game_speed;

    //main loop
    'main: loop {
        
        let eventPump = dispcontext.sdlContext.event_pump();

        'event: for event in eventPump.unwrap().poll_iter() { //needed to empty the event queue
            print!("Event: {:?}\n", event);
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
    for row in num::range_inclusive(-1,HEIGHT)  {
        for col in num::range_inclusive(-1,WIDTH) {
            let ucell = Cord{r:row as isize,c:col as isize}.to_cell();
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
