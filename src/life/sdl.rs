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
    let ctx:sdl2::Sdl = trying!(
        sdl2::init(sdl2::INIT_VIDEO));

    //Set the window dimensions
    let (window_width,window_height):(i32,i32) = (800,600);

    //ask SDL to build a window
    let window = trying!(
        sdl2::video::Window::new(
            &ctx,                               //SDL context
            "rust-sdl2 demo: Video",            //window title
            sdl2::video::WindowPos::PosCentered,//window x position
            sdl2::video::WindowPos::PosCentered,//Window y position
            window_width,                       //window width
            window_height,                      //window height
            sdl2::video::OPENGL)                //render method
        );

    //start the sdl renderer
    let renderer = trying!(
        sdl2::render::Renderer::from_window(
            window,                                 //the window above 
            sdl2::render::RenderDriverIndex::Auto,  //black magic!!
            sdl2::render::ACCELERATED)              //use gl not sofware
        );

    //SDL2_TTF Init
    match sdl2_ttf::init() {
        true => (),
        false=> panic!(format!("failed to Init Font")),
    }

    //TODO: fix hard path loading
    let font_path: &Path = Path::new("/usr/share/fonts/droid/DroidSans.ttf");
    let font = trying!(sdl2_ttf::Font::from_file(font_path,20));

    //set the areas of the screen we are going to draw on
    let vp_board = Rect::new(0,0,window_width,(window_height-menu_height));
    let vp_menu  = Rect::new(0,(window_height-menu_height),window_width,menu_height);

    //create surfaces 
    let surf_board = trying!(
        Surface::new(
            sdl2::surface::RLEACCEL,    //ACCELERATED surface
            width as i32,               //width
            height as i32,              //height
            24,0,0,0,0)                 //see wiki.libsdl.org/SDL_CreateRGBSurface
        );

    let surf_menu  = trying!(
        Surface::new(
            sdl2::surface::RLEACCEL,
            vp_menu.w ,
            vp_menu.h ,
            24,0,0,0,0)
        );

    //return all the needed parts to the main loop
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

//update the surface for the board 
pub fn update_board(game: &GameState, dispcontext: &mut GameContext) {
    
    //clear the board 
    let _ = dispcontext.surf_board.fill_rect(None, RGB(255,255,255));
    
    //render the board surface
    dispcontext.surf_board.with_lock(|buffer: &mut[u8]| {
        for x in game.alpha.borrow().deref().board.iter() {
            let offset = x*3;
            buffer[offset + 0] = 0 as u8;
            buffer[offset + 1] = 0 as u8;
            buffer[offset + 2] = 0 as u8;
        }
    });

}

//returns a surface with the cell highlighted for the cursor
pub fn board_cursor<'a>(game: &GameState,dispcontext: &mut GameContext) -> Surface<'a> {
    
    let (_,x,y) = sdl2::mouse::get_mouse_state();

    let cursor = Point::new(x,y);
    let mut temp_surface = 
        Surface::new(
            sdl2::surface::RLEACCEL,
            dispcontext.surf_board.get_width(),
            dispcontext.surf_board.get_height(),
            24,0,0,0,0).unwrap();

    if is_enclosed(dispcontext.vp_board,cursor) {
    temp_surface.blit(&dispcontext.surf_board,None,None);
        let offset = game.mouse_to_board(x,y,dispcontext.vp_board)*3;
        temp_surface.with_lock(|buffer: &mut[u8]| {
            buffer[offset + 0] = 0 as u8;
            buffer[offset + 1] = 255 as u8;
            buffer[offset + 2] = 0 as u8;
        })
    }

    temp_surface
}

//update the surface for the menu 
pub fn update_menu(game: &GameState,dispcontext: &mut GameContext) {
    //clear the menu background
    let _ = dispcontext.surf_menu.fill_rect(None,RGB(20,20,20));
    
    //Render the menu
    let speed = format!("Game Speed: {}", game.game_speed);
    
    //Render the menu text
    let text = trying!(dispcontext.font.render_str_blended(speed.as_str(),RGBA(255,255,255,255)));
    
    //Blit the surfaces
    dispcontext.surf_menu.blit(&text,None,None);
}

//actually rerender the screen
pub fn render_sdl(game: &GameState,dispcontext: &mut GameContext) {
   
    //game paused? render the cursor
    let temp_surface:Option<Surface> = match game.pause {
        true => Some(board_cursor(game,dispcontext)),
        false => None,
    };
       
    //do we need to update the menu?
    if game.update_menu {
        update_menu(game,dispcontext);
    }

    //do we need to update the board?
    if game.update_board {
        update_board(game,dispcontext);
    }
    
    //render the board to a texture
    let tex_board = match temp_surface{
        None => dispcontext.renderer.create_texture_from_surface(&dispcontext.surf_board).unwrap(),
        Some(s) => {dispcontext.renderer.create_texture_from_surface(&s).unwrap()}
    };
        
    //render the menu to a texture
    let tex_menu = dispcontext.renderer.create_texture_from_surface(&dispcontext.surf_menu).unwrap();

    //get the drawer from the render
    let mut drawer = dispcontext.renderer.drawer();

    //copy the textures onto the render/window
    drawer.copy(&tex_board,None,Some(dispcontext.vp_board));
    drawer.copy(&tex_menu,None,Some(dispcontext.vp_menu));

    //push the changes to the screen
    drawer.present();
}                        

