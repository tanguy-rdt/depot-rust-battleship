use crate::Game;

pub struct User {
    pub name: String,
    pub solution: [[&'static str; 10]; 10],
    pub board: [[&'static str; 10]; 10],
    pub player: bool,
    pub winner: bool, 
}

#[derive(Eq, PartialEq)]
pub enum GameState {
    Init, 
    InGame,
    End,
}

pub fn fill_user_solution((x, y): ([i32; 2], [i32; 2]), user: &mut User){
    if x[0] == x[1] {
        for i in y[0]..y[1]+1{
            user.solution[x[0] as usize][i as usize] = "x";
        }
    }
    else {
        for i in x[0]..x[1]+1{
            user.solution[i as usize][y[0] as usize] = "x";
        }
    }
}

pub fn fill_user_board((x, y): (i32, i32), user: &mut User){
    if user.solution[x as usize][y as usize] == "x" {
        user.board[x as usize][y as usize] = "x";
    }
    else {
        user.board[x as usize][y as usize] = " ";
    }
}

pub fn get_game_state(user1: &mut User, user2: &mut User) -> GameState{
    let mut user1_winner = true;
    let mut user2_winner = true;

    for x in 0..9 {
        for y in 0..9 {
            if ((user1.solution[x][y] == "x") && (user1.board[x][y] != "x")) {
                user2_winner = false;
            }
            
            if ((user2.solution[x][y] == "x") && (user2.board[x][y] != "x")) {
                user1_winner = false;
            }
        }
    }

    if user1_winner {
        user1.winner = true;
        return GameState::End;
    }
    else if user2_winner {
        user2.winner = true;
        return GameState::End;
    }
    else {
        return GameState::InGame;
    }

}