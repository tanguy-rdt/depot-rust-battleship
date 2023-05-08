use std::collections::HashMap;

pub const SHIPS: [&str; 6] = ["Aircraft carrier", "Cruiser", "Destroyer 1", "Destroyer 2", "Submarine 1", "Submarine 2"];
pub const SHIP_SIZES: [u8; 6] = [5, 4, 3, 3, 2, 2];

#[derive(Eq, PartialEq)]
pub enum UserStatus {
    None, 
    Init, 
    Player, 
    Winner, 
    Looser
}

pub struct User {
    solution: [[&'static str; 10]; 10],
    board: [[&'static str; 10]; 10],
    status: UserStatus,
    ships_status: HashMap<&'static str, bool>, 
    ships_position: HashMap<&'static str, Vec<(u8, u8)>>
}

impl User {
    pub fn new() -> Self {
        let mut ships_status_hashmap = HashMap::new();
        let mut ships_position_hashmap = HashMap::new();
        for (ship, ship_nb) in SHIPS.iter().zip(SHIP_SIZES.iter()) {
            ships_status_hashmap.insert(*ship, false);
            ships_position_hashmap.insert(*ship, vec![(0, 0); SHIP_SIZES[*ship_nb as usize].into()]);
        }

        User {
            solution: [[" "; 10]; 10],
            board: [["-"; 10]; 10],
            status: UserStatus::None, 
            ships_status: ships_status_hashmap, 
            ships_position: ships_position_hashmap
        }
    }

    pub fn get_board(&self) -> [[&'static str; 10]; 10]{
        match self.status {
            UserStatus::None => self.board,
            UserStatus::Init => self.solution,
            UserStatus::Player => self.board,
            UserStatus::Winner => self.solution,
            UserStatus::Looser => self.solution, 
        }
    }

    pub fn set_solution(&mut self, (x, y): ([i32; 2], [i32; 2])) {
        if x[0] == x[1] {
            for i in y[0]..y[1]+1{
                self.solution[x[0] as usize][i as usize] = "x";
            }
        }
        else {
            for i in x[0]..x[1]+1{
                self.solution[i as usize][y[0] as usize] = "x";
            }
        }
    }

    pub fn set_ship_position(&mut self, (x, y): ([i32; 2], [i32; 2]), ship: &'static str) {
        let mut position: Vec<(u8, u8)> = vec![(0, 0); SHIP_SIZES[SHIPS.iter().position(|&s| s == ship).unwrap()].into()];

        if x[0] == x[1] {
            let x = x[0];
            for y in y[0]..y[1]+1{
                self.solution[x as usize][y as usize] = "x";
                position[y as usize] = (x as u8, y as u8);
            }
        }
        else {
            let y = y[0];
            for x in x[0]..x[1]+1{
                self.solution[x as usize][y as usize] = "x";
                position[x as usize] = (x as u8, y as u8);
            }
        }

        self.ships_position.insert(ship, position);
    }

    pub fn set_board(&mut self, (x, y): (i32, i32)) {
        if self.solution[x as usize][y as usize] == "x" {
            self.board[x as usize][y as usize] = "x";
        }
        else {
            self.board[x as usize][y as usize] = " ";
        }
    }

    pub fn get_status(&self) -> &UserStatus {
        &self.status
    }

    pub fn set_status(&mut self, state: UserStatus) {
        self.status = state;
    }

    pub fn get_ship_status(&self, ship: &str) -> &bool {
        self.ships_status.get(ship).unwrap()
    }

    pub fn set_ship_status(&mut self, ship: &'static str, status: bool) {
        self.ships_status.insert(ship, status);
    }
}

#[derive(Eq, PartialEq)]
pub enum GameStatus {
    Init, 
    InGame,
    End,
    Error,
}

pub fn get_game_status(user1: &User, user2: &User) -> GameStatus{
    if (user1.status == UserStatus::Init) || (user2.status == UserStatus::Init) {
        GameStatus::Init
    }
    else if (user1.status == UserStatus::Player) || (user2.status == UserStatus::Player) {
        GameStatus::InGame
    }
    else if (user1.status == UserStatus::Winner) || (user2.status == UserStatus::Winner){
        GameStatus::End
    }
    else {
        GameStatus::Error
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

pub fn check_ship_position((x, y): ([i32; 2], [i32; 2]), ship: &str) -> bool {
    let x = x[1]-x[0]+1;
    let y = y[1]-y[0]+1;
    let res = match ship {
        "Aircraft carrier" => if (x == 5) || (y == 5) { true } else { false },
        "Cruiser" => if (x == 4) || (y == 4) { true } else { false },
        "Destroyer 1" | "Destroyer 2" => if (x == 3) || (y == 3) { true } else { false },
        "Submarine 1" | "Submarine 2" => if (x == 2) || (y == 2) { true } else { false },
        _ => false,
    };

    res
}

pub fn there_is_winner(user1: &mut User, user2: &mut User){
    let mut user1_winner = true;
    let mut user2_winner = true;

    for x in 0..user1.solution.len() {
        for y in 0..user1.solution[x as usize].len() {
            if user1.solution[x as usize][y as usize] == "x" && user1.board[x as usize][y as usize] != "x" {
                user2_winner = false;
            }
            if user2.solution[x as usize][y as usize] == "x" && user2.board[x as usize][y as usize] != "x" {
                user1_winner = false;
            } 
        }
    }

    if user1_winner {
        user1.status = UserStatus::Winner;
        user2.status = UserStatus::Looser;
    }
    else if user2_winner {
        user1.status = UserStatus::Looser;
        user2.status = UserStatus::Winner;  
    }
}