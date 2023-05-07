use crate::game::User;

pub trait Ui {
    fn new() -> Self;
    fn update_ui(&self, user1: &User, user2: &User);
    fn ask_ship_position(&self, ship: &str, nb_case: u8) -> ([i32; 2], [i32; 2]);
    fn ask_target(&self) -> (i32, i32);
    fn show_winner(&self, user1: &User, user2: &User);
}