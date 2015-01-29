extern crate sdl2;
use sdl2::rect::Point;
use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;
use std::collections::{BTreeSet,BitvSet};
use std::iter::{range_inclusive};
use std::io::Timer;
use std::time::Duration;
use std::num::ToPrimitive;
use std::mem::swap;

static WIDTH: isize = 400;
static HEIGHT: isize = 300;

#[derive(Copy)]
struct Cell {
    v:usize,
}

impl Cell {
    pub fn to_uint(&self) -> Option<usize> {
        self.v.to_uint()
    }

    pub fn to_cord(&self) -> Cord {
        let i = match self.v.to_int() {
            Some(i) => i,
            None => panic!("to_Cord"),
        };
        Cord{r:(i/WIDTH)%HEIGHT,c:(i%WIDTH)%WIDTH}
    }   
}


#[derive(Copy)]
struct Cord {
    r:isize,
    c:isize,
}

impl Cord {
    pub fn to_cell(&self) -> Cell {
        if self.r > HEIGHT || self.c > WIDTH  || self.r < -1 || self.c < -1 {
            panic!("Out of range ({},{})",self.r,self.c);
        }
        match (((self.r+HEIGHT)%HEIGHT) * WIDTH + ((self.c+WIDTH)%WIDTH)).to_uint() {
            Some(x) => Cell{v:x},
            None => panic!("to_cell Failed ({},{})",self.r,self.c),
        }
    }
}

fn main() {

    let total = match (WIDTH * HEIGHT).to_uint() {
        Some(x) => x,
        None => panic!("Unable to allocate table of size {}",(WIDTH*HEIGHT)),
    };
    //allocate two boards
    let mut a = BitvSet::with_capacity(total);
    let mut b  = BitvSet::with_capacity(total);
    let alpha = &mut a;
    let beta = &mut b;

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


        println!("initial state!");

    //create timer
    let mut timer = Timer::new().unwrap();
    let periodic = timer.periodic(Duration::milliseconds(10));

    println!("WIDTH={}\nHEIGHT={}\n",WIDTH,HEIGHT);

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

    //SDL2 Init
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::WindowPos::PosCentered, sdl2::video::WindowPos::PosCentered, 800, 600, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };

    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(128, 128, 128));
    let _ = renderer.set_scale(2.0,2.0);
    let _ = renderer.clear();
    let _ = renderer.present();

    //main loop

    loop {
        //clear the new board
        evolve_board(beta,alpha);
        print_board(beta,&renderer);
        match poll_event() {
            Quit(_) => break,
            KeyDown(_, _, key, _, _, _) => {
                if key == KeyCode::Escape {
                    break;
                }
                if key == KeyCode::G {
                    build_glider(beta);
                }
            }
            _ => {},
        }
        swap(alpha,beta);
        //periodic.recv();
    }
    sdl2::quit();

}

fn build_glider(input : &mut BitvSet) {
    //build a glider
    set_cell(Cord{r:2,c:0},input);
    set_cell(Cord{r:2,c:1},input);
    set_cell(Cord{r:2,c:2},input);
    set_cell(Cord{r:1,c:2},input);
    set_cell(Cord{r:0,c:1},input);
}

fn set_cell(a:Cord ,input: &mut BitvSet) {
    let cell = a.to_cell();
    (*input).insert(cell.v);
}

fn get_cell(a:Cord,input: &BitvSet) -> bool{
    let cell = a.to_cell();
    (*input).contains(&cell.v)
}

fn evolve_cell(a:Cord,new: &mut BitvSet,old:&BitvSet) {
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


fn evolve_board(new: &mut BitvSet, old: &BitvSet) {
    new.clear();
    let mut cells:BTreeSet<isize> = BTreeSet::new();
    for x in old.iter() {
        let c:Cord = Cell{v:x}.to_cord();
        for r in range_inclusive(c.r-1,c.r+1) {
            for c in range_inclusive(c.c-1,c.c+1) {
                cells.insert(Cord{r:r,c:c}.to_cell().v.to_int().unwrap());
            }
        }
    }

    //println!("{}",cells);
    for x in cells.iter() {
        //println!("itter:{}",x);
        let c = Cell{v:x.to_uint().unwrap()}.to_cord();
        //println!("row:{} col:{}",r,c);
        evolve_cell(c,new,old);
    }
}

fn print_board(input: &BitvSet,renderer: &sdl2::render::Renderer) -> () {
    //println!("Printing");
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    renderer.clear();
    let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    for x in input.iter() {
        let c:Cord = Cell{v:x}.to_cord();
        let r = c.r.to_i32().unwrap();
        let c = c.c.to_i32().unwrap();
        let _ = renderer.draw_point(Point::new(c,r));
    }
    renderer.present();
}
