pub const SHIPS: [&str; 6] = ["Aircraft carrier", "Cruiser", "Destroyer", "Destroyer", "Submarine", "Submarine"];
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
    status: UserStatus
}

impl User {
    pub fn new() -> Self {
        User {
            solution: [[" "; 10]; 10],
            board: [["-"; 10]; 10],
            status: UserStatus::None
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
        "Destroyer" => if (x == 3) || (y == 3) { true } else { false },
        "Submarine" => if (x == 2) || (y == 2) { true } else { false },
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