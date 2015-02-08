use std::cell::{RefCell};

use life::board::Board;

pub struct GameState<'a> {
    pub pause       :bool,
    pub game_speed  :usize,
    pub alpha       :RefCell<Board>,
    pub beta        :RefCell<Board>,
}

impl<'a> GameState<'a> {
    pub fn new(width:usize,height:usize) -> GameState<'a>{
        GameState {
        pause: false,
        game_speed: 15,
        alpha:RefCell::new(Board::new(width,height)),
        beta:RefCell::new(Board::new(width,height)),
        }
    }

}
