use std::ops::{Deref};
use std::path::Path;

use life::game::GameState;

use sdl2;
use sdl2::rect::{Point,Rect};
use sdl2::surface::Surface;
use sdl2::pixels::Color::{RGB, RGBA};

use sdl2_ttf;
// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

pub fn is_enclosed(rect:Rect,point:Point) -> bool {
    if rect.x <= point.x && point.x <= (rect.x + rect.w) {
        if rect.y <= point.y && point.y <= (rect.y + rect.h) {
            return true
        }
    }
    return false
}

pub struct GameContext<'a> {
    renderer:sdl2::render::Renderer<'a>,
    surf_board:Surface<'a>,
    surf_menu:Surface<'a>,
    pub font:sdl2_ttf::Font,
    pub vp_board:Rect,
    pub vp_menu:Rect,
}

pub fn init_sdl<'a>(width:usize,height:usize) -> (sdl2::Sdl,GameContext<'a>) {
    let menu_height = 100;
    
    //SDL2 Init
    let ctx:sdl2::Sdl = match sdl2::init(sdl2::INIT_VIDEO) {
        Ok(ctx) => ctx,
        Err(err) => panic!("Failed to start SDL2: {}",err),
    };

    //Create a window
    let (window_width,window_height):(i32,i32) = (800,600);

    let window = match sdl2::video::Window::new(&ctx,"rust-sdl2 demo: Video", sdl2::video::WindowPos::PosCentered, sdl2::video::WindowPos::PosCentered, window_width, window_height, sdl2::video::OPENGL) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    //start the sdl renderer
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };

    //load the font
    match sdl2_ttf::init() {
        true => (),
        false=> panic!(format!("failed to Init Font")),
    }

    let font_path: &Path = Path::new("/usr/share/fonts/droid/DroidSans.ttf");
    let font = trying!(sdl2_ttf::Font::from_file(font_path,20));

    let vp_board = Rect::new(0,0,window_width,(window_height-menu_height));
    let vp_menu  = Rect::new(0,(window_height-menu_height),window_width,menu_height);

    let surf_board = Surface::new(sdl2::surface::RLEACCEL,width as i32,height as i32,24,0,0,0,0).unwrap();
    let surf_menu  = Surface::new(sdl2::surface::RLEACCEL,vp_menu.w ,vp_menu.h ,24,0,0,0,0).unwrap();


    (ctx,
     GameContext{
         renderer:renderer,
         surf_board:surf_board,
         surf_menu:surf_menu,
         font:font,
         vp_board:vp_board,
         vp_menu:vp_menu
     }
    )
}

pub fn render_sdl(game: &GameState,dispcontext: &mut GameContext) {

    let (_,x,y) = sdl2::mouse::get_mouse_state();

    let cursor = Point::new(x,y);

    let _ = dispcontext.surf_board.fill_rect(None, RGB(255,255,255));

    let _ = dispcontext.surf_menu.fill_rect(None,RGB(20,20,20));

    //render the board surface
    if game.pause {
        if is_enclosed(dispcontext.vp_board,cursor) {
            let offset = game.mouse_to_board(x,y,dispcontext.vp_board)*3;
            dispcontext.surf_board.with_lock(|buffer: &mut[u8]| {
                buffer[offset + 0] = 0 as u8;
                buffer[offset + 1] = 255 as u8;
                buffer[offset + 2] = 0 as u8;
            })
        }
    }
    
    dispcontext.surf_board.with_lock(|buffer: &mut[u8]| {
        for x in game.alpha.borrow().deref().board.iter() {
            let offset = x*3;
            buffer[offset + 0] = 0 as u8;
            buffer[offset + 1] = 0 as u8;
            buffer[offset + 2] = 0 as u8;
        }
    });
    
    let tex_board = dispcontext.renderer.create_texture_from_surface(&dispcontext.surf_board).unwrap();

    //Render the menu
    let speed = format!("Game Speed: {}", game.game_speed);
    
    let text = trying!(dispcontext.font.render_str_blended(speed.as_str(),RGBA(255,255,255,255)));
    
    dispcontext.surf_menu.blit(&text,None,None);

    let tex_menu = dispcontext.renderer.create_texture_from_surface(&dispcontext.surf_menu).unwrap();

    let mut drawer = dispcontext.renderer.drawer();

    let _ = drawer.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
    drawer.clear();
    drawer.copy(&tex_board,None,Some(dispcontext.vp_board));
    drawer.copy(&tex_menu,None,Some(dispcontext.vp_menu));
    drawer.present();
}                        

