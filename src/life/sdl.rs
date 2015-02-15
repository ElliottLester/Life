use std::num::ToPrimitive;
use std::ops::{Deref};


use life::cord::Cord;
use life::game::GameState;


use sdl2;
use sdl2::rect::{Point,Rect};
use sdl2::surface::Surface;

pub struct DispContext {
    renderer:sdl2::render::Renderer,
    surf_board:Surface,
    surf_menu:Surface,
    vp_board:Rect,
    vp_menu:Rect,
}

pub fn init_sdl(width:usize,height:usize) -> DispContext {
    //SDL2 Init
    sdl2::init(sdl2::INIT_VIDEO);

    let (window_width,window_height):(i32,i32) = (800,600);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::WindowPos::PosCentered, sdl2::video::WindowPos::PosCentered, window_width, window_height, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };
    
    let vp_board = Rect::new(0,0,window_width,(window_height-100));
    let vp_menu  = Rect::new(0,(window_height-100),window_width,100);

    let surf_board = Surface::new(sdl2::surface::RLEACCEL,width as i32,height as i32,24,0,0,0,0).unwrap();
    let surf_menu  = Surface::new(sdl2::surface::RLEACCEL,vp_menu.w ,vp_menu.h ,24,0,0,0,0).unwrap();

    {
        let mut drawer = renderer.drawer();

        let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(128, 128, 128));
        let _ = drawer.clear();
        let _ = drawer.present();
    }
    DispContext{renderer:renderer,surf_board:surf_board,surf_menu:surf_menu,vp_board:vp_board,vp_menu:vp_menu}
}

pub fn render_sdl(game: &GameState,dispcontext: &mut DispContext) {


    //let (_,x,y) = sdl2::mouse::get_mouse_state();
    //let cursor = mouse_to_board(x,y,renderer);

    dispcontext.surf_board.with_lock(|buffer: &mut[u8]| {
       for x in 0..(buffer.iter().len()) {
            buffer[x] = 255 as u8;
        }
        for x in game.alpha.borrow().deref().board.iter() {
            let offset = x*3;
            buffer[offset + 0] = 0 as u8;
            buffer[offset + 1] = 0 as u8;
            buffer[offset + 2] = 0 as u8;
        }
    });
    
    let mut tex_board = dispcontext.renderer.create_texture_from_surface(&dispcontext.surf_board).unwrap();

    let mut drawer = dispcontext.renderer.drawer();

    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
    drawer.clear();
    drawer.copy(&tex_board,None,Some(dispcontext.vp_board));
    drawer.present();
    /*
    if game.pause {
        let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
        drawer.draw_point(cursor);
    }

    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    for x in game.alpha.borrow().deref().board.iter() {
        let c:Cord = Cord::from_uint(x,game.alpha.borrow().deref().width,game.alpha.borrow().deref().height);
        let r = c.r.to_i32().unwrap();
        let c = c.c.to_i32().unwrap();
        let _ = drawer.draw_point(Point::new(c,r));
    }
    drawer.present(); 
    */
    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    drawer.clear();
}                        
pub fn quit_sdl() {
    sdl2::quit();
}
fn mouse_to_board(x:i32, y:i32,render:&sdl2::render::Renderer) -> Point {
     let (x_scale,y_scale) = render.drawer().get_scale();
     let x_size = (x.to_f32().unwrap()/x_scale).to_i32().unwrap();
     let y_size = (y.to_f32().unwrap()/y_scale).to_i32().unwrap();
     Point::new(x_size,y_size)
 }
