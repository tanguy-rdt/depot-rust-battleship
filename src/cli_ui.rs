use std::io::{self, Write};

use crate::game;

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

pub fn show_solution(user: &game::User){
    println!("   1  2  3  4  5  6  7  8  9  10");

    for x in 0..10 {
        print!("{} ", (x + 'A' as u8) as char);
        for y in 0..10 {
            print!("[{}]", user.solution[x as usize][y as usize]);
        }

        println!("\n");
    }
}

pub fn show_boards(user1: [[&'static str; 10]; 10], user2: [[&'static str; 10]; 10], player: u8){
    if player == 0{
        println!("\t\t           Your board                        Opponent's board");
    }
    else{
        println!("\t\t        Opponent's board                        Your board");
    }
    
    println!("\t\t   0  1  2  3  4  5  6  7  8  9        0  1  2  3  4  5  6  7  8  9");

    for x in 0..10 {
        print!("\t\t{} ", (x + 'A' as u8) as char);
        for y in 0..10 {
            print!("[{}]", user1[x as usize][y as usize]);
        }

        print!("    ");

        print!("{} ", (x + 'A' as u8) as char);
        for y in 0..10 {
            print!("[{}]", user2[x as usize][y as usize]);
        }

        println!("");
    }
    println!("\n\n");
}

pub fn ask_ship_position(ship: &str) -> ([i32; 2], [i32; 2]){
    print!("Enter the position of the {}: ", ship);
    _ = io::stdout().flush();
    let mut pos = String::new();
    _ = io::stdin().read_line(&mut pos);

    let pos_split: Vec<&str> = pos.trim().split(":").collect();

    let (x1, y1) = game::get_position(pos_split[0]);
    let (x2, y2) = game::get_position(pos_split[1]);

    ([x1, x2], [y1, y2])
}