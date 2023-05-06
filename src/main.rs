use std::io::{self, Write};

use cli_ui::update_ui;

mod cli_ui;
mod game;
struct Game{
    board: [[&'static str; 10]; 10],
}

fn init() -> (game::User, game::User){
    let mut user1 = game::User {
        name: " ".to_string(),
        solution: [[" "; 10]; 10],
        board: [["-"; 10]; 10],
        player: false,
        winner: false
    };

    let mut user2 = game::User {
        name: " ".to_string(),
        solution: [[" "; 10]; 10],
        board: [["-"; 10]; 10],
        player: false,
        winner: false
    };


    let ships = ["Aircraft carrier", "Cruiser", "Destroyer", "Destroyer", "Submarine", "Submarine"]; 

    user1.player = true;
    for ship in ships {
        cli_ui::update_ui(&user1, &user2, game::GameState::INIT);
        loop {
            let (x, y) = cli_ui::ask_ship_position(ship);
            if game::check_ship_position((x, y), ship) {
                game::fill_user_solution((x, y), &mut user1);
                break;
            }
            else {
                println!("Error");
            }
        }
    }
    user1.player = false;

    user2.player = true;
    for ship in ships {
        cli_ui::update_ui(&user1, &user2, game::GameState::INIT);
        loop {
            let (x, y) = cli_ui::ask_ship_position(ship);
            if game::check_ship_position((x, y), ship) {
                game::fill_user_solution((x, y), &mut user2);
                break;
            }
            else {
                println!("Error");
            }
        }
    }
    user2.player = false;

    (user1, user2)
}

fn in_game(user1: &mut game::User, user2: &mut game::User) {
    update_ui(user1, user2, game::GameState::IN_GAME);

    let (x, y) = cli_ui::ask_target();

    if user1.player {
        game::fill_user_board((x, y), user2);
    }
    else {
        game::fill_user_board((x, y), user1);
    }
}


fn main(){


    let mut game_state = game::GameState::INIT;

    let (mut user1, mut user2) = init();


    while game_state != game::GameState::END {
        in_game(&mut user1, &mut user2);
        user1.player = !user1.player;
        user2.player = !user2.player;
        game_state = game::get_game_state(&user1, &user2);
    }

    update_ui(&user1, &user2, game::GameState::END);
}