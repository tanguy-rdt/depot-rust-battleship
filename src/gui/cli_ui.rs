use std::io::{self, Write};

use crate::game::{self, User, UserStatus};
use crate::gui::ui::Ui;

pub struct CliUi;

impl Ui for CliUi {
    fn new() -> Self {
        CliUi
    }
    
    fn update_ui(&self, user1: &User, user2: &User) {
        clear_screen();
        show_battleship();
        show_username();
        show_boards(user1.get_board(), user2.get_board());
        show_ship_status(user1, user2);
    }

    fn ask_ship_position(&self, ship: &str, nb_case: u8) -> ([i32; 2], [i32; 2]){
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
    
    fn ask_target(&self) -> (i32, i32){
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

    fn show_winner(&self, user1: &User, user2: &User) {
        if *user1.get_status() == UserStatus::Winner {
            println!("The winner is User 1")
        }
        else if *user2.get_status() == UserStatus::Winner {
            println!("The winner is User 2")
        }
    }
}

fn show_ship_status(user1: &User, user2: &User) {
    println!("{:<15}{}", " ", "-".repeat(70));
    println!("{:>55}", "Ship status");
    println!("{:<15}{}", " ", "-".repeat(70));
    println!("\n");

    for ship in game::SHIPS {
        let symbol_user1 = if *user1.get_ship_status(ship) { "✔" } else { "✖" };
        let symbol_user2 = if *user2.get_ship_status(ship) { "✔" } else { "✖" };
        println!("{:<15}-> {:<18} {} {:<15}-> {:<18} {}", " ", ship, symbol_user1, " ", ship, symbol_user2);
    }

    println!("\n\n");
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
    println!("\n");
}
