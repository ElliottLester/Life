use std::ops::{Deref};
use std::path::Path;

use life::game::GameState;

use sdl2;
use sdl2::rect::{Point,Rect};
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color::{RGB, RGBA};

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

pub fn is_enclosed(rect:Rect,point:Point) -> bool {
    if rect.x() <= point.x() && point.x() <= (rect.x() + rect.width() as i32) {
        if rect.y() <= point.y() && point.y() <= (rect.y() + rect.height() as i32) {
            return true
        }
    }
    return false
}

pub struct GameContext<'a> {
    pub sdlContext:sdl2::Sdl,
    pub ttfContext:sdl2::ttf::Sdl2TtfContext,
    renderer:sdl2::render::Renderer<'a>,
    surf_board:Surface<'a>,
    surf_menu:Surface<'a>,
    pub font:sdl2::ttf::Font<'a,'a>,
    pub vp_board:Rect,
    pub vp_menu:Rect,
}

pub fn init_sdl<'a>(width:usize,height:usize) -> (GameContext<'a>) {
    let menu_height:u32 = 100;

    //SDL2 Init
    let sdl_context:sdl2::Sdl = trying!(sdl2::init());

    //SDL2 Video context
    let video_subsystem = sdl_context.video().unwrap();

    //Set the window dimensions
    let (window_width,window_height):(u32,u32) = (800,600);

    //ask SDL to build a window
    let window = trying!(
        video_subsystem.window(
            "rust-sdl2 demo: Video",            //window title
            window_width,                       //window width
            window_height)
            .position_centered()
            .opengl()
            .build()
        );

    //start the sdl renderer
    let renderer = trying!(
        window.renderer().build()
        );

    //SDL2_TTF Init
    let ttf_context = trying!(
        sdl2::ttf::init()
        );

    //TODO: fix hard path loading
    let font_path: &Path = Path::new("/usr/share/fonts/droid/DroidSans.ttf");
    let font = trying!(ttf_context.load_font(font_path,20));

    //set the areas of the screen we are going to draw on
    let vp_board = Rect::new(0,0,window_width,(window_height-menu_height));
    let vp_menu  = Rect::new(0,(window_height-menu_height) as i32,window_width,menu_height);

    //create surfaces
    let surf_board = trying!(
        Surface::new(
            width as u32,               //width
            height as u32,              //height
            PixelFormatEnum::RGB24)     //see wiki.libsdl.org/SDL_CreateRGBSurface
        );

    let surf_menu  = trying!(
        Surface::new(
            vp_menu.width() ,
            vp_menu.height() ,
            PixelFormatEnum::RGB24)
        );

    //return all the needed parts to the main loop
    (GameContext{
         sdlContext:sdl_context,
         ttfContext:ttf_context,
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
    dispcontext.surf_board.with_lock_mut(|buffer: &mut[u8]| {
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

    //let (_,x,y) = sdl2::mouse::get_mouse_state();

    let x = 2;
    let y = 2;

    let cursor = Point::new(x,y);
    let mut temp_surface =
        Surface::new(
            dispcontext.surf_board.width(),
            dispcontext.surf_board.height(),
            PixelFormatEnum::RGB24).unwrap();

    if is_enclosed(dispcontext.vp_board,cursor) {
    trying!(temp_surface.blit(None,&mut dispcontext.surf_board,None));
        let offset = game.mouse_to_board(x,y,dispcontext.vp_board)*3;
        temp_surface.with_lock_mut(|buffer: &mut[u8]| {
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
    let text = trying!(
        dispcontext.font.render(
            speed.as_str())
            .blended(RGBA(255,255,255,255))
        );

    //Blit the surfaces
    trying!(text.blit(None,&mut dispcontext.surf_menu,None));
    //dispcontext.surf_menu.blit(None,&text,None);
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
    let ref mut renderer = dispcontext.renderer;

    //copy the textures onto the render/window
    renderer.copy(&tex_board,None,Some(dispcontext.vp_board));
    renderer.copy(&tex_menu,None,Some(dispcontext.vp_menu));

    //push the changes to the screen
    renderer.present();
}

