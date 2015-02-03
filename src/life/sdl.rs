use std::collections::BitvSet;
use std::num::ToPrimitive;

use life::cord::Cord;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use life::board::Board;


use sdl2;
use sdl2::rect::Point;

pub fn init_sdl(width:usize,height:usize) -> sdl2::render::Renderer {
    //SDL2 Init
    sdl2::init(sdl2::INIT_VIDEO);

    let (window_width,window_height):(i32,i32) = (1024,768);

    let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::WindowPos::PosCentered, sdl2::video::WindowPos::PosCentered, window_width, window_height, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };
    {
        let mut drawer = renderer.drawer();

        let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(128, 128, 128));
        let (xscale,yscale):(f32,f32) = (
                window_width.to_f32().unwrap()/width.to_f32().unwrap(),
                window_height.to_f32().unwrap()/height.to_f32().unwrap()
            );
        let _ = drawer.set_scale(xscale,yscale);
        let _ = drawer.clear();
        let _ = drawer.present();
    }
    renderer
}

pub fn render_sdl(input: &Board,renderer: &sdl2::render::Renderer,width:usize,height:usize) {

    let mut drawer = renderer.drawer();

    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    drawer.clear();
    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    for x in input.board.iter() {
        let c:Cord = Cord::from_uint(x,width,height);
        let r = c.r.to_i32().unwrap();
        let c = c.c.to_i32().unwrap();
        let _ = drawer.draw_point(Point::new(c,r));
    }
    drawer.present();
}

pub fn quit_sdl() {
    sdl2::quit();
}
