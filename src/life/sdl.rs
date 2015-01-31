use std::collections::{BTreeSet,BitvSet};

use life::cord::Cord;
use life::cell::Cell;

use sdl2;
use sdl2::rect::Point;
use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;

pub fn init_sdl() -> &mut sdl2::render::RenderDrawer {
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
    
    let mut drawer = renderer.drawer();

    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(128, 128, 128));
    let _ = drawer.set_scale(1.0,1.0);
    let _ = drawer.clear();
    let _ = drawer.present();
    
    drawer
}

pub fn render_sdl(input: &BitvSet,drawer: &mut sdl2::render::RenderDrawer) {
    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    drawer.clear();
    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    for x in input.iter() {
        let c:Cord = Cell{v:x}.to_cord();
        let r = c.r.to_i32().unwrap();
        let c = c.c.to_i32().unwrap();
        let _ = drawer.draw_point(Point::new(c,r));
    }
    drawer.present(); 
}

pub fn quit_sdl() {
    sdl2::quit();
}
