use std::cell::{RefCell};
use std::num::ToPrimitive;

use life::board::Board;
use life::cord::Cord;

use sdl2::rect::Rect;

pub struct GameState {
    pub pause       :bool,
    pub game_speed  :usize,
    pub width       :usize,
    pub height      :usize,
    pub update_menu :bool,
    pub update_board:bool,
    pub alpha       :RefCell<Board>,
    pub beta        :RefCell<Board>,
}

impl<'a> GameState {
    pub fn new(width:usize,height:usize) -> GameState{
        GameState {
        pause       :false,
        game_speed  :40,
        width       :width,
        height      :height,
        update_menu :true,
        update_board:true,
        alpha       :RefCell::new(Board::new(width,height)),
        beta        :RefCell::new(Board::new(width,height)),
        }
    }
    
    pub fn mouse_to_board(&self,x:i32, y:i32,vp:Rect) -> usize {
        let width = self.width.to_i32().unwrap();
        let height = self.height.to_i32().unwrap();

        let x_scale:f32 = vp.w.to_f32().unwrap() / width.to_f32().unwrap();
        let y_scale:f32 = vp.h.to_f32().unwrap() / height.to_f32().unwrap();

        let x_size = (x.to_f32().unwrap()/x_scale).to_i32().unwrap();
        let y_size = (y.to_f32().unwrap()/y_scale).to_i32().unwrap();

        (y_size*width + x_size).to_usize().unwrap()
    }

    pub fn mouse_to_cord(&self,x:i32, y:i32,vp:Rect) -> Cord {
        let width = self.width.to_i32().unwrap();
        let height = self.height.to_i32().unwrap();

        let x_scale:f32 = vp.w.to_f32().unwrap() / width.to_f32().unwrap();
        let y_scale:f32 = vp.h.to_f32().unwrap() / height.to_f32().unwrap();

        let x_size = (x.to_f32().unwrap()/x_scale).to_isize().unwrap();
        let y_size = (y.to_f32().unwrap()/y_scale).to_isize().unwrap();

        Cord::new(y_size,x_size)

    }
}
