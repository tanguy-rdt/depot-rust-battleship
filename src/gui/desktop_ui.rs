use std::io::{self, Write};

use crate::game::{User};
use crate::gui::ui::Ui;

pub struct DesktopUi;

impl Ui for DesktopUi {
    fn new() -> Self {
        DesktopUi
    }

    fn update_ui(&self, user1: &User, user2: &User){

    }
    
    fn ask_ship_position(&self, ship: &str, nb_case: u8) -> ([i32; 2], [i32; 2]){
        ([0, 0], [0, 0])
    }
    
    fn ask_target(&self) -> (i32, i32){
        
        (0, 0)
    }    

    fn show_winner(&self, user1: &User, user2: &User) {

    }
}





