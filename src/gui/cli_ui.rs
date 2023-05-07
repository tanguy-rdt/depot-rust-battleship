use std::io::{self, Write};

use crate::game::{self, User, UserStatus};

pub fn update_ui(user1: &User, user2: &User) {
    clear_screen();
    show_battleship();
    show_username();
    show_boards(user1.get_board(), user2.get_board());
}

pub fn show_winner(user1: &User, user2: &User) {
    if *user1.get_status() == UserStatus::Winner {
        println!("The winner is User 1")
    }
    else if *user2.get_status() == UserStatus::Winner {
        println!("The winner is User 2")
    }
}


fn show_battleship(){
    println!("
    ███████████             █████     █████    ████            █████████  █████       ███           
    ░░███░░░░░███           ░░███     ░░███    ░░███           ███░░░░░███░░███       ░░░            
     ░███    ░███  ██████   ███████   ███████   ░███   ██████ ░███    ░░░  ░███████   ████  ████████ 
     ░██████████  ░░░░░███ ░░░███░   ░░░███░    ░███  ███░░███░░█████████  ░███░░███ ░░███ ░░███░░███
     ░███░░░░░███  ███████   ░███      ░███     ░███ ░███████  ░░░░░░░░███ ░███ ░███  ░███  ░███ ░███
     ░███    ░███ ███░░███   ░███ ███  ░███ ███ ░███ ░███░░░   ███    ░███ ░███ ░███  ░███  ░███ ░███
     ███████████ ░░████████  ░░█████   ░░█████  █████░░██████ ░░█████████  ████ █████ █████ ░███████ 
    ░░░░░░░░░░░   ░░░░░░░░    ░░░░░     ░░░░░  ░░░░░  ░░░░░░   ░░░░░░░░░  ░░░░ ░░░░░ ░░░░░  ░███░░░  
                                                                                            ░███     
                                                                                            █████    
                                                                                           ░░░░░     
    ");
}

fn clear_screen() {
    _ = io::stdout().write_all(b"\x1B[2J\x1B[1;1H");
}

fn show_username() {
    println!("\t\t              User 1                             User 2");
}

fn show_boards(user1_board: [[&str; 10]; 10], user2_board: [[&str; 10]; 10]){
    println!("\t\t   0  1  2  3  4  5  6  7  8  9        0  1  2  3  4  5  6  7  8  9");

    for x in 0..10 {
        print!("\t\t{} ", (x + 'A' as u8) as char);
        for y in 0..10 {
            print!("[{}]", user1_board[x as usize][y as usize]);
        }

        print!("    ");

        print!("{} ", (x + 'A' as u8) as char);
        for y in 0..10 {
            print!("[{}]", user2_board[x as usize][y as usize]);
        }

        println!("");
    }
    println!("\n\n");
}

pub fn ask_name() -> (String, String) {
    let mut username: Vec<String> = vec![Default::default(), Default::default()];

    for user in 0..2{
        print!("User {}, enter your name: ", user+1);
        _ = io::stdout().flush();

        let mut input = String::new();
        _ = io::stdin().read_line(&mut input);
        username[user] = input.trim().to_string();
    }

    (username.swap_remove(0), username.swap_remove(0))
}

pub fn ask_ship_position(ship: &str, nb_case: u8) -> ([i32; 2], [i32; 2]){
    let mut valid_input: [bool; 2] = [false; 2];

    loop {
        print!("Enter the position of the {} (number of case {}): ", ship, nb_case);
        _ = io::stdout().flush();

        let mut input = String::new();
        _ = io::stdin().read_line(&mut input);
        input = input.trim().to_string();

        if input.len() == 5 {    
            let input_split: Vec<&str> = input.trim().split(":").collect();

            for i in 0..2 {
                let input_split_split: Vec<char> = input_split[i].trim().chars().collect();
                let letter = input_split_split[0];
                let number = input_split_split[1];

                if ((letter >= 'A' && letter <= 'J') || (letter >= 'a' && letter <= 'j')) && (number >= '0' && number <= '9') {
                    valid_input[i] = true;
                }
                else {
                    valid_input = [false; 2];
                    println!("Error --> The input is not valid (help: 'a0:a4' or 'A0:A4')");
                }
            }

            if valid_input[0] && valid_input[1] {
                let (x1, y1) = game::get_position(input_split[0]);
                let (x2, y2) = game::get_position(input_split[1]);
            
                return ([x1, x2], [y1, y2]);
            }
        }
        else {
            valid_input = [false; 2];
            println!("Error --> The input is not valid (help: 'a0:a4' or 'A0:A4')");
        }
    }
}

pub fn ask_target() -> (i32, i32){
    let mut input = String::new();
    let mut valid_input = false;

    while !valid_input {
        print!("Enter the position of the target: ");
        _ = io::stdout().flush();
        _ = io::stdin().read_line(&mut input);
        input = input.trim().to_string();

        if input.len() == 2 {
            let input_split: Vec<char> = input.trim().chars().collect();
            let letter = input_split[0];
            let number = input_split[1];

            if ((letter >= 'A' && letter <= 'J') || (letter >= 'a' && letter <= 'j')) && (number >= '0' && number <= '9') {
                valid_input = true;
            }
            else {
                println!("Error --> The input is not valid (help: 'a0' or 'A0')");
                input.clear();
            }
        }        
        else {
            println!("Error --> The input is not valid (help: 'a0' or 'A0')");
            input.clear();
        }
    }

    let (x, y) = game::get_position(&input);

    (x, y)
}        