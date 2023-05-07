mod gui;
mod game;

use crate::gui::Gui;
use crate::gui::ui::Ui;
use crate::game::{UserStatus, User};

fn init_game<T: Ui>(ui: &T) -> (User, User){
    let mut user1 = User::new();
    let mut user2 = User::new();

    user1.set_status(UserStatus::Init);
    set_ship_position(ui, &mut user1, &mut user2);
    user1.set_status(UserStatus::None);

    user2.set_status(UserStatus::Init);
    set_ship_position(ui, &mut user1, &mut user2);
    user2.set_status(UserStatus::None);

    (user1, user2)
}

fn set_ship_position<T: Ui>(ui: &T, user1: &mut User, user2: &mut User) {
    for (ship, &size) in game::SHIPS.iter().zip(game::SHIP_SIZES.iter()) {
        ui.update_ui(&user1, &user2);
        loop {
            let (x, y) = ui.ask_ship_position(ship, size);
            if game::check_ship_position((x, y), ship) {
                if *user1.get_status() == UserStatus::Init {
                    user1.set_solution((x, y));
                }
                else {
                    user2.set_solution((x, y));
                }
                
                break;
            }
            else {
                println!("Error");
            }
        }
    }
}

fn playing_turn<T: Ui>(ui: &T, user1: &mut User, user2: &mut User) {
    if *user1.get_status() == UserStatus::None {
        user1.set_status(UserStatus::Player);
        user2.set_status(UserStatus::None);
    }
    else {
        user1.set_status(UserStatus::None);
        user2.set_status(UserStatus::Player);
    }

    ui.update_ui(user1, user2);

    let (x, y) = ui.ask_target();

    if *user1.get_status() == UserStatus::Player {
        user1.set_board((x, y));
    }
    else {
        user2.set_board((x, y));
    }
}

fn main(){
    let ui = Gui::new();
    let (mut user1, mut user2) = init_game(&ui);

    while game::get_game_status(&user1, &user2) != game::GameStatus::End {
        playing_turn(&ui, &mut user1, &mut user2);
        game::there_is_winner(&mut user1, &mut user2);
    }

    ui.update_ui(&user1, &user2);
    ui.show_winner(&user1, &user2);
}