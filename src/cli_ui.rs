use std::io::{self, Write};

use crate::game;

pub fn update_ui(user1: &game::User, user2: &game::User, game_state: game::GameState){
    clear_screen();
    show_battleship();

    match game_state {
        game::GameState::Init => {
            if user1.player {
                show_boards(user1.solution, user2.board, 1);
            }
            else {
                show_boards(user1.board, user2.solution, 2);
            }
        }
        game::GameState::InGame => {
            if user1.player {
                show_boards(user1.board, user2.board, 1);
            }
            else {
                show_boards(user1.board, user2.board, 2);
            }
        }
        game::GameState::End => {
            show_boards(user1.solution, user2.solution, 1);
            if user1.winner {
                println!("The winner is User 1 !");
            }
            else {
                println!("The winner is User 2 !");
            }
        }
    }
}


fn check_ship_position((x, y): ([i32; 2], [i32; 2]), ship: &str) -> bool {
    let x = x[1]-x[0]+1;
    let y = y[1]-y[0]+1;
    let res = match ship {
        "Aircraft carrier" => if (x == 5) || (y == 5) { true } else { false },
        "Cruiser" => if (x == 4) || (y == 4) { true } else { false },
        "Destroyer" => if (x == 3) || (y == 3) { true } else { false },
        "Submarine" => if (x == 2) || (y == 2) { true } else { false },
        _ => false,
    };

    res
}


pub fn show_battleship(){
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

pub fn clear_screen() {
    _ = io::stdout().write_all(b"\x1B[2J\x1B[1;1H");
}

pub fn show_boards(user1_board: [[&str; 10]; 10], user2_board: [[&str; 10]; 10], player: u8){
    if player == 1{
        println!("\t\t           Your board                        Opponent's board");
    }
    else{
        println!("\t\t        Opponent's board                        Your board");
    }
    
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
                let (x1, y1) = get_position(input_split[0]);
                let (x2, y2) = get_position(input_split[1]);
            
                return ([x1, x2], [y1, y2]);
            }
        }
        else {
            valid_input = [false; 2];
            println!("Error --> The input is not valid (help: 'a0:a4' or 'A0:A4')");
        }
    }
}

pub fn get_position(pos: &str) -> (i32, i32){
    let mut chars = pos.chars();
    let letter = chars.next().unwrap();

    let mut x = 0;
    if letter.is_ascii_lowercase() {
        x = (letter as i32) - ('a' as i32);
    }
    else if letter.is_ascii_uppercase() {
        x = (letter as i32) - ('A' as i32);
    }
    
    let y = chars.as_str().parse::<i32>().unwrap();

    (x, y)
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

    let (x, y) = get_position(&input);

    (x, y)
}

