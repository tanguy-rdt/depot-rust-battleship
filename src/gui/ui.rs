use crate::game;
pub trait Ui {
    fn new() -> Self;
    fn update_ui(user1: &game::User, user2: &game::User, game_state: game::GameState);
    fn ask_ship_position(ship: &str, nb_case: u8) -> ([i32; 2], [i32; 2]);
    fn ask_target() -> (i32, i32);
}