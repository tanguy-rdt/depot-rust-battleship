mod cli_ui;
mod game;

use crate::game::{UserStatus, User};

fn init_game() -> (User, User){
    let mut user1 = User::new();
    let mut user2 = User::new();

    user1.set_status(UserStatus::Init);
    set_ship_position(&mut user1, &mut user2);
    user1.set_status(UserStatus::None);

    user2.set_status(UserStatus::Init);
    set_ship_position(&mut user1, &mut user2);
    user2.set_status(UserStatus::None);

    (user1, user2)
}

fn set_ship_position(user1: &mut User, user2: &mut User) {
    for (ship, &size) in game::SHIPS.iter().zip(game::SHIP_SIZES.iter()) {
        cli_ui::update_ui(&user1, &user2);
        loop {
            let (x, y) = cli_ui::ask_ship_position(ship, size);
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

fn playing_turn(user1: &mut User, user2: &mut User) {
    if *user1.get_status() == UserStatus::None {
        user1.set_status(UserStatus::Player);
        user2.set_status(UserStatus::None);
    }
    else {
        user1.set_status(UserStatus::None);
        user2.set_status(UserStatus::Player);
    }

    cli_ui::update_ui(user1, user2);

    let (x, y) = cli_ui::ask_target();

    if *user1.get_status() == UserStatus::Player {
        user1.set_board((x, y));
    }
    else {
        user2.set_board((x, y));
    }
}

fn main(){
    let (mut user1, mut user2) = init_game();

    while game::get_game_status(&user1, &user2) != game::GameStatus::End {
        playing_turn(&mut user1, &mut user2);
        game::there_is_winner(&mut user1, &mut user2);
    }

    cli_ui::update_ui(&user1, &user2);
    cli_ui::show_winner(&user1, &user2);
}