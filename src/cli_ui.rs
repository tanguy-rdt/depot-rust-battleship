use std::io::{self, Write};

use crate::game;

pub fn update_ui(user1: &game::User, user2: &game::User, game_state: game::GameState){
    clear_screen();
    show_battleship();

    match game_state {
        game::GameState::INIT => {
            if user1.player {
                show_boards(user1.solution, user2.board, 1);
            }
            else {
                show_boards(user1.board, user2.solution, 2);
            }
        }
        game::GameState::IN_GAME => {
            if user1.player {
                show_boards(user1.board, user2.board, 1);
            }
            else {
                show_boards(user1.board, user2.board, 2);
            }
        }
        game::GameState::END => {
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

pub fn ask_ship_position(ship: &str) -> ([i32; 2], [i32; 2]){
    print!("Enter the position of the {}: ", ship);
    _ = io::stdout().flush();
    let mut pos = String::new();
    _ = io::stdin().read_line(&mut pos);

    let pos_split: Vec<&str> = pos.trim().split(":").collect();

    let (x1, y1) = get_position(pos_split[0]);
    let (x2, y2) = get_position(pos_split[1]);

    ([x1, x2], [y1, y2])
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
    print!("Enter the position of the target: ");
    _ = io::stdout().flush();
    let mut pos = String::new();
    _ = io::stdin().read_line(&mut pos);
    pos = pos.trim().to_string();

    let (x, y) = get_position(&pos);

    (x, y)
}

