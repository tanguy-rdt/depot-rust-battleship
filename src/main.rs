use std::io::{self, Write};

mod cli_ui;
mod game;
struct Game{
    board: [[&'static str; 10]; 10],
}

fn setup() -> (game::User, game::User){
    cli_ui::show_battleship();

    let mut user1 = game::User {
        solution: [[" "; 10]; 10],
        state: [["-"; 10]; 10],
    };

    let mut user2 = game::User {
        solution: [[" "; 10]; 10],
        state: [["-"; 10]; 10],
    };

    let ships = ["Aircraft carrier", "Cruiser", "Destroyer", "Destroyer", "Submarine", "Submarine"];

    for ship in ships{
        _ = cli_ui::clear_screen();
        cli_ui::show_battleship();
        cli_ui::show_boards(user1.solution, user2.state, 0);

        let (mut x, mut y) = ([0, 0], [0, 0]);
        while !game::check_ship_position((x, y), ship) {
            (x, y) = cli_ui::ask_ship_position(ship);
        }
        game::fill_user_solution((x, y), &mut user1)
    }

    for ship in ships{
        _ = cli_ui::clear_screen();
        cli_ui::show_battleship();
        cli_ui::show_boards(user1.state, user2.solution, 1);

        let (mut x, mut y) = ([0, 0], [0, 0]);
        while !game::check_ship_position((x, y), ship) {
            (x, y) = cli_ui::ask_ship_position(ship);
        }
        game::fill_user_solution((x, y), &mut user2)
    }

    (user1, user2)
}

fn play(user1: &mut game::User, user2: &mut game::User, player: u8){
    cli_ui::clear_screen();
    cli_ui::show_battleship();
    cli_ui::show_boards(user1.state, user2.state, player);
    let (x, y) = cli_ui::ask_target();
    if player == 0 {
        game::fill_user_state((x, y), user2);
    } else {
        game::fill_user_state((x, y), user1);
    }
}


fn main(){
    let (mut user1, mut user2) = setup();
    let mut player = 0;
    while !game::end(&user1, &user2){
        play(&mut user1, &mut user2, player);

        if player == 0 {
            player = 1;
        } else {
            player = 0;
        }
    }
}